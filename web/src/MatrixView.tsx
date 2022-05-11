import React from 'react';
import './App.css';
import * as gtpLib from 'lib-wasm';


type Props = {
  matrix: gtpLib.JSMatrix
}

export default class MatrixView extends React.Component<Props, {}> {
  render() {
    const svg = new Blob([this.props.matrix.svg], { type: "image/svg+xml" });
    const url = URL.createObjectURL(svg);
    return <img src={url} className="solution" alt=''/>
  }
}
