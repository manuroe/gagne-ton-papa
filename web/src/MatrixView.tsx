import React from 'react';
import './App.css';
import * as gtpLib from 'lib-wasm';


type Props = {
  matrix: gtpLib.JSMatrix
}

type State = {
  isVisible: boolean;
  blobUrl: string | null;
}

export default class MatrixView extends React.Component<Props, State> {
  private containerRef: React.RefObject<HTMLDivElement>;
  private observer: IntersectionObserver | null = null;

  constructor(props: Props) {
    super(props);
    this.state = {
      isVisible: false,
      blobUrl: null,
    };
    this.containerRef = React.createRef();
  }

  componentDidMount() {
    // Set up Intersection Observer for lazy loading
    this.observer = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting && !this.state.isVisible) {
            // Create blob URL when image becomes visible
            const svg = new Blob([this.props.matrix.svg], { type: "image/svg+xml" });
            const url = URL.createObjectURL(svg);
            this.setState({ isVisible: true, blobUrl: url });
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
    // Revoke blob URL to free memory
    if (this.state.blobUrl) {
      URL.revokeObjectURL(this.state.blobUrl);
    }
    if (this.observer && this.containerRef.current) {
      this.observer.unobserve(this.containerRef.current);
    }
  }

  render() {
    // Only render SVG when visible
    if (!this.state.isVisible || !this.state.blobUrl) {
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

    return (
      <div ref={this.containerRef}>
        <img src={this.state.blobUrl} className="solution-image" alt='' />
      </div>
    );
  }
}
