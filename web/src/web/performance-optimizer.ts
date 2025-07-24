/**
 * Performance Optimization Module
 * 
 * Enhanced performance monitoring and optimization for Marco 2.0 Web
 * Includes frame rate monitoring, memory tracking, and adaptive quality
 */

export interface PerformanceMetrics {
  fps: number;
  frameTime: number;
  memoryUsage: number;
  renderTime: number;
  wasmHeapSize: number;
  touchLatency: number;
  isOptimized: boolean;
}

export interface OptimizationSettings {
  targetFps: number;
  maxMemoryMB: number;
  enableAdaptiveQuality: boolean;
  enableFrameRateLimit: boolean;
  enableMemoryCleanup: boolean;
  qualityLevel: 'low' | 'medium' | 'high' | 'auto';
}

export class PerformanceOptimizer {
  private metrics: PerformanceMetrics;
  private settings: OptimizationSettings;
  private frameBuffer: number[] = [];
  private memoryBuffer: number[] = [];
  private lastFrameTime = 0;
  private frameCount = 0;
  private rafId: number | null = null;
  private isMonitoring = false;
  
  // Performance thresholds
  private readonly PERFORMANCE_THRESHOLDS = {
    MIN_FPS_DESKTOP: 30,
    MIN_FPS_MOBILE: 20,
    MAX_MEMORY_MB: 100,
    MAX_FRAME_TIME_MS: 33.33, // 30fps
    FRAME_BUFFER_SIZE: 60,
    MEMORY_BUFFER_SIZE: 30,
  };
  
  // Device capability detection
  private deviceCapabilities = {
    isHighEnd: false,
    isMobile: false,
    hasWebGL2: false,
    cores: navigator.hardwareConcurrency || 4,
    memory: (navigator as any).deviceMemory || 4,
  };

  constructor(settings: Partial<OptimizationSettings> = {}) {
    this.settings = {
      targetFps: 60,
      maxMemoryMB: 80,
      enableAdaptiveQuality: true,
      enableFrameRateLimit: true,
      enableMemoryCleanup: true,
      qualityLevel: 'auto',
      ...settings,
    };
    
    this.metrics = {
      fps: 0,
      frameTime: 0,
      memoryUsage: 0,
      renderTime: 0,
      wasmHeapSize: 0,
      touchLatency: 0,
      isOptimized: false,
    };
    
    this.detectDeviceCapabilities();
    this.initializePerformanceObserver();
  }

  /**
   * Start performance monitoring
   */
  public startMonitoring(): void {
    if (this.isMonitoring) return;
    
    this.isMonitoring = true;
    this.lastFrameTime = performance.now();
    this.frameLoop();
    
    // Memory monitoring interval
    setInterval(() => this.updateMemoryMetrics(), 1000);
    
    console.log('Performance monitoring started');
  }

  /**
   * Stop performance monitoring
   */
  public stopMonitoring(): void {
    if (!this.isMonitoring) return;
    
    this.isMonitoring = false;
    if (this.rafId) {
      cancelAnimationFrame(this.rafId);
      this.rafId = null;
    }
    
    console.log('Performance monitoring stopped');
  }

  /**
   * Get current performance metrics
   */
  public getMetrics(): PerformanceMetrics {
    return { ...this.metrics };
  }

  /**
   * Update optimization settings
   */
  public updateSettings(newSettings: Partial<OptimizationSettings>): void {
    this.settings = { ...this.settings, ...newSettings };
    this.applyOptimizations();
  }

  /**
   * Measure touch input latency
   */
  public measureTouchLatency(touchStartTime: number): void {
    const latency = performance.now() - touchStartTime;
    this.metrics.touchLatency = latency;
    
    // Track touch performance
    if (latency > 16) { // More than one frame
      console.warn(`High touch latency detected: ${latency.toFixed(2)}ms`);
      this.suggestOptimizations('touch');
    }
  }

  /**
   * Measure WASM operation time
   */
  public measureWasmOperation<T>(operation: () => T, operationName: string): T {
    const startTime = performance.now();
    const result = operation();
    const endTime = performance.now();
    
    const duration = endTime - startTime;
    this.metrics.renderTime = duration;
    
    if (duration > 16) { // More than one frame
      console.warn(`Slow WASM operation ${operationName}: ${duration.toFixed(2)}ms`);
      this.suggestOptimizations('wasm');
    }
    
    return result;
  }

  /**
   * Request performance optimization
   */
  public optimize(): void {
    this.applyOptimizations();
    this.metrics.isOptimized = true;
  }

  /**
   * Get optimization recommendations
   */
  public getRecommendations(): string[] {
    const recommendations: string[] = [];
    
    if (this.metrics.fps < this.PERFORMANCE_THRESHOLDS.MIN_FPS_MOBILE) {
      recommendations.push('Reduce render quality to improve frame rate');
    }
    
    if (this.metrics.memoryUsage > this.settings.maxMemoryMB) {
      recommendations.push('Enable memory cleanup or reduce canvas size');
    }
    
    if (this.metrics.touchLatency > 32) {
      recommendations.push('Optimize touch handling or reduce processing load');
    }
    
    if (this.metrics.renderTime > 20) {
      recommendations.push('Simplify rendering pipeline or enable frame limiting');
    }
    
    return recommendations;
  }

  /**
   * Frame monitoring loop
   */
  private frameLoop(): void {
    if (!this.isMonitoring) return;
    
    const currentTime = performance.now();
    const frameTime = currentTime - this.lastFrameTime;
    
    // Update frame metrics
    this.frameBuffer.push(frameTime);
    if (this.frameBuffer.length > this.PERFORMANCE_THRESHOLDS.FRAME_BUFFER_SIZE) {
      this.frameBuffer.shift();
    }
    
    // Calculate FPS from frame buffer
    const avgFrameTime = this.frameBuffer.reduce((a, b) => a + b, 0) / this.frameBuffer.length;
    this.metrics.fps = 1000 / avgFrameTime;
    this.metrics.frameTime = avgFrameTime;
    
    this.frameCount++;
    this.lastFrameTime = currentTime;
    
    // Check for performance issues
    if (this.frameCount % 60 === 0) {
      this.checkPerformanceThresholds();
    }
    
    this.rafId = requestAnimationFrame(() => this.frameLoop());
  }

  /**
   * Update memory usage metrics
   */
  private updateMemoryMetrics(): void {
    // Standard memory API
    if ('memory' in performance) {
      const memory = (performance as any).memory;
      const memoryMB = memory.usedJSHeapSize / (1024 * 1024);
      
      this.memoryBuffer.push(memoryMB);
      if (this.memoryBuffer.length > this.PERFORMANCE_THRESHOLDS.MEMORY_BUFFER_SIZE) {
        this.memoryBuffer.shift();
      }
      
      this.metrics.memoryUsage = memoryMB;
    }
    
    // WASM memory estimation (if available)
    if (typeof WebAssembly !== 'undefined' && WebAssembly.Memory) {
      // This would need integration with the WASM module
      // For now, we'll estimate based on canvas operations
      this.estimateWasmMemory();
    }
  }

  /**
   * Estimate WASM memory usage
   */
  private estimateWasmMemory(): void {
    // Rough estimation based on canvas size and operations
    const canvas = document.querySelector('canvas');
    if (canvas) {
      const pixels = canvas.width * canvas.height;
      const estimatedMB = (pixels * 4) / (1024 * 1024); // 4 bytes per pixel
      this.metrics.wasmHeapSize = estimatedMB;
    }
  }

  /**
   * Check performance thresholds and trigger optimizations
   */
  private checkPerformanceThresholds(): void {
    const { fps, memoryUsage, touchLatency } = this.metrics;
    const { MIN_FPS_DESKTOP, MIN_FPS_MOBILE, MAX_MEMORY_MB } = this.PERFORMANCE_THRESHOLDS;
    
    const targetFps = this.deviceCapabilities.isMobile ? MIN_FPS_MOBILE : MIN_FPS_DESKTOP;
    
    if (fps < targetFps && this.settings.enableAdaptiveQuality) {
      this.reduceQuality();
    }
    
    if (memoryUsage > MAX_MEMORY_MB && this.settings.enableMemoryCleanup) {
      this.cleanupMemory();
    }
    
    if (touchLatency > 50) {
      this.optimizeTouchHandling();
    }
  }

  /**
   * Apply current optimization settings
   */
  private applyOptimizations(): void {
    if (this.settings.enableFrameRateLimit) {
      this.limitFrameRate();
    }
    
    if (this.settings.qualityLevel !== 'auto') {
      this.setQualityLevel(this.settings.qualityLevel);
    }
    
    if (this.settings.enableMemoryCleanup) {
      this.scheduleMemoryCleanup();
    }
  }

  /**
   * Reduce rendering quality to improve performance
   */
  private reduceQuality(): void {
    console.log('Reducing quality to improve performance');
    
    // Reduce canvas resolution
    const canvas = document.querySelector('canvas') as HTMLCanvasElement;
    if (canvas) {
      const scale = 0.8;
      canvas.style.transform = `scale(${1/scale})`;
      canvas.style.transformOrigin = 'top left';
      canvas.width *= scale;
      canvas.height *= scale;
    }
    
    // Reduce particle density, disable effects, etc.
    this.dispatchOptimizationEvent('quality-reduced', { level: 'low' });
  }

  /**
   * Set specific quality level
   */
  private setQualityLevel(level: 'low' | 'medium' | 'high'): void {
    const qualitySettings = {
      low: { scale: 0.7, effects: false, antialiasing: false },
      medium: { scale: 0.85, effects: true, antialiasing: false },
      high: { scale: 1.0, effects: true, antialiasing: true },
    };
    
    const settings = qualitySettings[level];
    this.dispatchOptimizationEvent('quality-changed', settings);
  }

  /**
   * Limit frame rate to reduce CPU usage
   */
  private limitFrameRate(): void {
    const targetInterval = 1000 / this.settings.targetFps;
    this.dispatchOptimizationEvent('framerate-limited', { interval: targetInterval });
  }

  /**
   * Clean up memory by triggering garbage collection
   */
  private cleanupMemory(): void {
    console.log('Cleaning up memory');
    
    // Force garbage collection if available
    if ('gc' in window) {
      (window as any).gc();
    }
    
    // Clear unnecessary caches
    this.dispatchOptimizationEvent('memory-cleanup', {});
  }

  /**
   * Schedule periodic memory cleanup
   */
  private scheduleMemoryCleanup(): void {
    setInterval(() => {
      if (this.metrics.memoryUsage > this.settings.maxMemoryMB * 0.8) {
        this.cleanupMemory();
      }
    }, 30000); // Every 30 seconds
  }

  /**
   * Optimize touch input handling
   */
  private optimizeTouchHandling(): void {
    console.log('Optimizing touch handling');
    
    // Reduce touch event frequency
    this.dispatchOptimizationEvent('touch-optimized', {
      throttle: true,
      passive: true,
    });
  }

  /**
   * Detect device capabilities
   */
  private detectDeviceCapabilities(): void {
    // Mobile detection
    this.deviceCapabilities.isMobile = /Mobi|Android/i.test(navigator.userAgent);
    
    // High-end device detection
    this.deviceCapabilities.isHighEnd = 
      this.deviceCapabilities.cores >= 8 && 
      this.deviceCapabilities.memory >= 8;
    
    // WebGL2 support
    try {
      const canvas = document.createElement('canvas');
      const gl = canvas.getContext('webgl2');
      this.deviceCapabilities.hasWebGL2 = !!gl;
    } catch (e) {
      this.deviceCapabilities.hasWebGL2 = false;
    }
    
    console.log('Device capabilities detected:', this.deviceCapabilities);
  }

  /**
   * Initialize Performance Observer for detailed metrics
   */
  private initializePerformanceObserver(): void {
    if ('PerformanceObserver' in window) {
      try {
        const observer = new PerformanceObserver((list) => {
          const entries = list.getEntries();
          entries.forEach((entry) => {
            if (entry.entryType === 'measure') {
              console.log(`Performance measure: ${entry.name} took ${entry.duration}ms`);
            }
          });
        });
        
        observer.observe({ entryTypes: ['measure', 'navigation'] });
      } catch (e) {
        console.warn('PerformanceObserver not fully supported');
      }
    }
  }

  /**
   * Suggest optimizations based on performance bottlenecks
   */
  private suggestOptimizations(category: 'touch' | 'wasm' | 'memory' | 'rendering'): void {
    const suggestions = {
      touch: [
        'Enable touch event throttling',
        'Use passive touch listeners',
        'Reduce touch point processing',
      ],
      wasm: [
        'Optimize WASM function calls',
        'Reduce data transfer between JS and WASM',
        'Use WASM SIMD if available',
      ],
      memory: [
        'Enable automatic garbage collection',
        'Reduce canvas buffer size',
        'Clear unused resources',
      ],
      rendering: [
        'Reduce render resolution',
        'Disable expensive effects',
        'Limit frame rate',
      ],
    };
    
    console.group(`Performance Suggestions - ${category}`);
    suggestions[category].forEach(suggestion => console.log(`• ${suggestion}`));
    console.groupEnd();
  }

  /**
   * Dispatch optimization events to the main application
   */
  private dispatchOptimizationEvent(type: string, data: any): void {
    window.dispatchEvent(new CustomEvent('marco2-optimization', {
      detail: { type, data, timestamp: performance.now() }
    }));
  }
}

/**
 * Performance utilities
 */
export class PerformanceUtils {
  /**
   * Debounce function calls for performance
   */
  static debounce<T extends (...args: any[]) => any>(
    func: T, 
    wait: number
  ): (...args: Parameters<T>) => void {
    let timeout: number;
    return (...args: Parameters<T>) => {
      clearTimeout(timeout);
      timeout = window.setTimeout(() => func(...args), wait);
    };
  }

  /**
   * Throttle function calls for performance
   */
  static throttle<T extends (...args: any[]) => any>(
    func: T, 
    limit: number
  ): (...args: Parameters<T>) => void {
    let inThrottle: boolean;
    return (...args: Parameters<T>) => {
      if (!inThrottle) {
        func(...args);
        inThrottle = true;
        setTimeout(() => inThrottle = false, limit);
      }
    };
  }

  /**
   * Measure function execution time
   */
  static measure<T>(name: string, func: () => T): T {
    performance.mark(`${name}-start`);
    const result = func();
    performance.mark(`${name}-end`);
    performance.measure(name, `${name}-start`, `${name}-end`);
    return result;
  }

  /**
   * Check if device supports high refresh rate
   */
  static supportsHighRefreshRate(): boolean {
    return 'getDisplayMedia' in navigator.mediaDevices;
  }

  /**
   * Get optimal canvas size for device
   */
  static getOptimalCanvasSize(): { width: number; height: number } {
    const dpr = window.devicePixelRatio || 1;
    const maxSize = /Mobi|Android/i.test(navigator.userAgent) ? 1024 : 2048;
    
    const width = Math.min(window.innerWidth * dpr, maxSize);
    const height = Math.min(window.innerHeight * dpr, maxSize);
    
    return { width, height };
  }
}

export default PerformanceOptimizer;
