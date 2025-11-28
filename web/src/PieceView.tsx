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
    // Pass the width in "cells" to CSS via a custom property
    const style = { "--piece-width": this.props.piece.matrix.width } as React.CSSProperties;
    return <img src={url} className="piece-image" style={style} alt='' />
  }
}
