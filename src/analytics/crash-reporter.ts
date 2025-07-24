/**
 * Marco 2.0 - Crash Reporter
 * Automated crash detection and reporting system
 */

export interface CrashReport {
  id: string;
  timestamp: number;
  sessionId: string;
  userId?: string;
  type: 'javascript_error' | 'unhandled_rejection' | 'wasm_panic' | 'webrtc_error' | 'render_error';
  severity: 'low' | 'medium' | 'high' | 'critical';
  message: string;
  stack?: string;
  context: {
    url: string;
    userAgent: string;
    viewport: { width: number; height: number };
    scope?: string;
    nodeId?: string;
    lastActions: string[];
  };
  systemInfo: {
    memoryUsage: number;
    connectionType?: string;
    batteryLevel?: number;
    isOnline: boolean;
  };
  reproductionSteps?: string[];
  attachments?: {
    screenshot?: string;
    logs?: string[];
    performanceProfile?: any;
    submitted?: boolean;
  };
}

export interface CrashReporterConfig {
  endpoint: string;
  maxReports: number;
  screenshotOnCrash: boolean;
  includePerformanceProfile: boolean;
  anonymizeUserData: boolean;
  autoReport: boolean;
}

export class CrashReporter {
  private reports: CrashReport[] = [];
  private config: CrashReporterConfig;
  private sessionId: string;
  private userId?: string;
  private lastActions: string[] = [];
  private maxLastActions: number = 10;
  private isInitialized: boolean = false;

  constructor(config: Partial<CrashReporterConfig> = {}) {
    this.config = {
      endpoint: '/api/crashes',
      maxReports: 100,
      screenshotOnCrash: true,
      includePerformanceProfile: true,
      anonymizeUserData: false,
      autoReport: true,
      ...config
    };

    this.sessionId = this.generateSessionId();
    this.initialize();
  }

  private generateSessionId(): string {
    return `crash_session_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
  }

  private initialize(): void {
    if (this.isInitialized) return;

    // JavaScript error handling
    window.addEventListener('error', (event) => {
      this.handleJavaScriptError(event);
    });

    // Unhandled promise rejection handling
    window.addEventListener('unhandledrejection', (event) => {
      this.handleUnhandledRejection(event);
    });

    // WebRTC error handling
    this.setupWebRTCErrorHandling();

    // WASM panic handling
    this.setupWasmPanicHandling();

    // Track user actions for context
    this.setupActionTracking();

    // Monitor system resources
    this.startResourceMonitoring();

    this.isInitialized = true;
    console.log('CrashReporter initialized');
  }

  private handleJavaScriptError(event: ErrorEvent): void {
    const crashReport: CrashReport = {
      id: this.generateCrashId(),
      timestamp: Date.now(),
      sessionId: this.sessionId,
      userId: this.userId,
      type: 'javascript_error',
      severity: this.classifyErrorSeverity(event.message, event.error),
      message: event.message,
      stack: event.error?.stack,
      context: {
        url: event.filename || window.location.href,
        userAgent: navigator.userAgent,
        viewport: {
          width: window.innerWidth,
          height: window.innerHeight
        },
        lastActions: [...this.lastActions]
      },
      systemInfo: this.getSystemInfo()
    };

    this.processCrashReport(crashReport);
  }

  private handleUnhandledRejection(event: PromiseRejectionEvent): void {
    const crashReport: CrashReport = {
      id: this.generateCrashId(),
      timestamp: Date.now(),
      sessionId: this.sessionId,
      userId: this.userId,
      type: 'unhandled_rejection',
      severity: this.classifyRejectionSeverity(event.reason),
      message: `Unhandled Promise Rejection: ${event.reason}`,
      stack: event.reason?.stack,
      context: {
        url: window.location.href,
        userAgent: navigator.userAgent,
        viewport: {
          width: window.innerWidth,
          height: window.innerHeight
        },
        lastActions: [...this.lastActions]
      },
      systemInfo: this.getSystemInfo()
    };

    this.processCrashReport(crashReport);
  }

  private setupWebRTCErrorHandling(): void {
    // Listen for WebRTC-specific errors
    document.addEventListener('webrtc-error', (event: any) => {
      const crashReport: CrashReport = {
        id: this.generateCrashId(),
        timestamp: Date.now(),
        sessionId: this.sessionId,
        userId: this.userId,
        type: 'webrtc_error',
        severity: 'high',
        message: `WebRTC Error: ${event.detail.message}`,
        context: {
          url: window.location.href,
          userAgent: navigator.userAgent,
          viewport: {
            width: window.innerWidth,
            height: window.innerHeight
          },
          lastActions: [...this.lastActions]
        },
        systemInfo: this.getSystemInfo()
      };

      this.processCrashReport(crashReport);
    });
  }

  private setupWasmPanicHandling(): void {
    // Listen for WASM panic events
    document.addEventListener('wasm-panic', (event: any) => {
      const crashReport: CrashReport = {
        id: this.generateCrashId(),
        timestamp: Date.now(),
        sessionId: this.sessionId,
        userId: this.userId,
        type: 'wasm_panic',
        severity: 'critical',
        message: `WASM Panic: ${event.detail.message}`,
        stack: event.detail.stack,
        context: {
          url: window.location.href,
          userAgent: navigator.userAgent,
          viewport: {
            width: window.innerWidth,
            height: window.innerHeight
          },
          lastActions: [...this.lastActions]
        },
        systemInfo: this.getSystemInfo()
      };

      this.processCrashReport(crashReport);
    });
  }

  private setupActionTracking(): void {
    const actionEvents = ['click', 'keydown', 'scroll', 'touchstart'];
    
    actionEvents.forEach(eventType => {
      document.addEventListener(eventType, (event) => {
        this.recordAction(eventType, event);
      }, { passive: true });
    });

    // Marco 2.0 specific actions
    document.addEventListener('canvas-node-created', (event: any) => {
      this.recordAction('node_created', { nodeType: event.detail.nodeType });
    });

    document.addEventListener('canvas-connection-made', (event: any) => {
      this.recordAction('connection_made', { 
        source: event.detail.sourceNode,
        target: event.detail.targetNode 
      });
    });
  }

  private recordAction(actionType: string, details?: any): void {
    const action = `${actionType}${details ? `:${JSON.stringify(details)}` : ''}`;
    this.lastActions.push(action);

    if (this.lastActions.length > this.maxLastActions) {
      this.lastActions.shift();
    }
  }

  private startResourceMonitoring(): void {
    // Monitor memory usage every 30 seconds
    setInterval(() => {
      const memInfo = (performance as any).memory;
      if (memInfo && memInfo.usedJSHeapSize > 100 * 1024 * 1024) { // 100MB threshold
        this.recordAction('high_memory_usage', {
          usage: memInfo.usedJSHeapSize,
          limit: memInfo.jsHeapSizeLimit
        });
      }
    }, 30000);
  }

  private classifyErrorSeverity(message: string, error?: Error): CrashReport['severity'] {
    const criticalKeywords = ['cannot read property', 'is not a function', 'null', 'undefined'];
    const highKeywords = ['network', 'fetch', 'websocket', 'webrtc'];
    const mediumKeywords = ['warning', 'deprecated'];

    const lowerMessage = message.toLowerCase();

    if (criticalKeywords.some(keyword => lowerMessage.includes(keyword))) {
      return 'critical';
    }
    if (highKeywords.some(keyword => lowerMessage.includes(keyword))) {
      return 'high';
    }
    if (mediumKeywords.some(keyword => lowerMessage.includes(keyword))) {
      return 'medium';
    }

    return 'low';
  }

  private classifyRejectionSeverity(reason: any): CrashReport['severity'] {
    if (reason instanceof Error) {
      return this.classifyErrorSeverity(reason.message, reason);
    }
    
    const reasonStr = String(reason);
    if (reasonStr.includes('network') || reasonStr.includes('fetch')) {
      return 'high';
    }

    return 'medium';
  }

  private generateCrashId(): string {
    return `crash_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
  }

  private getSystemInfo(): CrashReport['systemInfo'] {
    const memInfo = (performance as any).memory;
    const connection = (navigator as any).connection;
    const battery = (navigator as any).getBattery?.();

    return {
      memoryUsage: memInfo?.usedJSHeapSize || 0,
      connectionType: connection?.effectiveType,
      batteryLevel: battery?.level,
      isOnline: navigator.onLine
    };
  }

  private async processCrashReport(report: CrashReport): Promise<void> {
    console.error('Crash detected:', report);

    // Add attachments if configured
    if (this.config.screenshotOnCrash) {
      report.attachments = {
        ...report.attachments,
        screenshot: await this.captureScreenshot()
      };
    }

    if (this.config.includePerformanceProfile) {
      report.attachments = {
        ...report.attachments,
        performanceProfile: this.capturePerformanceProfile()
      };
    }

    // Anonymize if configured
    if (this.config.anonymizeUserData) {
      report = this.anonymizeReport(report);
    }

    // Store report
    this.reports.push(report);

    // Maintain max reports limit
    if (this.reports.length > this.config.maxReports) {
      this.reports.shift();
    }

    // Auto-report if configured
    if (this.config.autoReport) {
      await this.submitReport(report);
    }

    // Trigger custom event for UI notification
    document.dispatchEvent(new CustomEvent('crash-detected', {
      detail: { report }
    }));
  }

  private async captureScreenshot(): Promise<string | undefined> {
    try {
      if (!('html2canvas' in window)) {
        return undefined;
      }

      const canvas = await (window as any).html2canvas(document.body, {
        width: window.innerWidth,
        height: window.innerHeight,
        useCORS: true
      });

      return canvas.toDataURL('image/png');
    } catch (error) {
      console.warn('Failed to capture screenshot:', error);
      return undefined;
    }
  }

  private capturePerformanceProfile(): any {
    const entries = performance.getEntriesByType('measure');
    const memInfo = (performance as any).memory;

    return {
      measureEntries: entries.map(entry => ({
        name: entry.name,
        duration: entry.duration,
        startTime: entry.startTime
      })),
      memoryInfo: memInfo ? {
        usedJSHeapSize: memInfo.usedJSHeapSize,
        totalJSHeapSize: memInfo.totalJSHeapSize,
        jsHeapSizeLimit: memInfo.jsHeapSizeLimit
      } : null,
      timing: performance.timing
    };
  }

  private anonymizeReport(report: CrashReport): CrashReport {
    const anonymized = { ...report };
    
    // Remove user ID
    delete anonymized.userId;
    
    // Anonymize URLs
    anonymized.context.url = anonymized.context.url.replace(/\/\/[^\/]+/, '//[ANONYMIZED]');
    
    // Remove potentially sensitive stack trace paths
    if (anonymized.stack) {
      anonymized.stack = anonymized.stack.replace(/file:\/\/[^\s]+/g, 'file://[PATH]');
    }

    return anonymized;
  }

  public async submitReport(report: CrashReport): Promise<boolean> {
    try {
      const response = await fetch(this.config.endpoint, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(report)
      });

      return response.ok;
    } catch (error) {
      console.error('Failed to submit crash report:', error);
      return false;
    }
  }

  public async submitAllReports(): Promise<void> {
    const pendingReports = this.reports.filter(report => !report.attachments?.submitted);
    
    for (const report of pendingReports) {
      const success = await this.submitReport(report);
      if (success && report.attachments) {
        report.attachments.submitted = true;
      }
    }
  }

  public getReports(): CrashReport[] {
    return [...this.reports];
  }

  public clearReports(): void {
    this.reports = [];
  }

  public setUserId(userId: string): void {
    this.userId = userId;
  }

  public destroy(): void {
    this.isInitialized = false;
    // Remove event listeners would go here if we stored references
    this.clearReports();
  }
}

// Global crash reporter instance
export const crashReporter = new CrashReporter();
