/**
 * Performance Monitor for Marco 2.0 Web
 * 
 * Tracks performance metrics and provides optimization recommendations
 */

export interface PerformanceMetrics {
  fps: number;
  frameTime: number;
  averageFrameTime: number;
  memoryUsage?: number;
  isVSyncEnabled: boolean;
  devicePixelRatio: number;
  timestamp: number;
}

export class PerformanceMonitor {
  private isRunning: boolean = false;
  private isPaused: boolean = false;
  private lastTimestamp: number = 0;
  private frameCount: number = 0;
  private frameTimes: number[] = [];
  private maxFrameTimeHistory: number = 60; // Keep last 60 frames
  
  private metrics: PerformanceMetrics = {
    fps: 0,
    frameTime: 0,
    averageFrameTime: 0,
    isVSyncEnabled: false,
    devicePixelRatio: window.devicePixelRatio || 1,
    timestamp: 0,
  };

  constructor() {
    this.detectVSync();
  }

  start(): void {
    this.isRunning = true;
    this.isPaused = false;
    this.lastTimestamp = performance.now();
    console.log('Performance monitoring started');
  }

  stop(): void {
    this.isRunning = false;
    console.log('Performance monitoring stopped');
  }

  pause(): void {
    this.isPaused = true;
  }

  resume(): void {
    this.isPaused = false;
    this.lastTimestamp = performance.now();
  }

  update(timestamp: number): void {
    if (!this.isRunning || this.isPaused) {
      return;
    }

    const deltaTime = timestamp - this.lastTimestamp;
    this.lastTimestamp = timestamp;
    this.frameCount++;

    // Update frame times
    this.frameTimes.push(deltaTime);
    if (this.frameTimes.length > this.maxFrameTimeHistory) {
      this.frameTimes.shift();
    }

    // Calculate metrics
    this.updateMetrics(timestamp, deltaTime);
  }

  getMetrics(): PerformanceMetrics {
    return { ...this.metrics };
  }

  isPerformanceGood(): boolean {
    return this.metrics.fps >= 55;
  }

  isPerformancePoor(): boolean {
    return this.metrics.fps < 30;
  }

  getPerformanceLevel(): 'high' | 'medium' | 'low' {
    if (this.metrics.fps >= 55) return 'high';
    if (this.metrics.fps >= 30) return 'medium';
    return 'low';
  }

  private updateMetrics(timestamp: number, deltaTime: number): void {
    // Calculate FPS
    this.metrics.fps = 1000 / deltaTime;
    this.metrics.frameTime = deltaTime;
    this.metrics.timestamp = timestamp;

    // Calculate average frame time
    if (this.frameTimes.length > 0) {
      const sum = this.frameTimes.reduce((a, b) => a + b, 0);
      this.metrics.averageFrameTime = sum / this.frameTimes.length;
    }

    // Get memory usage if available
    if ('memory' in performance) {
      const memInfo = (performance as any).memory;
      this.metrics.memoryUsage = memInfo.usedJSHeapSize / (1024 * 1024); // MB
    }

    // Update device pixel ratio
    this.metrics.devicePixelRatio = window.devicePixelRatio || 1;
  }

  private detectVSync(): void {
    // Simple VSync detection
    let frameCount = 0;
    let startTime = performance.now();

    const detectFrame = () => {
      frameCount++;
      const currentTime = performance.now();
      
      if (currentTime - startTime >= 1000) {
        const fps = frameCount;
        // VSync is likely enabled if FPS is close to common refresh rates
        this.metrics.isVSyncEnabled = fps >= 58 && fps <= 62 || fps >= 118 && fps <= 122;
        return;
      }
      
      if (frameCount < 120) { // Test for 2 seconds max
        requestAnimationFrame(detectFrame);
      }
    };

    requestAnimationFrame(detectFrame);
  }

  // Utility methods for performance optimization
  shouldReduceQuality(): boolean {
    return this.isPerformancePoor();
  }

  shouldDisableAnimations(): boolean {
    return this.metrics.fps < 25;
  }

  getRecommendedSettings(): {
    animationsEnabled: boolean;
    qualityLevel: 'high' | 'medium' | 'low';
    vsyncRecommended: boolean;
  } {
    const level = this.getPerformanceLevel();
    
    return {
      animationsEnabled: level !== 'low',
      qualityLevel: level,
      vsyncRecommended: this.metrics.isVSyncEnabled,
    };
  }
}
