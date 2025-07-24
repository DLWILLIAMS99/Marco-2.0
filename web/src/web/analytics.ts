/**
 * Production Analytics & Crash Reporting for Marco 2.0
 * 
 * Real-world performance analytics, user behavior tracking, and error reporting
 */

export interface AnalyticsEvent {
  type: 'page_view' | 'user_action' | 'performance' | 'error' | 'feature_usage';
  category: string;
  action: string;
  label?: string;
  value?: number;
  timestamp: number;
  userId?: string;
  sessionId: string;
  metadata?: Record<string, any>;
}

export interface PerformanceMetrics {
  loadTime: number;
  renderTime: number;
  memoryUsage: number;
  fps: number;
  wasmInitTime: number;
  cacheHitRatio: number;
  networkLatency?: number;
  deviceInfo: DeviceInfo;
}

export interface DeviceInfo {
  userAgent: string;
  platform: string;
  cores: number;
  memory: number;
  screen: { width: number; height: number };
  pixelRatio: number;
  webglSupport: boolean;
  webRtcSupport: boolean;
  speechSupport: boolean;
}

export interface CrashReport {
  id: string;
  timestamp: number;
  error: {
    message: string;
    stack: string;
    type: string;
    fileName?: string;
    lineNumber?: number;
    columnNumber?: number;
  };
  context: {
    url: string;
    userAgent: string;
    userId?: string;
    sessionId: string;
    buildVersion: string;
    featureFlags: Record<string, boolean>;
  };
  performance: Partial<PerformanceMetrics>;
  breadcrumbs: BreadcrumbEntry[];
}

export interface BreadcrumbEntry {
  timestamp: number;
  category: 'navigation' | 'ui' | 'http' | 'console' | 'user' | 'performance';
  message: string;
  level: 'info' | 'warning' | 'error';
  data?: Record<string, any>;
}

export interface AnalyticsConfig {
  apiEndpoint: string;
  apiKey: string;
  enableCrashReporting: boolean;
  enablePerformanceTracking: boolean;
  enableUserTracking: boolean;
  enableDebugMode: boolean;
  batchSize: number;
  flushInterval: number;
  maxBreadcrumbs: number;
  sampleRate: number;
}

export class ProductionAnalytics {
  private config: AnalyticsConfig;
  private sessionId: string;
  private userId?: string;
  private eventQueue: AnalyticsEvent[] = [];
  private breadcrumbs: BreadcrumbEntry[] = [];
  private performanceObserver?: PerformanceObserver;
  private isInitialized = false;
  private flushTimer?: number;
  private deviceInfo: DeviceInfo;

  private readonly BUILD_VERSION = '2.0.0-sprint3';
  private readonly DEFAULT_CONFIG: Partial<AnalyticsConfig> = {
    enableCrashReporting: true,
    enablePerformanceTracking: true,
    enableUserTracking: true,
    enableDebugMode: false,
    batchSize: 10,
    flushInterval: 30000, // 30 seconds
    maxBreadcrumbs: 50,
    sampleRate: 1.0, // 100% sampling in production
  };

  constructor(config: Partial<AnalyticsConfig> & { apiEndpoint: string; apiKey: string }) {
    this.config = { ...this.DEFAULT_CONFIG, ...config } as AnalyticsConfig;
    this.sessionId = this.generateSessionId();
    this.deviceInfo = this.collectDeviceInfo();
    
    this.initialize();
  }

  /**
   * Initialize analytics system
   */
  private async initialize(): Promise<void> {
    try {
      // Set up error handlers
      this.setupErrorHandlers();
      
      // Set up performance monitoring
      if (this.config.enablePerformanceTracking) {
        this.setupPerformanceMonitoring();
      }
      
      // Set up automatic event tracking
      this.setupAutomaticTracking();
      
      // Start flush timer
      this.startFlushTimer();
      
      // Track initialization
      this.trackEvent({
        type: 'page_view',
        category: 'app',
        action: 'initialize',
        label: 'analytics_system',
        metadata: {
          buildVersion: this.BUILD_VERSION,
          deviceInfo: this.deviceInfo,
          config: this.sanitizeConfig(),
        },
      });
      
      this.isInitialized = true;
      console.log('Production analytics initialized');
      
    } catch (error) {
      console.error('Failed to initialize analytics:', error);
    }
  }

  /**
   * Track custom event
   */
  public trackEvent(event: Omit<AnalyticsEvent, 'timestamp' | 'sessionId'>): void {
    if (!this.shouldSample()) return;

    const fullEvent: AnalyticsEvent = {
      ...event,
      timestamp: Date.now(),
      sessionId: this.sessionId,
      userId: this.userId,
    };

    this.eventQueue.push(fullEvent);
    this.addBreadcrumb('user', `Event: ${event.category}.${event.action}`, 'info', event.metadata);

    if (this.config.enableDebugMode) {
      console.log('Analytics event:', fullEvent);
    }

    // Flush immediately for critical events
    if (event.type === 'error' || this.eventQueue.length >= this.config.batchSize) {
      this.flush();
    }
  }

  /**
   * Track performance metrics
   */
  public trackPerformance(metrics: Partial<PerformanceMetrics>): void {
    if (!this.config.enablePerformanceTracking || !this.shouldSample()) return;

    this.trackEvent({
      type: 'performance',
      category: 'performance',
      action: 'metrics',
      metadata: {
        ...metrics,
        deviceInfo: this.deviceInfo,
      },
    });
  }

  /**
   * Track user action
   */
  public trackUserAction(action: string, category: string = 'user', label?: string, value?: number): void {
    this.trackEvent({
      type: 'user_action',
      category,
      action,
      label,
      value,
    });
  }

  /**
   * Track feature usage
   */
  public trackFeatureUsage(feature: string, action: string, metadata?: Record<string, any>): void {
    this.trackEvent({
      type: 'feature_usage',
      category: 'features',
      action: `${feature}.${action}`,
      metadata,
    });
  }

  /**
   * Set user ID for tracking
   */
  public setUserId(userId: string): void {
    this.userId = userId;
    this.trackEvent({
      type: 'user_action',
      category: 'user',
      action: 'identify',
      label: userId,
    });
  }

  /**
   * Add breadcrumb for debugging
   */
  public addBreadcrumb(category: BreadcrumbEntry['category'], message: string, level: BreadcrumbEntry['level'] = 'info', data?: Record<string, any>): void {
    const breadcrumb: BreadcrumbEntry = {
      timestamp: Date.now(),
      category,
      message,
      level,
      data,
    };

    this.breadcrumbs.push(breadcrumb);

    // Limit breadcrumb history
    if (this.breadcrumbs.length > this.config.maxBreadcrumbs) {
      this.breadcrumbs.shift();
    }
  }

  /**
   * Report crash/error
   */
  public reportCrash(error: Error, context?: Record<string, any>): void {
    if (!this.config.enableCrashReporting) return;

    const crashReport: CrashReport = {
      id: this.generateCrashId(),
      timestamp: Date.now(),
      error: {
        message: error.message,
        stack: error.stack || '',
        type: error.constructor.name,
        fileName: (error as any).fileName,
        lineNumber: (error as any).lineNumber,
        columnNumber: (error as any).columnNumber,
      },
      context: {
        url: window.location.href,
        userAgent: navigator.userAgent,
        userId: this.userId,
        sessionId: this.sessionId,
        buildVersion: this.BUILD_VERSION,
        featureFlags: this.getFeatureFlags(),
        ...context,
      },
      performance: this.getCurrentPerformanceMetrics(),
      breadcrumbs: [...this.breadcrumbs],
    };

    // Send crash report immediately
    this.sendCrashReport(crashReport);

    // Also track as analytics event
    this.trackEvent({
      type: 'error',
      category: 'crash',
      action: 'report',
      label: error.message,
      metadata: {
        errorType: error.constructor.name,
        stack: error.stack,
        context,
      },
    });
  }

  /**
   * Flush queued events
   */
  public async flush(): Promise<void> {
    if (this.eventQueue.length === 0) return;

    const events = [...this.eventQueue];
    this.eventQueue = [];

    try {
      await this.sendEvents(events);
      
      if (this.config.enableDebugMode) {
        console.log(`Flushed ${events.length} analytics events`);
      }
    } catch (error) {
      console.error('Failed to flush analytics events:', error);
      
      // Re-queue events on failure (with limit)
      if (this.eventQueue.length < this.config.batchSize * 2) {
        this.eventQueue.unshift(...events);
      }
    }
  }

  /**
   * Get current session metrics
   */
  public getSessionMetrics(): Record<string, any> {
    return {
      sessionId: this.sessionId,
      userId: this.userId,
      sessionDuration: Date.now() - this.getSessionStartTime(),
      eventsTracked: this.eventQueue.length,
      breadcrumbsCollected: this.breadcrumbs.length,
      deviceInfo: this.deviceInfo,
    };
  }

  /**
   * Private methods
   */

  private setupErrorHandlers(): void {
    // Global error handler
    window.addEventListener('error', (event) => {
      this.reportCrash(event.error || new Error(event.message), {
        filename: event.filename,
        lineno: event.lineno,
        colno: event.colno,
      });
    });

    // Unhandled promise rejection handler
    window.addEventListener('unhandledrejection', (event) => {
      this.reportCrash(new Error(`Unhandled promise rejection: ${event.reason}`), {
        reason: event.reason,
      });
    });

    // Console error interceptor
    const originalError = console.error;
    console.error = (...args) => {
      this.addBreadcrumb('console', `Console error: ${args.join(' ')}`, 'error');
      originalError.apply(console, args);
    };
  }

  private setupPerformanceMonitoring(): void {
    if ('PerformanceObserver' in window) {
      try {
        this.performanceObserver = new PerformanceObserver((list) => {
          const entries = list.getEntries();
          
          entries.forEach(entry => {
            this.addBreadcrumb('performance', `${entry.entryType}: ${entry.name}`, 'info', {
              duration: entry.duration,
              startTime: entry.startTime,
            });

            // Track significant performance events
            if (entry.entryType === 'navigation') {
              const navEntry = entry as PerformanceNavigationTiming;
              this.trackPerformance({
                loadTime: navEntry.loadEventEnd - navEntry.loadEventStart,
                renderTime: navEntry.domContentLoadedEventEnd - navEntry.domContentLoadedEventStart,
              });
            }
          });
        });

        this.performanceObserver.observe({ entryTypes: ['navigation', 'measure', 'mark'] });
      } catch (error) {
        console.warn('Performance monitoring not available:', error);
      }
    }

    // Track memory usage periodically
    if ((performance as any).memory) {
      setInterval(() => {
        const memory = (performance as any).memory;
        this.trackPerformance({
          memoryUsage: memory.usedJSHeapSize / 1024 / 1024, // MB
        });
      }, 60000); // Every minute
    }
  }

  private setupAutomaticTracking(): void {
    // Track page visibility changes
    document.addEventListener('visibilitychange', () => {
      this.trackUserAction(
        document.hidden ? 'page_hidden' : 'page_visible',
        'navigation'
      );
    });

    // Track user interactions
    ['click', 'touchstart', 'keydown'].forEach(eventType => {
      document.addEventListener(eventType, (event) => {
        const target = event.target as HTMLElement;
        const tagName = target.tagName.toLowerCase();
        
        if (['button', 'a', 'input'].includes(tagName)) {
          this.addBreadcrumb('ui', `${eventType} on ${tagName}`, 'info', {
            elementId: target.id,
            className: target.className,
          });
        }
      });
    });

    // Track network status
    window.addEventListener('online', () => {
      this.trackUserAction('network_online', 'system');
    });

    window.addEventListener('offline', () => {
      this.trackUserAction('network_offline', 'system');
    });
  }

  private startFlushTimer(): void {
    this.flushTimer = window.setInterval(() => {
      this.flush();
    }, this.config.flushInterval);
  }

  private collectDeviceInfo(): DeviceInfo {
    const screen = window.screen;
    const navigator = window.navigator;

    return {
      userAgent: navigator.userAgent,
      platform: navigator.platform,
      cores: navigator.hardwareConcurrency || 1,
      memory: (navigator as any).deviceMemory || 0,
      screen: {
        width: screen.width,
        height: screen.height,
      },
      pixelRatio: window.devicePixelRatio || 1,
      webglSupport: this.checkWebGLSupport(),
      webRtcSupport: this.checkWebRTCSupport(),
      speechSupport: this.checkSpeechSupport(),
    };
  }

  private checkWebGLSupport(): boolean {
    try {
      const canvas = document.createElement('canvas');
      return !!(canvas.getContext('webgl') || canvas.getContext('experimental-webgl'));
    } catch {
      return false;
    }
  }

  private checkWebRTCSupport(): boolean {
    return !!(window.RTCPeerConnection || (window as any).webkitRTCPeerConnection);
  }

  private checkSpeechSupport(): boolean {
    return !!(window.SpeechRecognition || (window as any).webkitSpeechRecognition);
  }

  private shouldSample(): boolean {
    return Math.random() < this.config.sampleRate;
  }

  private sanitizeConfig(): Partial<AnalyticsConfig> {
    const { apiKey, ...sanitized } = this.config;
    return sanitized;
  }

  private getFeatureFlags(): Record<string, boolean> {
    // In a real app, this would come from a feature flag service
    return {
      collaboration: true,
      voiceCommands: true,
      advancedGestures: true,
      offlineMode: true,
      performanceOptimization: true,
    };
  }

  private getCurrentPerformanceMetrics(): Partial<PerformanceMetrics> {
    const memory = (performance as any).memory;
    
    return {
      memoryUsage: memory ? memory.usedJSHeapSize / 1024 / 1024 : 0,
      deviceInfo: this.deviceInfo,
    };
  }

  private getSessionStartTime(): number {
    return parseInt(this.sessionId.split('_')[1]) || Date.now();
  }

  private async sendEvents(events: AnalyticsEvent[]): Promise<void> {
    const payload = {
      events,
      sessionInfo: this.getSessionMetrics(),
      timestamp: Date.now(),
    };

    const response = await fetch(`${this.config.apiEndpoint}/events`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${this.config.apiKey}`,
        'X-Marco2-Version': this.BUILD_VERSION,
      },
      body: JSON.stringify(payload),
    });

    if (!response.ok) {
      throw new Error(`Analytics API error: ${response.status} ${response.statusText}`);
    }
  }

  private async sendCrashReport(report: CrashReport): Promise<void> {
    try {
      const response = await fetch(`${this.config.apiEndpoint}/crashes`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${this.config.apiKey}`,
          'X-Marco2-Version': this.BUILD_VERSION,
        },
        body: JSON.stringify(report),
      });

      if (!response.ok) {
        console.error(`Failed to send crash report: ${response.status}`);
      }
    } catch (error) {
      console.error('Error sending crash report:', error);
    }
  }

  private generateSessionId(): string {
    return `session_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
  }

  private generateCrashId(): string {
    return `crash_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
  }

  /**
   * Cleanup on app destroy
   */
  public destroy(): void {
    if (this.flushTimer) {
      clearInterval(this.flushTimer);
    }

    if (this.performanceObserver) {
      this.performanceObserver.disconnect();
    }

    // Final flush
    this.flush();

    console.log('Production analytics destroyed');
  }
}

export default ProductionAnalytics;
