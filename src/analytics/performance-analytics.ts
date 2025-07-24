/**
 * Marco 2.0 - Performance Analytics
 * Real-time performance monitoring and optimization insights
 */

export interface PerformanceMetric {
  name: string;
  value: number;
  unit: 'ms' | 'bytes' | 'fps' | 'count' | 'percentage';
  timestamp: number;
  category: 'load' | 'render' | 'interaction' | 'memory' | 'network' | 'wasm';
  threshold?: {
    warning: number;
    critical: number;
  };
}

export interface PerformanceSnapshot {
  timestamp: number;
  sessionId: string;
  metrics: PerformanceMetric[];
  context: {
    url: string;
    viewport: { width: number; height: number };
    scope?: string;
    activeNodes: number;
    connections: number;
  };
}

export interface PerformanceBudget {
  loadTime: number; // Max initial load time in ms
  renderTime: number; // Max render time in ms
  memoryUsage: number; // Max memory usage in bytes
  bundleSize: number; // Max bundle size in bytes
  frameRate: number; // Min frame rate in fps
  interactionLatency: number; // Max interaction response time in ms
}

export class PerformanceAnalytics {
  private metrics: PerformanceMetric[] = [];
  private snapshots: PerformanceSnapshot[] = [];
  private sessionId: string;
  private budget: PerformanceBudget;
  private observers: Map<string, PerformanceObserver> = new Map();
  private frameRateMonitor?: number;
  private memoryMonitor?: number;
  private isMonitoring: boolean = false;
  private endpoint: string = '/api/performance';

  constructor(config?: {
    budget?: Partial<PerformanceBudget>;
    endpoint?: string;
  }) {
    this.sessionId = this.generateSessionId();
    this.endpoint = config?.endpoint || this.endpoint;
    
    // Default performance budget for Marco 2.0
    this.budget = {
      loadTime: 3000, // 3 seconds
      renderTime: 100, // 100ms
      memoryUsage: 100 * 1024 * 1024, // 100MB
      bundleSize: 5 * 1024 * 1024, // 5MB
      frameRate: 60, // 60 FPS
      interactionLatency: 50, // 50ms
      ...config?.budget
    };

    this.initialize();
  }

  private generateSessionId(): string {
    return `perf_session_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
  }

  private initialize(): void {
    this.setupPerformanceObservers();
    this.startFrameRateMonitoring();
    this.startMemoryMonitoring();
    this.trackInitialLoad();
    this.setupInteractionMonitoring();
    this.isMonitoring = true;
  }

  private setupPerformanceObservers(): void {
    // Navigation timing
    if ('PerformanceObserver' in window) {
      const navObserver = new PerformanceObserver((list) => {
        for (const entry of list.getEntries()) {
          this.processNavigationTiming(entry as PerformanceNavigationTiming);
        }
      });
      navObserver.observe({ entryTypes: ['navigation'] });
      this.observers.set('navigation', navObserver);

      // Resource timing
      const resourceObserver = new PerformanceObserver((list) => {
        for (const entry of list.getEntries()) {
          this.processResourceTiming(entry as PerformanceResourceTiming);
        }
      });
      resourceObserver.observe({ entryTypes: ['resource'] });
      this.observers.set('resource', resourceObserver);

      // Measure timing
      const measureObserver = new PerformanceObserver((list) => {
        for (const entry of list.getEntries()) {
          this.processMeasureTiming(entry);
        }
      });
      measureObserver.observe({ entryTypes: ['measure'] });
      this.observers.set('measure', measureObserver);

      // Long task timing
      const longTaskObserver = new PerformanceObserver((list) => {
        for (const entry of list.getEntries()) {
          this.processLongTask(entry);
        }
      });
      longTaskObserver.observe({ entryTypes: ['longtask'] });
      this.observers.set('longtask', longTaskObserver);
    }
  }

  private processNavigationTiming(entry: PerformanceNavigationTiming): void {
    const metrics: PerformanceMetric[] = [
      {
        name: 'dom_content_loaded',
        value: entry.domContentLoadedEventEnd - entry.fetchStart,
        unit: 'ms',
        timestamp: Date.now(),
        category: 'load',
        threshold: { warning: 2000, critical: 4000 }
      },
      {
        name: 'load_complete',
        value: entry.loadEventEnd - entry.fetchStart,
        unit: 'ms',
        timestamp: Date.now(),
        category: 'load',
        threshold: { warning: this.budget.loadTime * 0.8, critical: this.budget.loadTime }
      },
      {
        name: 'first_byte',
        value: entry.responseStart - entry.fetchStart,
        unit: 'ms',
        timestamp: Date.now(),
        category: 'network',
        threshold: { warning: 500, critical: 1000 }
      }
    ];

    metrics.forEach(metric => this.addMetric(metric));
  }

  private processResourceTiming(entry: PerformanceResourceTiming): void {
    // Track bundle size and load times
    if (entry.name.includes('.wasm') || entry.name.includes('.js') || entry.name.includes('.css')) {
      const metric: PerformanceMetric = {
        name: 'resource_load_time',
        value: entry.duration,
        unit: 'ms',
        timestamp: Date.now(),
        category: 'load',
        threshold: { warning: 500, critical: 1000 }
      };

      this.addMetric(metric);

      // Track transfer size for bundle size monitoring
      if (entry.transferSize) {
        const sizeMetric: PerformanceMetric = {
          name: 'resource_size',
          value: entry.transferSize,
          unit: 'bytes',
          timestamp: Date.now(),
          category: 'network'
        };

        this.addMetric(sizeMetric);
      }
    }
  }

  private processMeasureTiming(entry: PerformanceEntry): void {
    const metric: PerformanceMetric = {
      name: entry.name,
      value: entry.duration,
      unit: 'ms',
      timestamp: Date.now(),
      category: this.categorizeMeasure(entry.name)
    };

    this.addMetric(metric);
  }

  private processLongTask(entry: PerformanceEntry): void {
    const metric: PerformanceMetric = {
      name: 'long_task',
      value: entry.duration,
      unit: 'ms',
      timestamp: Date.now(),
      category: 'render',
      threshold: { warning: 50, critical: 100 }
    };

    this.addMetric(metric);
  }

  private categorizeMeasure(name: string): PerformanceMetric['category'] {
    if (name.includes('render') || name.includes('paint')) return 'render';
    if (name.includes('load') || name.includes('fetch')) return 'load';
    if (name.includes('wasm')) return 'wasm';
    if (name.includes('interaction')) return 'interaction';
    return 'render';
  }

  private startFrameRateMonitoring(): void {
    let lastTime = performance.now();
    let frameCount = 0;
    
    const measureFPS = () => {
      frameCount++;
      const currentTime = performance.now();
      
      if (currentTime - lastTime >= 1000) { // Every second
        const fps = (frameCount * 1000) / (currentTime - lastTime);
        
        const metric: PerformanceMetric = {
          name: 'frame_rate',
          value: fps,
          unit: 'fps',
          timestamp: Date.now(),
          category: 'render',
          threshold: { warning: this.budget.frameRate * 0.8, critical: this.budget.frameRate * 0.6 }
        };

        this.addMetric(metric);
        
        frameCount = 0;
        lastTime = currentTime;
      }
      
      if (this.isMonitoring) {
        this.frameRateMonitor = requestAnimationFrame(measureFPS);
      }
    };

    this.frameRateMonitor = requestAnimationFrame(measureFPS);
  }

  private startMemoryMonitoring(): void {
    const checkMemory = () => {
      const memInfo = (performance as any).memory;
      if (memInfo) {
        const metric: PerformanceMetric = {
          name: 'memory_usage',
          value: memInfo.usedJSHeapSize,
          unit: 'bytes',
          timestamp: Date.now(),
          category: 'memory',
          threshold: { 
            warning: this.budget.memoryUsage * 0.8, 
            critical: this.budget.memoryUsage 
          }
        };

        this.addMetric(metric);
      }
    };

    checkMemory(); // Initial check
    this.memoryMonitor = window.setInterval(checkMemory, 5000); // Every 5 seconds
  }

  private trackInitialLoad(): void {
    window.addEventListener('load', () => {
      // Track Web Assembly initialization time
      performance.mark('wasm-init-start');
      
      // This would be called after WASM is loaded
      document.addEventListener('wasm-loaded', () => {
        performance.mark('wasm-init-end');
        performance.measure('wasm-initialization', 'wasm-init-start', 'wasm-init-end');
      });
    });
  }

  private setupInteractionMonitoring(): void {
    const interactionEvents = ['click', 'keydown', 'touchstart'];
    
    interactionEvents.forEach(eventType => {
      document.addEventListener(eventType, (event) => {
        const startTime = performance.now();
        
        // Use requestAnimationFrame to measure response time
        requestAnimationFrame(() => {
          const endTime = performance.now();
          const latency = endTime - startTime;
          
          const metric: PerformanceMetric = {
            name: 'interaction_latency',
            value: latency,
            unit: 'ms',
            timestamp: Date.now(),
            category: 'interaction',
            threshold: { 
              warning: this.budget.interactionLatency * 0.8, 
              critical: this.budget.interactionLatency 
            }
          };

          this.addMetric(metric);
        });
      }, { passive: true });
    });
  }

  public addMetric(metric: PerformanceMetric): void {
    this.metrics.push(metric);
    
    // Check thresholds and emit warnings
    this.checkThreshold(metric);
    
    // Maintain reasonable buffer size
    if (this.metrics.length > 1000) {
      this.metrics = this.metrics.slice(-500);
    }
  }

  private checkThreshold(metric: PerformanceMetric): void {
    if (!metric.threshold) return;

    let level: 'warning' | 'critical' | null = null;
    
    if (metric.value >= metric.threshold.critical) {
      level = 'critical';
    } else if (metric.value >= metric.threshold.warning) {
      level = 'warning';
    }

    if (level) {
      console.warn(`Performance ${level}: ${metric.name} = ${metric.value}${metric.unit}`, metric);
      
      // Emit custom event for UI notifications
      document.dispatchEvent(new CustomEvent('performance-threshold-exceeded', {
        detail: { metric, level }
      }));
    }
  }

  public createSnapshot(): PerformanceSnapshot {
    const snapshot: PerformanceSnapshot = {
      timestamp: Date.now(),
      sessionId: this.sessionId,
      metrics: [...this.metrics],
      context: {
        url: window.location.href,
        viewport: {
          width: window.innerWidth,
          height: window.innerHeight
        },
        activeNodes: this.getActiveNodeCount(),
        connections: this.getConnectionCount()
      }
    };

    this.snapshots.push(snapshot);
    return snapshot;
  }

  private getActiveNodeCount(): number {
    // This would integrate with the Marco 2.0 canvas system
    const canvasElements = document.querySelectorAll('[data-node-id]');
    return canvasElements.length;
  }

  private getConnectionCount(): number {
    // This would integrate with the Marco 2.0 connection system
    const connectionElements = document.querySelectorAll('[data-connection-id]');
    return connectionElements.length;
  }

  public getMetrics(category?: PerformanceMetric['category']): PerformanceMetric[] {
    if (category) {
      return this.metrics.filter(m => m.category === category);
    }
    return [...this.metrics];
  }

  public getAverageMetric(name: string, timeWindow: number = 60000): number {
    const cutoff = Date.now() - timeWindow;
    const relevantMetrics = this.metrics.filter(m => 
      m.name === name && m.timestamp >= cutoff
    );

    if (relevantMetrics.length === 0) return 0;

    const sum = relevantMetrics.reduce((acc, m) => acc + m.value, 0);
    return sum / relevantMetrics.length;
  }

  public getPerformanceScore(): number {
    // Calculate overall performance score (0-100)
    const weights = {
      load: 0.3,
      render: 0.25,
      interaction: 0.25,
      memory: 0.15,
      network: 0.05
    };

    let totalScore = 0;
    let totalWeight = 0;

    Object.entries(weights).forEach(([category, weight]) => {
      const categoryMetrics = this.getMetrics(category as PerformanceMetric['category']);
      if (categoryMetrics.length > 0) {
        const categoryScore = this.calculateCategoryScore(categoryMetrics);
        totalScore += categoryScore * weight;
        totalWeight += weight;
      }
    });

    return totalWeight > 0 ? Math.round(totalScore / totalWeight) : 100;
  }

  private calculateCategoryScore(metrics: PerformanceMetric[]): number {
    // Score each metric based on how it compares to thresholds
    let totalScore = 0;
    let scoredMetrics = 0;

    metrics.forEach(metric => {
      if (metric.threshold) {
        let score = 100;
        if (metric.value >= metric.threshold.critical) {
          score = 0;
        } else if (metric.value >= metric.threshold.warning) {
          score = 50;
        }
        totalScore += score;
        scoredMetrics++;
      }
    });

    return scoredMetrics > 0 ? totalScore / scoredMetrics : 100;
  }

  public getBudgetCompliance(): Record<string, boolean> {
    const compliance: Record<string, boolean> = {};
    
    // Check load time
    const loadMetrics = this.getMetrics('load');
    const avgLoadTime = this.getAverageMetric('load_complete');
    compliance.loadTime = avgLoadTime <= this.budget.loadTime;

    // Check frame rate
    const avgFrameRate = this.getAverageMetric('frame_rate');
    compliance.frameRate = avgFrameRate >= this.budget.frameRate;

    // Check memory usage
    const avgMemoryUsage = this.getAverageMetric('memory_usage');
    compliance.memoryUsage = avgMemoryUsage <= this.budget.memoryUsage;

    // Check interaction latency
    const avgInteractionLatency = this.getAverageMetric('interaction_latency');
    compliance.interactionLatency = avgInteractionLatency <= this.budget.interactionLatency;

    return compliance;
  }

  public async submitSnapshot(snapshot?: PerformanceSnapshot): Promise<boolean> {
    const snapshotToSubmit = snapshot || this.createSnapshot();
    
    try {
      const response = await fetch(this.endpoint, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(snapshotToSubmit)
      });

      return response.ok;
    } catch (error) {
      console.error('Failed to submit performance snapshot:', error);
      return false;
    }
  }

  public startRealTimeReporting(interval: number = 30000): void {
    setInterval(() => {
      this.submitSnapshot();
    }, interval);
  }

  public destroy(): void {
    this.isMonitoring = false;
    
    // Cancel monitoring
    if (this.frameRateMonitor) {
      cancelAnimationFrame(this.frameRateMonitor);
    }
    
    if (this.memoryMonitor) {
      clearInterval(this.memoryMonitor);
    }

    // Disconnect observers
    this.observers.forEach(observer => observer.disconnect());
    this.observers.clear();

    // Clear data
    this.metrics = [];
    this.snapshots = [];
  }
}

// Global performance analytics instance
export const performanceAnalytics = new PerformanceAnalytics();
