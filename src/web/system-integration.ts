/**
 * Marco 2.0 - System Integration Manager
 * Orchestrates all production components: analytics, security, workers, WASM
 */

import { analytics } from '../analytics/analytics';
import { crashReporter } from '../analytics/crash-reporter';
import { performanceAnalytics } from '../analytics/performance-analytics';
import { webWorkersManager } from './web-workers';
import { wasmOptimizer } from './wasm-optimization';
import { marco2CSP, wasmSecurity, webrtcSecurity } from '../security/csp';
import { securityHeaders, corsPolicy, securityMonitor } from '../security/headers';
import { authManager, webrtcPeerAuth, permissionManager } from '../security/auth';

export interface SystemConfig {
  enableAnalytics: boolean;
  enableCrashReporting: boolean;
  enablePerformanceMonitoring: boolean;
  enableWebWorkers: boolean;
  enableWasmOptimization: boolean;
  enableSecurity: boolean;
  enableAuthentication: boolean;
  productionMode: boolean;
}

export interface SystemStatus {
  initialized: boolean;
  componentsLoaded: number;
  totalComponents: number;
  errors: string[];
  performance: {
    initTime: number;
    memoryUsage: number;
    activeWorkers: number;
    loadedWasmModules: number;
  };
}

export class SystemIntegrationManager {
  private config: SystemConfig;
  private status: SystemStatus;
  private initStartTime: number = 0;
  private componentsInitialized: Set<string> = new Set();
  private isShuttingDown: boolean = false;

  constructor(config?: Partial<SystemConfig>) {
    this.config = {
      enableAnalytics: true,
      enableCrashReporting: true,
      enablePerformanceMonitoring: true,
      enableWebWorkers: true,
      enableWasmOptimization: true,
      enableSecurity: true,
      enableAuthentication: true,
      productionMode: typeof window !== 'undefined' ? window.location.hostname !== 'localhost' : false,
      ...config
    };

    this.status = {
      initialized: false,
      componentsLoaded: 0,
      totalComponents: this.getTotalComponents(),
      errors: [],
      performance: {
        initTime: 0,
        memoryUsage: 0,
        activeWorkers: 0,
        loadedWasmModules: 0
      }
    };

    this.initialize();
  }

  private getTotalComponents(): number {
    return Object.values(this.config).filter(enabled => enabled === true).length;
  }

  private async initialize(): Promise<void> {
    this.initStartTime = performance.now();
    console.log('🚀 Marco 2.0 System Integration starting...');

    try {
      // Initialize components in dependency order
      await this.initializeCore();
      await this.initializeSecurity();
      await this.initializeAnalytics();
      await this.initializePerformance();
      await this.initializeWorkers();
      await this.initializeWasm();
      await this.initializeAuthentication();
      
      // Final system setup
      await this.setupIntegrations();
      await this.setupMonitoring();
      
      this.finalizeInitialization();
      
    } catch (error) {
      this.handleInitializationError(error);
    }
  }

  private async initializeCore(): Promise<void> {
    console.log('📦 Initializing core systems...');
    
    // Setup error handling
    this.setupGlobalErrorHandlers();
    
    // Initialize CSP and security
    if (this.config.enableSecurity) {
      marco2CSP.enableReporting();
      wasmSecurity.configureForWASM();
      webrtcSecurity.configureForWebRTC();
      this.markComponentInitialized('security-core');
    }
    
    this.markComponentInitialized('core');
  }

  private async initializeSecurity(): Promise<void> {
    if (!this.config.enableSecurity) return;
    
    console.log('🔒 Initializing security systems...');
    
    // Apply security headers
    this.applySecurityHeaders();
    
    // Setup CORS
    this.setupCORS();
    
    // Initialize security monitoring
    this.setupSecurityMonitoring();
    
    this.markComponentInitialized('security');
  }

  private async initializeAnalytics(): Promise<void> {
    if (!this.config.enableAnalytics) return;
    
    console.log('📊 Initializing analytics systems...');
    
    // Start analytics engine
    analytics.enable();
    
    // Track system initialization
    analytics.trackEvent('system_init_start', 'system', {
      productionMode: this.config.productionMode,
      timestamp: Date.now()
    });
    
    this.markComponentInitialized('analytics');
  }

  private async initializePerformance(): Promise<void> {
    if (!this.config.enablePerformanceMonitoring) return;
    
    console.log('⚡ Initializing performance monitoring...');
    
    // Start real-time performance monitoring
    performanceAnalytics.startRealTimeReporting(30000); // Every 30 seconds
    
    // Setup performance budget alerts
    this.setupPerformanceAlerts();
    
    this.markComponentInitialized('performance');
  }

  private async initializeWorkers(): Promise<void> {
    if (!this.config.enableWebWorkers) return;
    
    console.log('👷 Initializing web workers...');
    
    // Workers are initialized automatically in WebWorkersManager constructor
    // Just verify they're running
    const workerStatus = webWorkersManager.getWorkerStatus();
    const activeWorkers = Object.values(workerStatus).filter(s => s.active).length;
    
    if (activeWorkers > 0) {
      console.log(`✅ ${activeWorkers} web workers active`);
      this.markComponentInitialized('workers');
    } else {
      throw new Error('Failed to initialize web workers');
    }
  }

  private async initializeWasm(): Promise<void> {
    if (!this.config.enableWasmOptimization) return;
    
    console.log('🔧 Initializing WASM optimization...');
    
    // WASM optimizer initializes automatically
    // Wait for preloaded modules
    await this.waitForWasmModules();
    
    this.markComponentInitialized('wasm');
  }

  private async initializeAuthentication(): Promise<void> {
    if (!this.config.enableAuthentication) return;
    
    console.log('🔐 Initializing authentication...');
    
    // Setup session cleanup
    setInterval(() => {
      authManager.cleanupExpiredSessions();
      webrtcPeerAuth.cleanupExpiredPeerTokens();
    }, 60000); // Every minute
    
    this.markComponentInitialized('authentication');
  }

  private async waitForWasmModules(): Promise<void> {
    // Wait up to 10 seconds for WASM modules to load
    const timeout = 10000;
    const startTime = Date.now();
    
    while (Date.now() - startTime < timeout) {
      const moduleInfo = wasmOptimizer.getModuleInfo();
      const loadedModules = Object.values(moduleInfo).filter(m => m.loaded).length;
      
      if (loadedModules > 0) {
        console.log(`✅ ${loadedModules} WASM modules loaded`);
        return;
      }
      
      await new Promise(resolve => setTimeout(resolve, 100));
    }
    
    console.warn('⚠️ WASM modules took longer than expected to load');
  }

  private async setupIntegrations(): Promise<void> {
    console.log('🔗 Setting up system integrations...');
    
    // Connect analytics to performance monitoring
    document.addEventListener('performance-threshold-exceeded', (event: any) => {
      analytics.trackEvent('performance_threshold_exceeded', 'performance', event.detail);
    });
    
    // Connect crash reporter to analytics
    document.addEventListener('crash-detected', (event: any) => {
      analytics.trackEvent('crash_detected', 'error', {
        crashId: event.detail.report.id,
        type: event.detail.report.type,
        severity: event.detail.report.severity
      });
    });
    
    // Connect workers to analytics
    document.addEventListener('wasm-result', (event: any) => {
      analytics.trackEvent('wasm_computation_complete', 'performance', event.detail);
    });
    
    // Connect collaboration updates
    document.addEventListener('collaboration-update', (event: any) => {
      analytics.trackEvent('collaboration_update', 'user_action', event.detail);
    });
  }

  private async setupMonitoring(): Promise<void> {
    console.log('📡 Setting up system monitoring...');
    
    // Monitor system health every 30 seconds
    setInterval(() => {
      this.updateSystemStatus();
      this.checkSystemHealth();
    }, 30000);
    
    // Monitor memory usage
    setInterval(() => {
      if (typeof (performance as any).memory !== 'undefined') {
        const memInfo = (performance as any).memory;
        this.status.performance.memoryUsage = memInfo.usedJSHeapSize;
        
        // Alert on high memory usage
        if (memInfo.usedJSHeapSize > 100 * 1024 * 1024) { // 100MB
          this.handleHighMemoryUsage(memInfo);
        }
      }
    }, 10000);
  }

  private setupGlobalErrorHandlers(): void {
    // Enhanced error handling that integrates with our crash reporter
    window.addEventListener('error', (event) => {
      if (this.config.enableCrashReporting) {
        crashReporter.reportCustomError({
          type: 'javascript_error',
          message: event.message,
          stack: event.error?.stack,
          context: {
            url: event.filename || window.location.href
          }
        });
      }
      securityMonitor.recordViolation('javascript_error');
    });
    
    window.addEventListener('unhandledrejection', (event) => {
      if (this.config.enableCrashReporting) {
        crashReporter.reportCustomError({
          type: 'unhandled_rejection',
          message: event.reason?.message || 'Unhandled promise rejection',
          stack: event.reason?.stack
        });
      }
      securityMonitor.recordViolation('promise_rejection');
    });
  }

  private applySecurityHeaders(): void {
    // Apply security headers via meta tags (for client-side)
    const headers = securityHeaders.getHeaders();
    const metaTags = securityHeaders.generateMetaTags();
    
    metaTags.forEach(tagHTML => {
      const parser = new DOMParser();
      const doc = parser.parseFromString(tagHTML, 'text/html');
      const metaTag = doc.querySelector('meta');
      if (metaTag) {
        document.head.appendChild(metaTag);
      }
    });
  }

  private setupCORS(): void {
    // CORS is handled server-side, but we can validate origins client-side
    const allowedOrigins = ['https://marco2.com', 'https://www.marco2.com'];
    
    if (this.config.productionMode && !allowedOrigins.includes(window.location.origin)) {
      console.warn('⚠️ Running on non-whitelisted origin:', window.location.origin);
    }
  }

  private setupSecurityMonitoring(): void {
    // Monitor for security violations
    setInterval(() => {
      const violations = securityMonitor.getViolationStats();
      
      Object.entries(violations).forEach(([type, count]) => {
        if (count > 0) {
          analytics.trackEvent('security_violation', 'error', { type, count });
        }
      });
    }, 60000); // Every minute
  }

  private setupPerformanceAlerts(): void {
    document.addEventListener('performance-alert', (event: any) => {
      const alert = event.detail;
      
      console.warn('Performance Alert:', alert);
      
      // Take corrective action based on alert type
      switch (alert.type) {
        case 'high_memory_usage':
          this.optimizeMemoryUsage();
          break;
        case 'low_frame_rate':
          this.optimizeRendering();
          break;
        case 'slow_response_time':
          this.optimizeProcessing();
          break;
      }
    });
  }

  private async optimizeMemoryUsage(): Promise<void> {
    console.log('🧹 Optimizing memory usage...');
    
    // Trigger garbage collection in workers
    if (this.config.enableWebWorkers) {
      try {
        await webWorkersManager.executeTask('performance', 'OPTIMIZE_MEMORY', {});
      } catch (error) {
        console.error('Failed to optimize memory in workers:', error);
      }
    }
    
    // Clear old analytics data
    if (this.config.enableAnalytics) {
      analytics.clearData();
    }
  }

  private async optimizeRendering(): Promise<void> {
    console.log('🎨 Optimizing rendering performance...');
    
    if (this.config.enableWebWorkers) {
      try {
        await webWorkersManager.executeTask('performance', 'OPTIMIZE_RENDERING', {});
      } catch (error) {
        console.error('Failed to optimize rendering:', error);
      }
    }
  }

  private async optimizeProcessing(): Promise<void> {
    console.log('⚡ Optimizing processing performance...');
    
    if (this.config.enableWebWorkers) {
      try {
        await webWorkersManager.executeTask('performance', 'OPTIMIZE_PROCESSING', {});
      } catch (error) {
        console.error('Failed to optimize processing:', error);
      }
    }
  }

  private handleHighMemoryUsage(memInfo: any): void {
    console.warn('⚠️ High memory usage detected:', {
      used: `${Math.round(memInfo.usedJSHeapSize / 1024 / 1024)}MB`,
      total: `${Math.round(memInfo.totalJSHeapSize / 1024 / 1024)}MB`,
      limit: `${Math.round(memInfo.jsHeapSizeLimit / 1024 / 1024)}MB`
    });
    
    // Trigger memory optimization
    this.optimizeMemoryUsage();
  }

  private markComponentInitialized(component: string): void {
    this.componentsInitialized.add(component);
    this.status.componentsLoaded = this.componentsInitialized.size;
    
    console.log(`✅ ${component} initialized (${this.status.componentsLoaded}/${this.status.totalComponents})`);
  }

  private updateSystemStatus(): void {
    // Update performance metrics
    const workerStatus = webWorkersManager.getWorkerStatus();
    this.status.performance.activeWorkers = Object.values(workerStatus).filter(s => s.active).length;
    
    const wasmModules = wasmOptimizer.getModuleInfo();
    this.status.performance.loadedWasmModules = Object.values(wasmModules).filter(m => m.loaded).length;
    
    if (typeof (performance as any).memory !== 'undefined') {
      this.status.performance.memoryUsage = (performance as any).memory.usedJSHeapSize;
    }
  }

  private checkSystemHealth(): void {
    const issues: string[] = [];
    
    // Check worker health
    const workerStatus = webWorkersManager.getWorkerStatus();
    const inactiveWorkers = Object.entries(workerStatus).filter(([_, status]) => !status.active);
    
    if (inactiveWorkers.length > 0) {
      issues.push(`${inactiveWorkers.length} workers inactive`);
    }
    
    // Check memory usage
    if (this.status.performance.memoryUsage > 200 * 1024 * 1024) { // 200MB
      issues.push('High memory usage');
    }
    
    // Check WASM modules
    if (this.config.enableWasmOptimization && this.status.performance.loadedWasmModules === 0) {
      issues.push('No WASM modules loaded');
    }
    
    if (issues.length > 0) {
      console.warn('⚠️ System health issues detected:', issues);
      
      // Track health issues
      if (this.config.enableAnalytics) {
        analytics.trackEvent('system_health_issues', 'system', { issues });
      }
    }
  }

  private finalizeInitialization(): void {
    this.status.performance.initTime = performance.now() - this.initStartTime;
    this.status.initialized = true;
    
    console.log(`🎉 Marco 2.0 System Integration complete! (${this.status.performance.initTime.toFixed(2)}ms)`);
    console.log(`📊 Components loaded: ${this.status.componentsLoaded}/${this.status.totalComponents}`);
    
    // Track successful initialization
    if (this.config.enableAnalytics) {
      analytics.trackEvent('system_init_complete', 'system', {
        initTime: this.status.performance.initTime,
        componentsLoaded: this.status.componentsLoaded,
        productionMode: this.config.productionMode
      });
    }
    
    // Emit system ready event
    document.dispatchEvent(new CustomEvent('marco2-system-ready', {
      detail: { status: this.status, config: this.config }
    }));
  }

  private handleInitializationError(error: any): void {
    const errorMessage = error instanceof Error ? error.message : String(error);
    this.status.errors.push(errorMessage);
    
    console.error('❌ System initialization failed:', error);
    
    // Track initialization failure
    if (this.config.enableAnalytics) {
      analytics.trackEvent('system_init_failed', 'error', {
        error: errorMessage,
        componentsLoaded: this.status.componentsLoaded,
        totalComponents: this.status.totalComponents
      });
    }
    
    // Emit error event
    document.dispatchEvent(new CustomEvent('marco2-system-error', {
      detail: { error: errorMessage, status: this.status }
    }));
  }

  // Public API
  public getStatus(): SystemStatus {
    this.updateSystemStatus();
    return { ...this.status };
  }

  public getConfig(): SystemConfig {
    return { ...this.config };
  }

  public async restartComponent(componentName: string): Promise<boolean> {
    console.log(`🔄 Restarting component: ${componentName}`);
    
    try {
      switch (componentName) {
        case 'workers':
          // Restart specific workers
          await webWorkersManager.terminateAllWorkers();
          // Workers will auto-restart
          break;
        case 'analytics':
          analytics.disable();
          analytics.enable();
          break;
        case 'wasm':
          wasmOptimizer.destroy();
          // Would need to reinitialize
          break;
        default:
          throw new Error(`Unknown component: ${componentName}`);
      }
      
      return true;
    } catch (error) {
      console.error(`Failed to restart ${componentName}:`, error);
      return false;
    }
  }

  public async shutdown(): Promise<void> {
    if (this.isShuttingDown) return;
    
    this.isShuttingDown = true;
    console.log('🛑 Shutting down Marco 2.0 systems...');
    
    // Track shutdown
    if (this.config.enableAnalytics) {
      analytics.trackEvent('system_shutdown', 'system', {
        uptime: Date.now() - (this.initStartTime || 0)
      });
      
      // Flush analytics data
      await analytics.flush(true);
    }
    
    // Shutdown components
    if (this.config.enableWebWorkers) {
      await webWorkersManager.terminateAllWorkers();
    }
    
    if (this.config.enableWasmOptimization) {
      wasmOptimizer.destroy();
    }
    
    if (this.config.enableCrashReporting) {
      crashReporter.destroy();
    }
    
    console.log('✅ Marco 2.0 shutdown complete');
  }
}

// Global system integration manager
export const systemManager = new SystemIntegrationManager();

// Handle page unload
if (typeof window !== 'undefined') {
  window.addEventListener('beforeunload', () => {
    systemManager.shutdown();
  });
}
