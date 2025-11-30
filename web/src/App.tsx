import React from 'react';
import './App.css';
import { useTranslation } from 'react-i18next';

import * as gtpLib from 'lib-wasm';
import MatrixView from './MatrixView';
import PieceView from './PieceView';
import LanguageSelector from './LanguageSelector';
import CameraDetection from './CameraDetection';


type AppInnerProps = {
  allPiecesGame: gtpLib.JSGame;
  t: (key: string, options?: Record<string, unknown>) => string;
}

type AppState = {
  allPieces: gtpLib.JSPiece[],
  selectedPieceIds: Set<number>,
  closingPieceIds: Set<number>, // Track pieces currently animating out
  isGameValid: boolean,
  missingCells: number,
  searching: boolean,
  solutions?: gtpLib.JSMatrix[],
  showCameraDetection: boolean,
}

class AppInner extends React.Component<AppInnerProps, AppState> {
  constructor(props: AppInnerProps) {
    super(props);
    this.setPieceSelected = this.setPieceSelected.bind(this);
    this.handleAnimationEnd = this.handleAnimationEnd.bind(this);
    this.handleCameraPiecesConfirmed = this.handleCameraPiecesConfirmed.bind(this);
  }

  state: AppState = {
    allPieces: this.props.allPiecesGame.pieces,
    selectedPieceIds: new Set<number>(),
    closingPieceIds: new Set<number>(),
    isGameValid: false,
    missingCells: 0,
    searching: false,
    showCameraDetection: false,
  };

  setPieceSelected(pieceId: number, selected: boolean) {
    if (selected) {
      // Selecting: Add immediately
      let selectedPieceIds = this.state.selectedPieceIds;
      selectedPieceIds.add(pieceId);
      // Ensure it's not in closing list (in case of rapid toggling)
      let closingPieceIds = this.state.closingPieceIds;
      closingPieceIds.delete(pieceId);

      this.setSelectedPieceIds(selectedPieceIds, closingPieceIds);
    } else {
      // Unselecting: Add to closing list first to trigger animation
      let closingPieceIds = this.state.closingPieceIds;
      closingPieceIds.add(pieceId);
      this.setState({ closingPieceIds: closingPieceIds });
    }
  }

  handleAnimationEnd(pieceId: number) {
    // Animation finished, now actually remove it
    let selectedPieceIds = this.state.selectedPieceIds;
    selectedPieceIds.delete(pieceId);

    let closingPieceIds = this.state.closingPieceIds;
    closingPieceIds.delete(pieceId);

    this.setSelectedPieceIds(selectedPieceIds, closingPieceIds);
  }

  setSelectedPieceIds(selectedPieceIds: Set<number>, closingPieceIds: Set<number> = this.state.closingPieceIds) {
    if (selectedPieceIds.size === 0) {
      this.setState({
        selectedPieceIds: selectedPieceIds,
        closingPieceIds: closingPieceIds,
        isGameValid: false,
        missingCells: 0,
        searching: false,
        solutions: undefined
      });
      return;
    }

    let game = gtpLib.JSGame.game_from_game(this.props.allPiecesGame, Uint32Array.from(selectedPieceIds));
    let isGameValid = game.is_valid();

    this.setState({
      selectedPieceIds: selectedPieceIds,
      closingPieceIds: closingPieceIds,
      isGameValid: isGameValid,
      searching: false,
      missingCells: 0,
      solutions: undefined
    });

    const MIN_SEARCH_DISPLAY_MS = 1000;

    if (isGameValid) {
      const searchStartTime = Date.now();

      this.setState({
        searching: true
      }, () => {
        // Use setTimeout to allow the UI to render the "searching" state
        // before the heavy computation blocks the main thread.
        setTimeout(() => {
          // Start computation immediately
          let solutions = game.resolve();

          // Calculate how long the computation took
          const searchDuration = Date.now() - searchStartTime;
          const remainingDisplayTime = Math.max(0, MIN_SEARCH_DISPLAY_MS - searchDuration);

          // Ensure the "searching" message is visible for at least MIN_SEARCH_DISPLAY_MS
          setTimeout(() => {
            this.setState({
              searching: false,
              solutions: solutions
            });
          }, remainingDisplayTime);
        }, 0);
      });
    }
    else {
      this.setState({
        missingCells: game.missing_cells()
      })
    }
  }

  resetSelection = () => {
    this.setSelectedPieceIds(new Set<number>(), new Set<number>());
  }

  openCameraDetection = () => {
    this.setState({ showCameraDetection: true });
  }

  closeCameraDetection = () => {
    this.setState({ showCameraDetection: false });
  }

  handleCameraPiecesConfirmed(pieceIds: Set<number>) {
    this.setState({ showCameraDetection: false });
    this.setSelectedPieceIds(pieceIds, new Set<number>());
  }



  renderAllPieces = () => {
    const { t } = this.props;
    return (
      <div id='all-pieces-area'>
        <div className='section-title'>
          {t('choosePieces')}
          <button 
            className="camera-button" 
            onClick={this.openCameraDetection}
            title={t('scanWithCamera')}
          >
            📷
          </button>
        </div>

        {this.state.allPieces.map((piece) => {
          let isPieceSelected = this.state.selectedPieceIds.has(piece.id);
          return (
            <div key={piece.id}
              onClick={() => this.setPieceSelected(piece.id, !isPieceSelected)}
              className={`piece-container ${isPieceSelected ? "selected-piece" : ""}`}>
              <PieceView piece={piece}></PieceView>
            </div>)
        })}
      </div>
    );
  }

  renderSelectedPieces = () => {
    const { t } = this.props;
    if (!this.state.selectedPieceIds.size) {
      return (<div></div>);
    }

    let isGameValid = this.state.isGameValid;

    return (
      <div id='selected-pieces-area' className={isGameValid ? "valid-game" : ""}>
        {this.state.allPieces.map((piece) => {
          if (this.state.selectedPieceIds.has(piece.id)) {
            const isClosing = this.state.closingPieceIds.has(piece.id);
            return (
              <div
                key={piece.id}
                onClick={() => this.setPieceSelected(piece.id, false)}
                className={`piece-container ${isClosing ? "closing" : ""}`}
                onAnimationEnd={() => isClosing && this.handleAnimationEnd(piece.id)}
              >
                <PieceView piece={piece}></PieceView>
              </div>)
          } else {
            return null;
          }
        })}
        <div className="reset-button-container">
          <button className="reset-button" onClick={this.resetSelection}>
            🗑️ {t('clearAll')}
          </button>
        </div>
      </div>
    );
  }

  renderSolutions = () => {
    const { t } = this.props;
    if (this.state.missingCells > 0) {
      return (
        <div id='solutions-area'>
          <div className='solution-count'>
            {t('missingCells', { count: this.state.missingCells })}
          </div>
        </div>
      )
    }

    if (this.state.searching) {
      return (
        <div id='solutions-area'>
          <div className='solution-count'>
            {t('thinking')}
          </div>
          <div className="spinner"></div>
        </div>
      )
    }

    if (typeof (this.state.solutions) === 'undefined') {
      return (<div></div>);
    }

    if (this.state.solutions.length === 0) {
      return (
        <div id='solutions-area'>
          <div className='solution-count'>
            {t('noSolution')}
          </div>
        </div>
      )
    }

    return (
      <div id='solutions-area'>
        <div className='solution-count'>
          {t('foundSolutions', { count: this.state.solutions.length })}
        </div>
        <div className='solutions-grid'>
          {this.state.solutions.map((solution, index) => {
            return (
              <div key={solution.svg + index} className='solution-card'>
                <MatrixView matrix={solution}></MatrixView>
              </div>
            );
          })}
        </div>
      </div>
    );
  }

  render() {
    const { t } = this.props;
    return (
      <div className="App">
        <header className="App-header">
          GAGNE TON PAPA !
        </header>

        <div className="App-body">
          {this.renderAllPieces()}
          {this.renderSelectedPieces()}
          {this.renderSolutions()}
        </div>

        <footer className="App-footer">
          <div className="footer-content">
            <a href="https://github.com/manuroe/gagne-ton-papa" target="_blank" rel="noopener noreferrer">{t('sourceCode')}</a>
            <LanguageSelector />
          </div>
        </footer>

        {this.state.showCameraDetection && (
          <CameraDetection
            allPiecesGame={this.props.allPiecesGame}
            onPiecesConfirmed={this.handleCameraPiecesConfirmed}
            onClose={this.closeCameraDetection}
          />
        )}
      </div>
    );
  }
}

// Wrapper component that uses hook and passes t function to class component
type AppProps = {
  allPiecesGame: gtpLib.JSGame;
};

export default function App({ allPiecesGame }: AppProps) {
  const { t } = useTranslation();
  return <AppInner allPiecesGame={allPiecesGame} t={t} />;
}
