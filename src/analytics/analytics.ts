/**
 * Marco 2.0 - Analytics Engine
 * Real-time user behavior tracking and performance monitoring
 */

export interface AnalyticsEvent {
  timestamp: number;
  sessionId: string;
  userId?: string;
  event: string;
  category: 'user_action' | 'performance' | 'error' | 'system';
  properties: Record<string, any>;
  context: {
    url: string;
    userAgent: string;
    viewport: { width: number; height: number };
    scope?: string;
    nodeId?: string;
  };
}

export interface PerformanceMetrics {
  loadTime: number;
  renderTime: number;
  wasmInitTime: number;
  memoryUsage: number;
  frameRate: number;
  nodeExecutionTime: Record<string, number>;
}

export interface UserBehaviorData {
  interactionCount: number;
  sessionDuration: number;
  featuresUsed: string[];
  errorCount: number;
  lastActivity: number;
}

export class AnalyticsEngine {
  private events: AnalyticsEvent[] = [];
  private sessionId: string;
  private userId?: string;
  private isEnabled: boolean = true;
  private batchSize: number = 50;
  private flushInterval: number = 30000; // 30 seconds
  private endpoint: string = '/api/analytics';
  private performanceObserver?: PerformanceObserver;
  private userBehavior: UserBehaviorData;
  
  constructor(config?: {
    endpoint?: string;
    batchSize?: number;
    flushInterval?: number;
    userId?: string;
  }) {
    this.sessionId = this.generateSessionId();
    this.userId = config?.userId;
    this.endpoint = config?.endpoint || this.endpoint;
    this.batchSize = config?.batchSize || this.batchSize;
    this.flushInterval = config?.flushInterval || this.flushInterval;
    
    this.userBehavior = {
      interactionCount: 0,
      sessionDuration: 0,
      featuresUsed: [],
      errorCount: 0,
      lastActivity: Date.now()
    };
    
    this.initializeTracking();
  }

  private generateSessionId(): string {
    return `session_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
  }

  private initializeTracking(): void {
    // Performance tracking
    this.setupPerformanceObserver();
    
    // User interaction tracking
    this.setupInteractionTracking();
    
    // Error tracking
    this.setupErrorTracking();
    
    // Periodic data flush
    setInterval(() => this.flush(), this.flushInterval);
    
    // Page unload handling
    window.addEventListener('beforeunload', () => this.flush(true));
    
    // Session tracking
    this.trackEvent('session_start', 'system', {
      timestamp: Date.now(),
      userAgent: navigator.userAgent,
      viewport: {
        width: window.innerWidth,
        height: window.innerHeight
      }
    });
  }

  private setupPerformanceObserver(): void {
    if ('PerformanceObserver' in window) {
      this.performanceObserver = new PerformanceObserver((list) => {
        for (const entry of list.getEntries()) {
          this.trackPerformanceEntry(entry);
        }
      });
      
      this.performanceObserver.observe({ entryTypes: ['measure', 'navigation', 'resource'] });
    }
  }

  private setupInteractionTracking(): void {
    // Mouse and touch interactions
    const interactionEvents = ['click', 'touchstart', 'keydown', 'scroll'];
    
    interactionEvents.forEach(eventType => {
      document.addEventListener(eventType, (event) => {
        this.trackUserInteraction(eventType, event);
      }, { passive: true });
    });
    
    // Canvas interactions (Marco 2.0 specific)
    document.addEventListener('canvas-node-created', (event: any) => {
      this.trackEvent('node_created', 'user_action', {
        nodeType: event.detail.nodeType,
        scopeId: event.detail.scopeId,
        position: event.detail.position
      });
    });
    
    document.addEventListener('canvas-connection-made', (event: any) => {
      this.trackEvent('connection_created', 'user_action', {
        sourceNode: event.detail.sourceNode,
        targetNode: event.detail.targetNode,
        connectionType: event.detail.connectionType
      });
    });
  }

  private setupErrorTracking(): void {
    window.addEventListener('error', (event) => {
      this.trackError('javascript_error', {
        message: event.message,
        filename: event.filename,
        lineno: event.lineno,
        colno: event.colno,
        stack: event.error?.stack
      });
    });
    
    window.addEventListener('unhandledrejection', (event) => {
      this.trackError('unhandled_promise_rejection', {
        reason: event.reason?.toString(),
        stack: event.reason?.stack
      });
    });
  }

  public trackEvent(
    event: string,
    category: AnalyticsEvent['category'],
    properties: Record<string, any> = {},
    context?: Partial<AnalyticsEvent['context']>
  ): void {
    if (!this.isEnabled) return;
    
    const analyticsEvent: AnalyticsEvent = {
      timestamp: Date.now(),
      sessionId: this.sessionId,
      userId: this.userId,
      event,
      category,
      properties,
      context: {
        url: window.location.href,
        userAgent: navigator.userAgent,
        viewport: {
          width: window.innerWidth,
          height: window.innerHeight
        },
        ...context
      }
    };
    
    this.events.push(analyticsEvent);
    this.updateUserBehavior(analyticsEvent);
    
    // Auto-flush if batch size reached
    if (this.events.length >= this.batchSize) {
      this.flush();
    }
  }

  private trackUserInteraction(eventType: string, event: Event): void {
    const target = event.target as HTMLElement;
    const properties: Record<string, any> = {
      eventType,
      elementTag: target?.tagName,
      elementId: target?.id,
      elementClass: target?.className
    };
    
    // Add position data for mouse/touch events
    if (event instanceof MouseEvent || event instanceof TouchEvent) {
      const clientX = event instanceof MouseEvent ? event.clientX : event.touches[0]?.clientX;
      const clientY = event instanceof MouseEvent ? event.clientY : event.touches[0]?.clientY;
      
      properties.position = { x: clientX, y: clientY };
    }
    
    // Add key data for keyboard events
    if (event instanceof KeyboardEvent) {
      properties.key = event.key;
      properties.ctrlKey = event.ctrlKey;
      properties.shiftKey = event.shiftKey;
      properties.altKey = event.altKey;
    }
    
    this.trackEvent(`user_${eventType}`, 'user_action', properties);
  }

  private trackPerformanceEntry(entry: PerformanceEntry): void {
    const properties: Record<string, any> = {
      name: entry.name,
      duration: entry.duration,
      startTime: entry.startTime
    };
    
    if (entry.entryType === 'resource') {
      const resourceEntry = entry as PerformanceResourceTiming;
      properties.transferSize = resourceEntry.transferSize;
      properties.encodedBodySize = resourceEntry.encodedBodySize;
      properties.decodedBodySize = resourceEntry.decodedBodySize;
    }
    
    if (entry.entryType === 'navigation') {
      const navEntry = entry as PerformanceNavigationTiming;
      properties.loadEventEnd = navEntry.loadEventEnd;
      properties.domContentLoadedEventEnd = navEntry.domContentLoadedEventEnd;
    }
    
    this.trackEvent(`performance_${entry.entryType}`, 'performance', properties);
  }

  public trackError(errorType: string, errorData: Record<string, any>): void {
    this.userBehavior.errorCount++;
    
    this.trackEvent('error_occurred', 'error', {
      errorType,
      ...errorData,
      timestamp: Date.now()
    });
  }

  public trackFeatureUsage(featureName: string, details?: Record<string, any>): void {
    if (!this.userBehavior.featuresUsed.includes(featureName)) {
      this.userBehavior.featuresUsed.push(featureName);
    }
    
    this.trackEvent('feature_used', 'user_action', {
      featureName,
      ...details
    });
  }

  public trackPerformanceMetrics(metrics: Partial<PerformanceMetrics>): void {
    this.trackEvent('performance_metrics', 'performance', metrics);
  }

  private updateUserBehavior(event: AnalyticsEvent): void {
    this.userBehavior.lastActivity = event.timestamp;
    
    if (event.category === 'user_action') {
      this.userBehavior.interactionCount++;
    }
    
    // Calculate session duration
    const sessionStart = this.events.find(e => e.event === 'session_start')?.timestamp || Date.now();
    this.userBehavior.sessionDuration = event.timestamp - sessionStart;
  }

  public getUserBehaviorSummary(): UserBehaviorData {
    return { ...this.userBehavior };
  }

  public getPerformanceSummary(): Record<string, { total: number; count: number; avg: number }> {
    const performanceEvents = this.events.filter(e => e.category === 'performance');
    const summary: Record<string, { total: number; count: number; avg: number }> = {};
    
    performanceEvents.forEach(event => {
      const eventName = event.event;
      const duration = event.properties.duration || 0;
      
      if (!summary[eventName]) {
        summary[eventName] = { total: 0, count: 0, avg: 0 };
      }
      
      summary[eventName].total += duration;
      summary[eventName].count++;
      summary[eventName].avg = summary[eventName].total / summary[eventName].count;
    });
    
    return summary;
  }

  public async flush(synchronous: boolean = false): Promise<void> {
    if (this.events.length === 0) return;
    
    const eventsToSend = [...this.events];
    this.events = [];
    
    const payload = {
      sessionId: this.sessionId,
      userId: this.userId,
      events: eventsToSend,
      userBehavior: this.getUserBehaviorSummary(),
      timestamp: Date.now()
    };
    
    try {
      if (synchronous && 'sendBeacon' in navigator) {
        // Use sendBeacon for reliable delivery on page unload
        navigator.sendBeacon(this.endpoint, JSON.stringify(payload));
      } else {
        await fetch(this.endpoint, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json'
          },
          body: JSON.stringify(payload)
        });
      }
    } catch (error) {
      console.error('Failed to send analytics data:', error);
      // Re-add events to queue for retry
      this.events.unshift(...eventsToSend);
    }
  }

  public setUserId(userId: string): void {
    this.userId = userId;
    this.trackEvent('user_identified', 'system', { userId });
  }

  public disable(): void {
    this.isEnabled = false;
    this.flush(true);
  }

  public enable(): void {
    this.isEnabled = true;
  }

  public clearData(): void {
    this.events = [];
    this.userBehavior = {
      interactionCount: 0,
      sessionDuration: 0,
      featuresUsed: [],
      errorCount: 0,
      lastActivity: Date.now()
    };
  }
}

// Global analytics instance
export const analytics = new AnalyticsEngine();

// Auto-track page load performance
window.addEventListener('load', () => {
  const navigation = performance.getEntriesByType('navigation')[0] as PerformanceNavigationTiming;
  
  analytics.trackPerformanceMetrics({
    loadTime: navigation.loadEventEnd - navigation.fetchStart,
    renderTime: navigation.domContentLoadedEventEnd - navigation.fetchStart
  });
});
