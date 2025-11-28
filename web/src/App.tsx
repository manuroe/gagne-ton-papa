import React from 'react';
import './App.css';

import * as gtpLib from 'lib-wasm';
import MatrixView from './MatrixView';
import PieceView from './PieceView';


type AppProps = {
  allPiecesGame: gtpLib.JSGame
}

type AppState = {
  allPieces: gtpLib.JSPiece[],
  selectedPieceIds: Set<number>,
  isGameValid: boolean,
  missingCells: number,
  searching: boolean,
  solutions?: gtpLib.JSMatrix[],
}

export default class App extends React.Component<AppProps, AppState> {
  constructor(props: AppProps) {
    super(props);
    this.setPieceSelected = this.setPieceSelected.bind(this);
  }

  state: AppState = {
    allPieces: this.props.allPiecesGame.pieces,
    selectedPieceIds: new Set<number>(),
    isGameValid: false,
    missingCells: 0,
    searching: false,
  };

  setPieceSelected(pieceId: number, selected: boolean) {
    let selectedPieceIds = this.state.selectedPieceIds;
    if (selected) {
      selectedPieceIds.add(pieceId);
    } else {
      selectedPieceIds.delete(pieceId);
    }

    this.setSelectedPieceIds(selectedPieceIds);
  }

  setSelectedPieceIds(selectedPieceIds: Set<number>) {
    if (selectedPieceIds.size === 0) {
      this.setState({
        selectedPieceIds: selectedPieceIds,
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
      isGameValid: isGameValid,
      searching: false,
      missingCells: 0,
      solutions: undefined
    });

    if (isGameValid) {
      this.setState({
        searching: true
      }, () => {
        // Use setTimeout to allow the UI to render the "searching" state
        // before the heavy computation blocks the main thread.
        setTimeout(() => {
          let solutions = game.resolve();
          this.setState({
            searching: false,
            solutions: solutions
          });
        }, 1000);
      });
    }
    else {
      this.setState({
        missingCells: game.missing_cells()
      })
    }
  }

  resetSelection = () => {
    this.setSelectedPieceIds(new Set<number>());
  }



  renderAllPieces = () => {
    return (
      <div id='all-pieces-area'>
        <div className='section-title'>
          Choisis tes pièces :
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
    if (!this.state.selectedPieceIds.size) {
      return (<div></div>);
    }

    let isGameValid = this.state.isGameValid;

    return (
      <div id='selected-pieces-area' className={isGameValid ? "valid-game" : ""}>
        {this.state.allPieces.map((piece) => {
          if (this.state.selectedPieceIds.has(piece.id)) {
            return (
              <div key={piece.id} onClick={() => this.setPieceSelected(piece.id, false)} className="piece-container">
                <PieceView piece={piece}></PieceView>
              </div>)
          } else {
            return null;
          }
        })}
        <div className="reset-button-container">
          <button className="reset-button" onClick={this.resetSelection}>
            🗑️ Tout effacer
          </button>
        </div>
      </div>
    );
  }

  renderSolutions = () => {
    if (this.state.missingCells > 0) {
      return (
        <div id='solutions-area'>
          <div className='solution-count'>
            Il manque des pièces pour recouvrir {this.state.missingCells} cases.
          </div>
        </div>
      )
    }

    if (this.state.searching) {
      return (
        <div id='solutions-area'>
          <div className='solution-count'>
            Je réfléchis... 🤔
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
            Pas de solution trouvée 😕
          </div>
        </div>
      )
    }

    return (
      <div id='solutions-area'>
        <div className='solution-count'>
          J'ai trouvé {this.state.solutions.length} solutions ! 🎉
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
          <a href="https://github.com/manuroe/gagne-ton-papa" target="_blank" rel="noopener noreferrer">Code source sur GitHub</a>
        </footer>
      </div>
    );
  }
}
