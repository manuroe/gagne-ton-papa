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
      });

      let solutions = game.resolve();

      this.setState({
        searching: false,
        solutions: solutions
      });
    }
    else {
      this.setState({
        missingCells: game.missing_cells()
      })
    }
  }



  renderAllPieces = () => {
    return (
      <div id='all-pieces-area'>
        <div className='text-left'>
          Choisis des pièces:
        </div>

        {this.state.allPieces.map((piece) => {
          let isPieceSelected = this.state.selectedPieceIds.has(piece.id);
          return (
            <span key={piece.id}
              onClick={() => this.setPieceSelected(piece.id, !isPieceSelected)}
              className={isPieceSelected ? "selected-piece" : ""}>
              <PieceView piece={piece}></PieceView>
            </span>)
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
              <span key={piece.id} onClick={() => this.setPieceSelected(piece.id, false)}>
                <PieceView piece={piece}></PieceView>
              </span>)
          } else {
            return (<span></span>)
          }
        })}
      </div>
    );
  }

  renderSolutions = () => {
    if (this.state.missingCells > 0) {
      return (
        <div id='solutions-area'>
          Il manque des pièces pour recouvrir { this.state.missingCells } cases.
        </div>
      )
    }

    if (this.state.searching) {
      return (
        <div id='solutions-area'>
          Je cherche...
        </div>
      )
    }

    if (typeof (this.state.solutions) === 'undefined') {
      return (<div></div>);
    }

    return (
      <div id='solutions-area'>
        <div>
          Il y a {this.state.solutions.length} solutions.
        </div>
        {this.state.solutions.map((solution) => {
          return <MatrixView matrix={solution} key={solution.svg}></MatrixView>;
        })}
      </div>
    );
  }

  render() {
    return (
      <div className="App">
        <header className="App-header">
          Solutions pour <a className='App-link' href='https://www.gigamic.com/jeu/gagne-ton-papa' target="_blank" rel="noopener noreferrer"> GAGNE TON PAPA!</a>
        </header>

        <div className="App-body">
          {this.renderAllPieces()}
          {this.renderSelectedPieces()}
          {this.renderSolutions()}
        </div>

        <footer className="App-footer">
          <a href="https://github.com/manuroe/gagne-ton-papa">GitHub</a>
        </footer>
      </div>
    );
  }
}
