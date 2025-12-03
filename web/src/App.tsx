import React from 'react';
import './App.css';
import { useTranslation } from 'react-i18next';

import * as gtpLib from 'lib-wasm';
import MatrixView from './MatrixView';
import PieceView from './PieceView';
import LanguageSelector from './LanguageSelector';


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
  totalSolutionsFound: number,
  isLoadingMore: boolean,
}

class AppInner extends React.Component<AppInnerProps, AppState> {
  constructor(props: AppInnerProps) {
    super(props);
    this.setPieceSelected = this.setPieceSelected.bind(this);
    this.handleAnimationEnd = this.handleAnimationEnd.bind(this);
  }

  // Abort token to cancel background solution loading when selection changes or is cleared
  private currentLoadToken: number = 0;


  state: AppState = {
    allPieces: this.props.allPiecesGame.pieces,
    selectedPieceIds: new Set<number>(),
    closingPieceIds: new Set<number>(),
    isGameValid: false,
    missingCells: 0,
    searching: false,
    totalSolutionsFound: 0,
    isLoadingMore: false,
  };

  calculateTotalCells(selectedPieceIds: Set<number>): number {
    let total = 0;
    this.state.allPieces.forEach(p => {
      if (selectedPieceIds.has(p.id)) {
        total += p.cells;
      }
    });
    return total;
  }

  setPieceSelected(pieceId: number, selected: boolean) {
    if (selected) {
      // Selecting: Add immediately
      let selectedPieceIds = new Set(this.state.selectedPieceIds); // Create a mutable copy
      // Check if adding this piece would exceed the 64-cell limit
      const currentCells = this.calculateTotalCells(selectedPieceIds);
      const piece = this.state.allPieces.find(p => p.id === pieceId);
      const pieceCells = piece ? piece.cells : 0;

      if (currentCells + pieceCells > 64) {
        // Ideally show a toast/notification here, but for now just prevent selection
        console.warn("Cannot select piece: Board would exceed 64 cells.");
        return;
      }

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
      // Cancel any ongoing progressive loading
      this.currentLoadToken++;
      this.setState({
        selectedPieceIds: selectedPieceIds,
        closingPieceIds: closingPieceIds,
        isGameValid: false,
        missingCells: 0,
        searching: false,
        solutions: undefined,
        totalSolutionsFound: 0,
        isLoadingMore: false,
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
      solutions: undefined,
      totalSolutionsFound: 0,
      isLoadingMore: false,
    });

    if (isGameValid) {
      // Begin a new progressive load cycle; invalidate previous ones
      this.currentLoadToken++;
      const loadToken = this.currentLoadToken;
      this.setState({
        searching: true,
        isLoadingMore: true,
      }, () => {
        // Load all solutions progressively
        this.loadAllSolutionsProgressively(game, loadToken);
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

  loadAllSolutionsProgressively = (game: gtpLib.JSGame, loadToken: number) => {
    const pageSize = 20;
    let currentPage = 0;
    let allSolutions: gtpLib.JSMatrix[] = [];

    const loadNextBatch = () => {
      setTimeout(() => {
        // Abort if a new loading cycle started
        if (loadToken !== this.currentLoadToken) {
          return;
        }
        const newSolutions = game.resolve_page(currentPage, pageSize);
        
        if (newSolutions.length > 0) {
          allSolutions.push(...newSolutions);
          this.setState({
            solutions: [...allSolutions],
            totalSolutionsFound: allSolutions.length,
            searching: true,
            isLoadingMore: true,
          });

          if (newSolutions.length === pageSize) {
            // More solutions might exist, load next batch
            currentPage++;
            loadNextBatch();
          } else {
            // Last batch loaded
            this.setState({
              searching: false,
              isLoadingMore: false,
            });
          }
        } else {
          // No more solutions; if none found at all, set empty array so UI can show "noSolution"
          this.setState({
            solutions: allSolutions,
            totalSolutionsFound: allSolutions.length,
            searching: false,
            isLoadingMore: false,
          });
        }
      }, 0);
    };

    loadNextBatch();
  }



  renderAllPieces = () => {
    const { t } = this.props;
    return (
      <div id='all-pieces-area'>
        <div className='section-title'>
          {t('choosePieces')}
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

    if (this.state.searching && this.state.totalSolutionsFound === 0) {
      return (
        <div id='solutions-area'>
          <div className='solution-count'>
            {t('thinking')}
          </div>
        </div>
      )
    }

    if (typeof (this.state.solutions) === 'undefined') {
      return (<div></div>);
    }

    if (this.state.solutions.length === 0 && !this.state.isLoadingMore) {
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
          {this.state.isLoadingMore 
            ? t('foundSolutionsLoading', { count: this.state.totalSolutionsFound })
            : t('foundSolutions', { count: this.state.solutions.length })}
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
