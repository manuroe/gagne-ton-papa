import React, { useRef, useState, useEffect, useCallback } from 'react';
import { useTranslation } from 'react-i18next';
import * as ort from 'onnxruntime-web';
import * as gtpLib from 'lib-wasm';
import PieceView from './PieceView';
import './CameraDetection.css';

// Piece class names from the ML model mapped to game piece IDs
// The ML model is trained on these class names
// Map them to the indices in the game's piece array (from Game::game_with_all_pieces)
const CLASS_LABELS: { [key: number]: { name: string; pieceId: number } } = {
  0: { name: 'RedSquare1', pieceId: 0 },
  1: { name: 'TanBar2', pieceId: 2 },
  2: { name: 'BrownL3', pieceId: 4 },
  3: { name: 'OrangeBar3', pieceId: 5 },
  4: { name: 'PinkBar4', pieceId: 6 },
  5: { name: 'GreenL4', pieceId: 7 },
  6: { name: 'BlueT4', pieceId: 8 },
  7: { name: 'YellowZigZag4', pieceId: 9 },
  8: { name: 'VioletSquare4', pieceId: 10 },
  9: { name: 'OrangeL5', pieceId: 11 },
  10: { name: 'BrownT5', pieceId: 12 },
  11: { name: 'VioletZigZag5', pieceId: 13 },
  12: { name: 'BlueL5', pieceId: 14 },
  13: { name: 'PinkNotSquare5', pieceId: 15 },
  14: { name: 'YellowU5', pieceId: 16 },
  15: { name: 'BlueS5', pieceId: 17 },
};

// Detection result
interface Detection {
  classId: number;
  pieceId: number;
  confidence: number;
  bbox: {
    x: number;
    y: number;
    width: number;
    height: number;
  };
}

type Props = {
  allPiecesGame: gtpLib.JSGame;
  onPiecesConfirmed: (pieceIds: Set<number>) => void;
  onClose: () => void;
};

// Constants for battery optimization
const DETECTION_INTERVAL_MS = 200; // Run detection every 200ms (5 fps for detection)
const VIDEO_TARGET_WIDTH = 640;
const VIDEO_TARGET_HEIGHT = 480;

// YOLO model input size
const MODEL_INPUT_SIZE = 640;

// Confidence threshold for detections
const CONFIDENCE_THRESHOLD = 0.5;
const NMS_IOU_THRESHOLD = 0.45;

export default function CameraDetection({ allPiecesGame, onPiecesConfirmed, onClose }: Props) {
  const { t } = useTranslation();
  const videoRef = useRef<HTMLVideoElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [isModelLoading, setIsModelLoading] = useState(true);
  const [modelLoadError, setModelLoadError] = useState<string | null>(null);
  const [session, setSession] = useState<ort.InferenceSession | null>(null);
  const [detections, setDetections] = useState<Detection[]>([]);
  const [cameraError, setCameraError] = useState<string | null>(null);
  const [confirmedPieceIds, setConfirmedPieceIds] = useState<Set<number>>(new Set());
  const [isProcessing, setIsProcessing] = useState(false);
  const [videoDimensions, setVideoDimensions] = useState({ width: 0, height: 0 });
  const animationFrameRef = useRef<number | null>(null);
  const lastDetectionTimeRef = useRef<number>(0);

  // Load ONNX model
  useEffect(() => {
    async function loadModel() {
      try {
        setIsModelLoading(true);
        setModelLoadError(null);
        
        // Configure ONNX Runtime WASM paths
        // Use PUBLIC_URL to support deployment on subdirectory (e.g., /gagne-ton-papa/)
        const basePath = process.env.PUBLIC_URL || '';
        ort.env.wasm.wasmPaths = basePath + '/';
        
        const modelPath = basePath + '/models/gtp-quantized.onnx';
        const newSession = await ort.InferenceSession.create(modelPath, {
          executionProviders: ['wasm'],
          graphOptimizationLevel: 'all',
        });
        
        setSession(newSession);
        setIsModelLoading(false);
      } catch (error) {
        console.error('Failed to load ONNX model:', error);
        setModelLoadError(error instanceof Error ? error.message : 'Failed to load model');
        setIsModelLoading(false);
      }
    }
    
    loadModel();
  }, []);

  // Initialize camera
  useEffect(() => {
    async function startCamera() {
      try {
        const stream = await navigator.mediaDevices.getUserMedia({
          video: {
            facingMode: 'environment', // Prefer back camera on mobile
            width: { ideal: VIDEO_TARGET_WIDTH },
            height: { ideal: VIDEO_TARGET_HEIGHT },
          },
          audio: false,
        });
        
        if (videoRef.current) {
          videoRef.current.srcObject = stream;
        }
      } catch (error) {
        console.error('Failed to access camera:', error);
        setCameraError(error instanceof Error ? error.message : 'Failed to access camera');
      }
    }
    
    startCamera();
    
    // Cleanup - copy ref to local variable
    const video = videoRef.current;
    return () => {
      if (video?.srcObject) {
        const stream = video.srcObject as MediaStream;
        stream.getTracks().forEach(track => track.stop());
      }
    };
  }, []);

  // Handle video metadata loaded to get dimensions
  const handleVideoMetadata = useCallback(() => {
    if (videoRef.current) {
      setVideoDimensions({
        width: videoRef.current.videoWidth,
        height: videoRef.current.videoHeight,
      });
    }
  }, []);

  // Preprocess image for YOLO model
  const preprocessImage = useCallback((video: HTMLVideoElement, canvas: HTMLCanvasElement): Float32Array => {
    const ctx = canvas.getContext('2d')!;
    canvas.width = MODEL_INPUT_SIZE;
    canvas.height = MODEL_INPUT_SIZE;
    
    // Calculate scaling to maintain aspect ratio
    const scale = Math.min(MODEL_INPUT_SIZE / video.videoWidth, MODEL_INPUT_SIZE / video.videoHeight);
    const scaledWidth = video.videoWidth * scale;
    const scaledHeight = video.videoHeight * scale;
    const offsetX = (MODEL_INPUT_SIZE - scaledWidth) / 2;
    const offsetY = (MODEL_INPUT_SIZE - scaledHeight) / 2;
    
    // Fill with gray (letterboxing)
    ctx.fillStyle = '#808080';
    ctx.fillRect(0, 0, MODEL_INPUT_SIZE, MODEL_INPUT_SIZE);
    
    // Draw scaled image
    ctx.drawImage(video, offsetX, offsetY, scaledWidth, scaledHeight);
    
    const imageData = ctx.getImageData(0, 0, MODEL_INPUT_SIZE, MODEL_INPUT_SIZE);
    const data = imageData.data;
    
    // Convert to CHW format and normalize to 0-1
    const float32Data = new Float32Array(3 * MODEL_INPUT_SIZE * MODEL_INPUT_SIZE);
    for (let i = 0; i < MODEL_INPUT_SIZE * MODEL_INPUT_SIZE; i++) {
      float32Data[i] = data[i * 4] / 255.0; // R
      float32Data[MODEL_INPUT_SIZE * MODEL_INPUT_SIZE + i] = data[i * 4 + 1] / 255.0; // G
      float32Data[2 * MODEL_INPUT_SIZE * MODEL_INPUT_SIZE + i] = data[i * 4 + 2] / 255.0; // B
    }
    
    return float32Data;
  }, []);

  // Non-maximum suppression
  const nonMaxSuppression = useCallback((detections: Detection[]): Detection[] => {
    if (detections.length === 0) return [];
    
    // Sort by confidence
    const sorted = [...detections].sort((a, b) => b.confidence - a.confidence);
    const selected: Detection[] = [];
    
    for (const detection of sorted) {
      let shouldSelect = true;
      
      for (const selectedDetection of selected) {
        const iou = calculateIoU(detection.bbox, selectedDetection.bbox);
        if (iou > NMS_IOU_THRESHOLD && detection.classId === selectedDetection.classId) {
          shouldSelect = false;
          break;
        }
      }
      
      if (shouldSelect) {
        selected.push(detection);
      }
    }
    
    return selected;
  }, []);

  // Run detection
  const runDetection = useCallback(async () => {
    if (!session || !videoRef.current || !canvasRef.current || isProcessing) {
      return;
    }
    
    const now = Date.now();
    if (now - lastDetectionTimeRef.current < DETECTION_INTERVAL_MS) {
      return;
    }
    lastDetectionTimeRef.current = now;
    
    try {
      setIsProcessing(true);
      
      const video = videoRef.current;
      const canvas = canvasRef.current;
      
      // Preprocess
      const inputData = preprocessImage(video, canvas);
      const tensor = new ort.Tensor('float32', inputData, [1, 3, MODEL_INPUT_SIZE, MODEL_INPUT_SIZE]);
      
      // Run inference
      const results = await session.run({ images: tensor });
      
      // Process output - YOLO output format is [1, 84, 8400] for YOLOv8
      // or [1, num_detections, 6] for post-processed output
      const output = results.output0 || results[Object.keys(results)[0]];
      const outputData = output.data as Float32Array;
      const outputDims = output.dims;
      
      const newDetections: Detection[] = [];
      
      // Calculate scale factors for letterboxed coordinates to original image coordinates
      const scale = Math.min(MODEL_INPUT_SIZE / video.videoWidth, MODEL_INPUT_SIZE / video.videoHeight);
      const offsetX = (MODEL_INPUT_SIZE - video.videoWidth * scale) / 2;
      const offsetY = (MODEL_INPUT_SIZE - video.videoHeight * scale) / 2;
      
      // Handle different YOLO output formats
      if (outputDims.length === 3 && outputDims[1] === 84) {
        // YOLOv8 format: [1, 84, 8400] - need to transpose to [1, 8400, 84]
        const numClasses = 16; // Number of piece classes
        const numPredictions = outputDims[2] as number;
        
        for (let i = 0; i < numPredictions; i++) {
          // Get box coordinates (first 4 values)
          const cx = outputData[0 * numPredictions + i];
          const cy = outputData[1 * numPredictions + i];
          const w = outputData[2 * numPredictions + i];
          const h = outputData[3 * numPredictions + i];
          
          // Get class confidences (next 16 values starting at index 4)
          let maxConf = 0;
          let maxClassId = 0;
          for (let c = 0; c < numClasses; c++) {
            const conf = outputData[(4 + c) * numPredictions + i];
            if (conf > maxConf) {
              maxConf = conf;
              maxClassId = c;
            }
          }
          
          if (maxConf >= CONFIDENCE_THRESHOLD && CLASS_LABELS[maxClassId]) {
            // Convert from letterboxed coordinates to original image coordinates
            const x1 = (cx - w / 2 - offsetX) / scale;
            const y1 = (cy - h / 2 - offsetY) / scale;
            const boxWidth = w / scale;
            const boxHeight = h / scale;
            
            newDetections.push({
              classId: maxClassId,
              pieceId: CLASS_LABELS[maxClassId].pieceId,
              confidence: maxConf,
              bbox: {
                x: x1,
                y: y1,
                width: boxWidth,
                height: boxHeight,
              },
            });
          }
        }
      } else if (outputDims.length === 3) {
        // Alternative format: [1, num_detections, 6] - (x1, y1, x2, y2, confidence, class)
        const numDetections = outputDims[1] as number;
        
        for (let i = 0; i < numDetections; i++) {
          const offset = i * 6;
          const x1 = (outputData[offset] - offsetX) / scale;
          const y1 = (outputData[offset + 1] - offsetY) / scale;
          const x2 = (outputData[offset + 2] - offsetX) / scale;
          const y2 = (outputData[offset + 3] - offsetY) / scale;
          const confidence = outputData[offset + 4];
          const classId = Math.round(outputData[offset + 5]);
          
          if (confidence >= CONFIDENCE_THRESHOLD && CLASS_LABELS[classId]) {
            newDetections.push({
              classId,
              pieceId: CLASS_LABELS[classId].pieceId,
              confidence,
              bbox: {
                x: x1,
                y: y1,
                width: x2 - x1,
                height: y2 - y1,
              },
            });
          }
        }
      }
      
      // Apply NMS
      const filteredDetections = nonMaxSuppression(newDetections);
      setDetections(filteredDetections);
      
    } catch (error) {
      console.error('Detection error:', error);
    } finally {
      setIsProcessing(false);
    }
  }, [session, isProcessing, preprocessImage, nonMaxSuppression]);

  // Detection loop
  useEffect(() => {
    if (!session || isModelLoading) return;
    
    const loop = () => {
      runDetection();
      animationFrameRef.current = requestAnimationFrame(loop);
    };
    
    animationFrameRef.current = requestAnimationFrame(loop);
    
    return () => {
      if (animationFrameRef.current) {
        cancelAnimationFrame(animationFrameRef.current);
      }
    };
  }, [session, isModelLoading, runDetection]);

  // Toggle piece confirmation
  const togglePieceConfirmation = useCallback((pieceId: number) => {
    setConfirmedPieceIds(prev => {
      const newSet = new Set(prev);
      if (newSet.has(pieceId)) {
        newSet.delete(pieceId);
      } else {
        newSet.add(pieceId);
      }
      return newSet;
    });
  }, []);

  // Handle confirmation
  const handleConfirm = useCallback(() => {
    onPiecesConfirmed(confirmedPieceIds);
  }, [confirmedPieceIds, onPiecesConfirmed]);

  // Get piece from game by ID
  const getPiece = useCallback((pieceId: number): gtpLib.JSPiece | null => {
    try {
      return allPiecesGame.piece(pieceId);
    } catch {
      return null;
    }
  }, [allPiecesGame]);

  // Render detection overlays
  const renderDetectionOverlays = () => {
    if (videoDimensions.width === 0) return null;
    
    return detections.map((detection, index) => {
      const piece = getPiece(detection.pieceId);
      if (!piece) return null;
      
      // Calculate position and scale relative to video display
      const videoElement = videoRef.current;
      if (!videoElement) return null;
      
      const displayWidth = videoElement.clientWidth;
      const displayHeight = videoElement.clientHeight;
      const scaleX = displayWidth / videoDimensions.width;
      const scaleY = displayHeight / videoDimensions.height;
      
      const left = detection.bbox.x * scaleX;
      const top = detection.bbox.y * scaleY;
      const width = detection.bbox.width * scaleX;
      const height = detection.bbox.height * scaleY;
      
      // Calculate rotation based on aspect ratio
      const aspectRatio = detection.bbox.width / detection.bbox.height;
      const rotation = aspectRatio < 1 ? 90 : 0;
      
      const isConfirmed = confirmedPieceIds.has(detection.pieceId);
      
      return (
        <div
          key={`detection-${index}-${detection.pieceId}`}
          className={`detection-overlay ${isConfirmed ? 'confirmed' : ''}`}
          style={{
            left: `${left}px`,
            top: `${top}px`,
            width: `${width}px`,
            height: `${height}px`,
          }}
          onClick={() => togglePieceConfirmation(detection.pieceId)}
        >
          <div 
            className="detection-piece-container"
            style={{
              transform: `rotate(${rotation}deg)`,
            }}
          >
            <PieceView piece={piece} />
          </div>
          <div className="detection-confidence">
            {Math.round(detection.confidence * 100)}%
          </div>
          {isConfirmed && <div className="detection-checkmark">✓</div>}
        </div>
      );
    });
  };

  // Get unique detected piece IDs
  const detectedPieceIds = new Set(detections.map(d => d.pieceId));

  if (cameraError) {
    return (
      <div className="camera-detection-container">
        <div className="camera-error">
          <h2>{t('cameraError')}</h2>
          <p>{cameraError}</p>
          <button onClick={onClose}>{t('back')}</button>
        </div>
      </div>
    );
  }

  if (modelLoadError) {
    return (
      <div className="camera-detection-container">
        <div className="camera-error">
          <h2>{t('modelLoadError')}</h2>
          <p>{modelLoadError}</p>
          <button onClick={onClose}>{t('back')}</button>
        </div>
      </div>
    );
  }

  return (
    <div className="camera-detection-container">
      <div className="camera-view-wrapper">
        <video
          ref={videoRef}
          autoPlay
          playsInline
          muted
          className="camera-video"
          onLoadedMetadata={handleVideoMetadata}
        />
        <canvas ref={canvasRef} style={{ display: 'none' }} />
        
        {isModelLoading && (
          <div className="loading-overlay">
            <div className="spinner"></div>
            <p>{t('loadingModel')}</p>
          </div>
        )}
        
        {!isModelLoading && renderDetectionOverlays()}
      </div>
      
      <div className="detection-controls">
        <div className="detected-pieces-summary">
          <span>{t('detectedPieces', { count: detectedPieceIds.size })}</span>
          <span className="confirmed-count">
            {t('confirmedPieces', { count: confirmedPieceIds.size })}
          </span>
        </div>
        
        <div className="detected-pieces-list">
          {Array.from(detectedPieceIds).map(pieceId => {
            const piece = getPiece(pieceId);
            if (!piece) return null;
            const isConfirmed = confirmedPieceIds.has(pieceId);
            return (
              <div
                key={pieceId}
                className={`detected-piece-item ${isConfirmed ? 'confirmed' : ''}`}
                onClick={() => togglePieceConfirmation(pieceId)}
              >
                <PieceView piece={piece} />
                {isConfirmed && <span className="checkmark">✓</span>}
              </div>
            );
          })}
        </div>
        
        <div className="action-buttons">
          <button className="cancel-button" onClick={onClose}>
            {t('cancel')}
          </button>
          <button
            className="confirm-button"
            onClick={handleConfirm}
            disabled={confirmedPieceIds.size === 0}
          >
            {t('usePieces', { count: confirmedPieceIds.size })}
          </button>
        </div>
      </div>
    </div>
  );
}

// Helper function to calculate IoU
function calculateIoU(box1: Detection['bbox'], box2: Detection['bbox']): number {
  const x1 = Math.max(box1.x, box2.x);
  const y1 = Math.max(box1.y, box2.y);
  const x2 = Math.min(box1.x + box1.width, box2.x + box2.width);
  const y2 = Math.min(box1.y + box1.height, box2.y + box2.height);
  
  const intersection = Math.max(0, x2 - x1) * Math.max(0, y2 - y1);
  const area1 = box1.width * box1.height;
  const area2 = box2.width * box2.height;
  const union = area1 + area2 - intersection;
  
  return union > 0 ? intersection / union : 0;
}
