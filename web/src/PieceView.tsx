import React from 'react';
import './App.css';
import * as gtpLib from 'lib-wasm';


type Props = {
  piece: gtpLib.JSPiece
}

export default class PieceView extends React.Component<Props, {}> {
  render() {
    const svg = new Blob([this.props.piece.matrix.svg], { type: "image/svg+xml" });
    const url = URL.createObjectURL(svg);
    return <img src={url} className="piece" style={{ maxWidth: 20 * this.props.piece.matrix.width }} alt=''/>
  }
}
