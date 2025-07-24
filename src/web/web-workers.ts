/**
 * Marco 2.0 - Web Workers Manager
 * Background processing for analytics, collaboration, and performance monitoring
 */

export interface WorkerMessage {
  id: string;
  type: string;
  payload: any;
  timestamp: number;
}

export interface WorkerResponse {
  id: string;
  type: string;
  result?: any;
  error?: string;
  timestamp: number;
}

export interface WorkerConfig {
  maxWorkers: number;
  workerTimeout: number;
  retryAttempts: number;
  enableSharedMemory: boolean;
}

export class WebWorkersManager {
  private workers: Map<string, Worker> = new Map();
  private pendingTasks: Map<string, { resolve: Function; reject: Function; timeout: number }> = new Map();
  private config: WorkerConfig;
  private taskQueue: WorkerMessage[] = [];
  private isProcessingQueue: boolean = false;

  constructor(config?: Partial<WorkerConfig>) {
    this.config = {
      maxWorkers: navigator.hardwareConcurrency || 4,
      workerTimeout: 30000, // 30 seconds
      retryAttempts: 3,
      enableSharedMemory: typeof SharedArrayBuffer !== 'undefined',
      ...config
    };

    this.initializeWorkers();
  }

  private initializeWorkers(): void {
    // Analytics Worker
    this.createWorker('analytics', '/workers/analytics-worker.js', {
      enablePerformanceTracking: true,
      batchSize: 100,
      flushInterval: 30000
    });

    // Collaboration Worker
    this.createWorker('collaboration', '/workers/collaboration-worker.js', {
      enableWebRTC: true,
      maxPeers: 10,
      syncInterval: 1000
    });

    // Performance Monitor Worker
    this.createWorker('performance', '/workers/performance-worker.js', {
      monitorInterval: 5000,
      enableMemoryTracking: true,
      enableFrameRateTracking: true
    });

    // WASM Processing Worker
    if (this.config.enableSharedMemory) {
      this.createWorker('wasm', '/workers/wasm-worker.js', {
        enableSharedArrayBuffer: true,
        memoryLimit: 256 * 1024 * 1024 // 256MB
      });
    }
  }

  private createWorker(name: string, scriptPath: string, workerConfig: any): void {
    try {
      const worker = new Worker(scriptPath);
      
      // Setup message handling
      worker.onmessage = (event) => this.handleWorkerMessage(name, event.data);
      worker.onerror = (error) => this.handleWorkerError(name, error);
      
      // Send initial configuration
      worker.postMessage({
        type: 'INIT',
        config: workerConfig,
        timestamp: Date.now()
      });

      this.workers.set(name, worker);
      console.log(`Worker '${name}' initialized successfully`);
      
    } catch (error) {
      console.error(`Failed to create worker '${name}':`, error);
    }
  }

  private handleWorkerMessage(workerName: string, message: WorkerResponse): void {
    const { id, type, result, error } = message;
    
    // Handle task responses
    if (id && this.pendingTasks.has(id)) {
      const task = this.pendingTasks.get(id)!;
      clearTimeout(task.timeout);
      this.pendingTasks.delete(id);
      
      if (error) {
        task.reject(new Error(error));
      } else {
        task.resolve(result);
      }
      return;
    }

    // Handle worker-initiated messages
    switch (type) {
      case 'ANALYTICS_BATCH_READY':
        this.handleAnalyticsBatch(result);
        break;
        
      case 'PERFORMANCE_ALERT':
        this.handlePerformanceAlert(result);
        break;
        
      case 'COLLABORATION_UPDATE':
        this.handleCollaborationUpdate(result);
        break;
        
      case 'WASM_RESULT':
        this.handleWasmResult(result);
        break;
        
      default:
        console.log(`Unhandled message from ${workerName}:`, message);
    }
  }

  private handleWorkerError(workerName: string, error: ErrorEvent): void {
    console.error(`Worker '${workerName}' error:`, error);
    
    // Attempt to restart the worker
    const worker = this.workers.get(workerName);
    if (worker) {
      worker.terminate();
      this.workers.delete(workerName);
      
      // Restart after a delay
      setTimeout(() => {
        this.restartWorker(workerName);
      }, 1000);
    }
  }

  private restartWorker(workerName: string): void {
    const workerConfigs = {
      analytics: { scriptPath: '/workers/analytics-worker.js', config: { enablePerformanceTracking: true } },
      collaboration: { scriptPath: '/workers/collaboration-worker.js', config: { enableWebRTC: true } },
      performance: { scriptPath: '/workers/performance-worker.js', config: { monitorInterval: 5000 } },
      wasm: { scriptPath: '/workers/wasm-worker.js', config: { enableSharedArrayBuffer: true } }
    };

    const workerConfig = workerConfigs[workerName as keyof typeof workerConfigs];
    if (workerConfig) {
      this.createWorker(workerName, workerConfig.scriptPath, workerConfig.config);
    }
  }

  public async executeTask(
    workerName: string, 
    taskType: string, 
    payload: any,
    priority: 'low' | 'normal' | 'high' = 'normal'
  ): Promise<any> {
    const taskId = this.generateTaskId();
    const message: WorkerMessage = {
      id: taskId,
      type: taskType,
      payload,
      timestamp: Date.now()
    };

    // Add to queue if worker is busy or doesn't exist
    const worker = this.workers.get(workerName);
    if (!worker) {
      throw new Error(`Worker '${workerName}' not found`);
    }

    return new Promise((resolve, reject) => {
      // Set timeout for task completion
      const timeout = setTimeout(() => {
        this.pendingTasks.delete(taskId);
        reject(new Error(`Task ${taskId} timed out after ${this.config.workerTimeout}ms`));
      }, this.config.workerTimeout);

      // Store task resolver
      this.pendingTasks.set(taskId, { resolve, reject, timeout });

      // Send task to worker
      try {
        worker.postMessage(message);
      } catch (error) {
        clearTimeout(timeout);
        this.pendingTasks.delete(taskId);
        reject(error);
      }
    });
  }

  // Analytics Tasks
  public async processAnalyticsData(events: any[]): Promise<any> {
    return this.executeTask('analytics', 'PROCESS_EVENTS', { events });
  }

  public async generateAnalyticsReport(timeRange: { start: number; end: number }): Promise<any> {
    return this.executeTask('analytics', 'GENERATE_REPORT', { timeRange });
  }

  // Collaboration Tasks
  public async syncCollaborativeState(state: any, peers: string[]): Promise<any> {
    return this.executeTask('collaboration', 'SYNC_STATE', { state, peers });
  }

  public async processWebRTCSignal(signal: any, peerId: string): Promise<any> {
    return this.executeTask('collaboration', 'PROCESS_SIGNAL', { signal, peerId });
  }

  // Performance Tasks
  public async analyzePerformanceMetrics(metrics: any[]): Promise<any> {
    return this.executeTask('performance', 'ANALYZE_METRICS', { metrics });
  }

  public async detectPerformanceAnomalies(currentMetrics: any): Promise<any> {
    return this.executeTask('performance', 'DETECT_ANOMALIES', { currentMetrics });
  }

  // WASM Tasks
  public async executeWasmFunction(functionName: string, args: any[]): Promise<any> {
    if (!this.config.enableSharedMemory) {
      throw new Error('SharedArrayBuffer not available for WASM tasks');
    }
    return this.executeTask('wasm', 'EXECUTE_FUNCTION', { functionName, args });
  }

  public async processLargeDataset(data: any[], operation: string): Promise<any> {
    return this.executeTask('wasm', 'PROCESS_DATASET', { data, operation });
  }

  // Message Handlers
  private handleAnalyticsBatch(batch: any): void {
    // Send analytics batch to server
    fetch('/api/analytics/batch', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(batch)
    }).catch(error => console.error('Failed to send analytics batch:', error));
  }

  private handlePerformanceAlert(alert: any): void {
    console.warn('Performance Alert:', alert);
    
    // Trigger performance optimization if needed
    if (alert.severity === 'critical') {
      this.triggerPerformanceOptimization(alert);
    }
    
    // Send alert to monitoring system
    document.dispatchEvent(new CustomEvent('performance-alert', { detail: alert }));
  }

  private handleCollaborationUpdate(update: any): void {
    // Apply collaborative changes to the main thread
    document.dispatchEvent(new CustomEvent('collaboration-update', { detail: update }));
  }

  private handleWasmResult(result: any): void {
    // Handle WASM computation results
    document.dispatchEvent(new CustomEvent('wasm-result', { detail: result }));
  }

  private triggerPerformanceOptimization(alert: any): void {
    // Implement performance optimization strategies
    switch (alert.type) {
      case 'high_memory_usage':
        this.executeTask('performance', 'OPTIMIZE_MEMORY', {});
        break;
      case 'low_frame_rate':
        this.executeTask('performance', 'OPTIMIZE_RENDERING', {});
        break;
      case 'slow_response_time':
        this.executeTask('performance', 'OPTIMIZE_PROCESSING', {});
        break;
    }
  }

  private generateTaskId(): string {
    return `task_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
  }

  public getWorkerStatus(): Record<string, { active: boolean; pendingTasks: number }> {
    const status: Record<string, { active: boolean; pendingTasks: number }> = {};
    
    for (const [name, worker] of this.workers.entries()) {
      const pendingTasks = Array.from(this.pendingTasks.keys()).length;
      status[name] = {
        active: !!worker,
        pendingTasks
      };
    }
    
    return status;
  }

  public async terminateWorker(workerName: string): Promise<void> {
    const worker = this.workers.get(workerName);
    if (worker) {
      worker.terminate();
      this.workers.delete(workerName);
      
      // Cancel pending tasks for this worker
      for (const [taskId, task] of this.pendingTasks.entries()) {
        clearTimeout(task.timeout);
        task.reject(new Error(`Worker '${workerName}' terminated`));
      }
    }
  }

  public async terminateAllWorkers(): Promise<void> {
    const terminationPromises = Array.from(this.workers.keys())
      .map(workerName => this.terminateWorker(workerName));
    
    await Promise.all(terminationPromises);
    this.pendingTasks.clear();
  }

  public transferToWorker(workerName: string, data: ArrayBuffer): boolean {
    const worker = this.workers.get(workerName);
    if (!worker) return false;

    try {
      worker.postMessage({ type: 'TRANSFER_DATA', data }, [data]);
      return true;
    } catch (error) {
      console.error(`Failed to transfer data to worker '${workerName}':`, error);
      return false;
    }
  }
}

// Shared Memory Manager for cross-worker communication
export class SharedMemoryManager {
  private sharedBuffers: Map<string, SharedArrayBuffer> = new Map();
  private isSupported: boolean = typeof SharedArrayBuffer !== 'undefined';

  constructor() {
    if (!this.isSupported) {
      console.warn('SharedArrayBuffer not supported - falling back to message passing');
    }
  }

  public createSharedBuffer(name: string, size: number): SharedArrayBuffer | null {
    if (!this.isSupported) return null;

    try {
      const buffer = new SharedArrayBuffer(size);
      this.sharedBuffers.set(name, buffer);
      return buffer;
    } catch (error) {
      console.error(`Failed to create shared buffer '${name}':`, error);
      return null;
    }
  }

  public getSharedBuffer(name: string): SharedArrayBuffer | null {
    return this.sharedBuffers.get(name) || null;
  }

  public removeSharedBuffer(name: string): boolean {
    return this.sharedBuffers.delete(name);
  }

  public createSharedArray(name: string, length: number, type: 'int32' | 'float32' | 'float64' = 'float32'): any | null {
    const buffer = this.createSharedBuffer(name, length * this.getTypeSize(type));
    if (!buffer) return null;

    switch (type) {
      case 'int32':
        return new Int32Array(buffer);
      case 'float32':
        return new Float32Array(buffer);
      case 'float64':
        return new Float64Array(buffer);
      default:
        return new Float32Array(buffer);
    }
  }

  private getTypeSize(type: string): number {
    switch (type) {
      case 'int32': return 4;
      case 'float32': return 4;
      case 'float64': return 8;
      default: return 4;
    }
  }
}

// Global instances
export const webWorkersManager = new WebWorkersManager();
export const sharedMemoryManager = new SharedMemoryManager();

// Initialize background processing on page load
if (typeof document !== 'undefined') {
  document.addEventListener('DOMContentLoaded', () => {
    console.log('Web Workers Manager initialized');
    
    // Start background analytics processing
    webWorkersManager.executeTask('analytics', 'START_BACKGROUND_PROCESSING', {})
      .catch(error => console.error('Failed to start background analytics:', error));
  });
}
