import React from 'react';
import './App.css';
import * as gtpLib from 'lib-wasm';


type Props = {
  matrix: gtpLib.JSMatrix
}

type State = {
  isVisible: boolean;
}

export default class MatrixView extends React.Component<Props, State> {
  private containerRef: React.RefObject<HTMLDivElement>;
  private observer: IntersectionObserver | null = null;

  constructor(props: Props) {
    super(props);
    this.state = {
      isVisible: false,
    };
    this.containerRef = React.createRef();
  }

  componentDidMount() {
    // Set up Intersection Observer for lazy loading
    this.observer = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting && !this.state.isVisible) {
            this.setState({ isVisible: true });
            // Once loaded, stop observing
            if (this.observer && this.containerRef.current) {
              this.observer.unobserve(this.containerRef.current);
            }
          }
        });
      },
      {
        rootMargin: '50px', // Start loading slightly before entering viewport
      }
    );

    if (this.containerRef.current) {
      this.observer.observe(this.containerRef.current);
    }
  }

  componentWillUnmount() {
    if (this.observer && this.containerRef.current) {
      this.observer.unobserve(this.containerRef.current);
    }
  }

  render() {
    // Only render SVG when visible
    if (!this.state.isVisible) {
      // Placeholder with aspect ratio to prevent layout shift
      const aspectRatio = this.props.matrix.height / this.props.matrix.width;
      return (
        <div 
          ref={this.containerRef}
          className="solution-image-placeholder"
          style={{ paddingBottom: `${aspectRatio * 100}%` }}
        />
      );
    }

    const svg = new Blob([this.props.matrix.svg], { type: "image/svg+xml" });
    const url = URL.createObjectURL(svg);
    return (
      <div ref={this.containerRef}>
        <img src={url} className="solution-image" alt='' />
      </div>
    );
  }
}
