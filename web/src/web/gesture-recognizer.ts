/**
 * Advanced Gesture Recognition for Marco 2.0
 * 
 * Multi-touch gesture support with pinch-to-zoom, rotation, and custom gestures
 */

export interface TouchPoint {
  id: number;
  x: number;
  y: number;
  force?: number;
  timestamp: number;
}

export interface GestureState {
  type: GestureType;
  startTime: number;
  duration: number;
  touchPoints: TouchPoint[];
  center: { x: number; y: number };
  scale: number;
  rotation: number;
  velocity: { x: number; y: number };
  isActive: boolean;
}

export type GestureType = 
  | 'tap' 
  | 'double-tap' 
  | 'long-press' 
  | 'pan' 
  | 'pinch' 
  | 'rotate' 
  | 'swipe' 
  | 'two-finger-tap'
  | 'three-finger-tap'
  | 'custom';

export interface GestureEvent {
  type: GestureType;
  state: GestureState;
  preventDefault: () => void;
  stopPropagation: () => void;
}

export interface GestureConfig {
  // Tap settings
  tapTimeout: number;
  doubleTapTimeout: number;
  longPressTimeout: number;
  tapThreshold: number;
  
  // Pan settings
  panMinDistance: number;
  panMinVelocity: number;
  
  // Pinch settings
  pinchMinScale: number;
  pinchMaxScale: number;
  pinchThreshold: number;
  
  // Rotation settings
  rotationThreshold: number;
  
  // Swipe settings
  swipeMinDistance: number;
  swipeMinVelocity: number;
  swipeMaxTime: number;
  
  // General settings
  enablePreventDefault: boolean;
  enableMultiTouch: boolean;
  maxTouchPoints: number;
}

export class AdvancedGestureRecognizer {
  private element: HTMLElement;
  private config: GestureConfig;
  private activeTouches: Map<number, TouchPoint> = new Map();
  private gestureState: GestureState | null = null;
  private gestureHistory: GestureState[] = [];
  private timers: Map<string, number> = new Map();
  
  // Event listeners
  private gestureListeners: Map<GestureType, ((event: GestureEvent) => void)[]> = new Map();
  
  // Gesture recognition state
  private lastTapTime = 0;
  private lastTapPosition: { x: number; y: number } | null = null;
  private initialDistance = 0;
  private initialAngle = 0;
  private initialCenter: { x: number; y: number } | null = null;

  private readonly DEFAULT_CONFIG: GestureConfig = {
    tapTimeout: 300,
    doubleTapTimeout: 300,
    longPressTimeout: 500,
    tapThreshold: 10,
    panMinDistance: 10,
    panMinVelocity: 0.3,
    pinchMinScale: 0.1,
    pinchMaxScale: 10,
    pinchThreshold: 10,
    rotationThreshold: 15,
    swipeMinDistance: 50,
    swipeMinVelocity: 0.5,
    swipeMaxTime: 300,
    enablePreventDefault: true,
    enableMultiTouch: true,
    maxTouchPoints: 10,
  };

  constructor(element: HTMLElement, config: Partial<GestureConfig> = {}) {
    this.element = element;
    this.config = { ...this.DEFAULT_CONFIG, ...config };
    
    this.setupEventListeners();
    console.log('Advanced gesture recognizer initialized');
  }

  /**
   * Add gesture event listener
   */
  public on(gestureType: GestureType, callback: (event: GestureEvent) => void): void {
    if (!this.gestureListeners.has(gestureType)) {
      this.gestureListeners.set(gestureType, []);
    }
    this.gestureListeners.get(gestureType)!.push(callback);
  }

  /**
   * Remove gesture event listener
   */
  public off(gestureType: GestureType, callback: (event: GestureEvent) => void): void {
    const listeners = this.gestureListeners.get(gestureType);
    if (listeners) {
      const index = listeners.indexOf(callback);
      if (index !== -1) {
        listeners.splice(index, 1);
      }
    }
  }

  /**
   * Update gesture configuration
   */
  public updateConfig(config: Partial<GestureConfig>): void {
    this.config = { ...this.config, ...config };
  }

  /**
   * Get current gesture state
   */
  public getCurrentGesture(): GestureState | null {
    return this.gestureState;
  }

  /**
   * Get gesture history
   */
  public getGestureHistory(): GestureState[] {
    return [...this.gestureHistory];
  }

  /**
   * Clear gesture history
   */
  public clearHistory(): void {
    this.gestureHistory = [];
  }

  /**
   * Enable/disable gesture recognition
   */
  public setEnabled(enabled: boolean): void {
    if (enabled) {
      this.setupEventListeners();
    } else {
      this.removeEventListeners();
    }
  }

  /**
   * Destroy gesture recognizer
   */
  public destroy(): void {
    this.removeEventListeners();
    this.clearAllTimers();
    this.gestureListeners.clear();
    this.activeTouches.clear();
    this.gestureHistory = [];
  }

  /**
   * Private methods
   */

  private setupEventListeners(): void {
    // Touch events
    this.element.addEventListener('touchstart', this.handleTouchStart.bind(this), { passive: false });
    this.element.addEventListener('touchmove', this.handleTouchMove.bind(this), { passive: false });
    this.element.addEventListener('touchend', this.handleTouchEnd.bind(this), { passive: false });
    this.element.addEventListener('touchcancel', this.handleTouchCancel.bind(this), { passive: false });

    // Mouse events (for desktop testing)
    this.element.addEventListener('mousedown', this.handleMouseDown.bind(this));
    this.element.addEventListener('mousemove', this.handleMouseMove.bind(this));
    this.element.addEventListener('mouseup', this.handleMouseUp.bind(this));
    this.element.addEventListener('wheel', this.handleWheel.bind(this), { passive: false });

    // Pointer events (modern approach)
    if ('PointerEvent' in window) {
      this.element.addEventListener('pointerdown', this.handlePointerDown.bind(this));
      this.element.addEventListener('pointermove', this.handlePointerMove.bind(this));
      this.element.addEventListener('pointerup', this.handlePointerUp.bind(this));
      this.element.addEventListener('pointercancel', this.handlePointerCancel.bind(this));
    }
  }

  private removeEventListeners(): void {
    // Touch events
    this.element.removeEventListener('touchstart', this.handleTouchStart.bind(this));
    this.element.removeEventListener('touchmove', this.handleTouchMove.bind(this));
    this.element.removeEventListener('touchend', this.handleTouchEnd.bind(this));
    this.element.removeEventListener('touchcancel', this.handleTouchCancel.bind(this));

    // Mouse events
    this.element.removeEventListener('mousedown', this.handleMouseDown.bind(this));
    this.element.removeEventListener('mousemove', this.handleMouseMove.bind(this));
    this.element.removeEventListener('mouseup', this.handleMouseUp.bind(this));
    this.element.removeEventListener('wheel', this.handleWheel.bind(this));

    // Pointer events
    if ('PointerEvent' in window) {
      this.element.removeEventListener('pointerdown', this.handlePointerDown.bind(this));
      this.element.removeEventListener('pointermove', this.handlePointerMove.bind(this));
      this.element.removeEventListener('pointerup', this.handlePointerUp.bind(this));
      this.element.removeEventListener('pointercancel', this.handlePointerCancel.bind(this));
    }
  }

  // Touch Event Handlers
  private handleTouchStart(event: TouchEvent): void {
    if (this.config.enablePreventDefault) {
      event.preventDefault();
    }

    const now = Date.now();
    
    for (let i = 0; i < event.changedTouches.length; i++) {
      const touch = event.changedTouches[i];
      const touchPoint: TouchPoint = {
        id: touch.identifier,
        x: touch.clientX,
        y: touch.clientY,
        force: touch.force,
        timestamp: now,
      };
      
      this.activeTouches.set(touch.identifier, touchPoint);
    }

    this.processGestureStart();
  }

  private handleTouchMove(event: TouchEvent): void {
    if (this.config.enablePreventDefault) {
      event.preventDefault();
    }

    const now = Date.now();
    
    for (let i = 0; i < event.changedTouches.length; i++) {
      const touch = event.changedTouches[i];
      const existingTouch = this.activeTouches.get(touch.identifier);
      
      if (existingTouch) {
        const touchPoint: TouchPoint = {
          id: touch.identifier,
          x: touch.clientX,
          y: touch.clientY,
          force: touch.force,
          timestamp: now,
        };
        
        this.activeTouches.set(touch.identifier, touchPoint);
      }
    }

    this.processGestureMove();
  }

  private handleTouchEnd(event: TouchEvent): void {
    if (this.config.enablePreventDefault) {
      event.preventDefault();
    }

    for (let i = 0; i < event.changedTouches.length; i++) {
      const touch = event.changedTouches[i];
      this.activeTouches.delete(touch.identifier);
    }

    this.processGestureEnd();
  }

  private handleTouchCancel(event: TouchEvent): void {
    this.handleTouchEnd(event);
  }

  // Mouse Event Handlers (for desktop testing)
  private handleMouseDown(event: MouseEvent): void {
    const touchPoint: TouchPoint = {
      id: 0,
      x: event.clientX,
      y: event.clientY,
      timestamp: Date.now(),
    };
    
    this.activeTouches.set(0, touchPoint);
    this.processGestureStart();
  }

  private handleMouseMove(event: MouseEvent): void {
    if (this.activeTouches.has(0)) {
      const touchPoint: TouchPoint = {
        id: 0,
        x: event.clientX,
        y: event.clientY,
        timestamp: Date.now(),
      };
      
      this.activeTouches.set(0, touchPoint);
      this.processGestureMove();
    }
  }

  private handleMouseUp(event: MouseEvent): void {
    this.activeTouches.delete(0);
    this.processGestureEnd();
  }

  private handleWheel(event: WheelEvent): void {
    if (this.config.enablePreventDefault) {
      event.preventDefault();
    }

    // Simulate pinch gesture with wheel
    const scale = event.deltaY > 0 ? 0.9 : 1.1;
    const center = { x: event.clientX, y: event.clientY };
    
    this.emitGestureEvent('pinch', {
      type: 'pinch',
      startTime: Date.now(),
      duration: 0,
      touchPoints: [],
      center,
      scale,
      rotation: 0,
      velocity: { x: 0, y: 0 },
      isActive: false,
    });
  }

  // Pointer Event Handlers
  private handlePointerDown(event: PointerEvent): void {
    const touchPoint: TouchPoint = {
      id: event.pointerId,
      x: event.clientX,
      y: event.clientY,
      force: event.pressure,
      timestamp: Date.now(),
    };
    
    this.activeTouches.set(event.pointerId, touchPoint);
    this.processGestureStart();
  }

  private handlePointerMove(event: PointerEvent): void {
    const existingTouch = this.activeTouches.get(event.pointerId);
    
    if (existingTouch) {
      const touchPoint: TouchPoint = {
        id: event.pointerId,
        x: event.clientX,
        y: event.clientY,
        force: event.pressure,
        timestamp: Date.now(),
      };
      
      this.activeTouches.set(event.pointerId, touchPoint);
      this.processGestureMove();
    }
  }

  private handlePointerUp(event: PointerEvent): void {
    this.activeTouches.delete(event.pointerId);
    this.processGestureEnd();
  }

  private handlePointerCancel(event: PointerEvent): void {
    this.handlePointerUp(event);
  }

  // Gesture Processing
  private processGestureStart(): void {
    const touchCount = this.activeTouches.size;
    const now = Date.now();
    
    if (touchCount === 1) {
      const touch = Array.from(this.activeTouches.values())[0];
      
      // Check for double tap
      if (this.lastTapTime && this.lastTapPosition) {
        const timeDiff = now - this.lastTapTime;
        const distance = this.getDistance(touch, this.lastTapPosition);
        
        if (timeDiff < this.config.doubleTapTimeout && distance < this.config.tapThreshold) {
          this.emitTapGesture('double-tap', touch);
          this.lastTapTime = 0;
          this.lastTapPosition = null;
          return;
        }
      }
      
      // Set up long press timer
      this.setTimer('longPress', () => {
        if (this.activeTouches.has(touch.id)) {
          this.emitTapGesture('long-press', touch);
        }
      }, this.config.longPressTimeout);
      
      // Initialize pan gesture
      this.initializeGesture('pan', [touch]);
      
    } else if (touchCount === 2) {
      const touches = Array.from(this.activeTouches.values());
      
      // Initialize pinch/rotate gesture
      this.initialDistance = this.getDistance(touches[0], touches[1]);
      this.initialAngle = this.getAngle(touches[0], touches[1]);
      this.initialCenter = this.getCenter(touches);
      
      this.initializeGesture('pinch', touches);
      
    } else if (touchCount === 3) {
      const touches = Array.from(this.activeTouches.values());
      this.initializeGesture('three-finger-tap', touches);
    }
  }

  private processGestureMove(): void {
    if (!this.gestureState) return;
    
    const touches = Array.from(this.activeTouches.values());
    const touchCount = touches.length;
    
    if (touchCount === 1 && this.gestureState.type === 'pan') {
      this.updatePanGesture(touches[0]);
    } else if (touchCount === 2 && (this.gestureState.type === 'pinch' || this.gestureState.type === 'rotate')) {
      this.updatePinchRotateGesture(touches);
    }
    
    // Update gesture state
    this.gestureState.touchPoints = touches;
    this.gestureState.duration = Date.now() - this.gestureState.startTime;
    this.gestureState.center = this.getCenter(touches);
  }

  private processGestureEnd(): void {
    const touchCount = this.activeTouches.size;
    
    if (touchCount === 0) {
      // All touches ended
      if (this.gestureState) {
        const finalState = { ...this.gestureState, isActive: false };
        
        // Check for tap gesture
        if (finalState.type === 'pan' && finalState.duration < this.config.tapTimeout) {
          const touch = finalState.touchPoints[0];
          if (touch) {
            const distance = this.getDistance(touch, finalState.touchPoints[0]);
            if (distance < this.config.tapThreshold) {
              this.emitTapGesture('tap', touch);
            }
          }
        }
        
        // Check for swipe gesture
        if (finalState.type === 'pan' && this.isSwipeGesture(finalState)) {
          this.emitSwipeGesture(finalState);
        }
        
        // Emit final gesture event
        this.emitGestureEvent(finalState.type, finalState);
        
        // Add to history
        this.addToHistory(finalState);
        
        this.gestureState = null;
      }
      
      this.clearAllTimers();
      
    } else if (this.gestureState) {
      // Some touches remain, update gesture
      const touches = Array.from(this.activeTouches.values());
      this.gestureState.touchPoints = touches;
    }
  }

  private initializeGesture(type: GestureType, touches: TouchPoint[]): void {
    this.clearAllTimers();
    
    this.gestureState = {
      type,
      startTime: Date.now(),
      duration: 0,
      touchPoints: touches,
      center: this.getCenter(touches),
      scale: 1,
      rotation: 0,
      velocity: { x: 0, y: 0 },
      isActive: true,
    };
  }

  private updatePanGesture(touch: TouchPoint): void {
    if (!this.gestureState) return;
    
    const startTouch = this.gestureState.touchPoints[0];
    if (!startTouch) return;
    
    const deltaX = touch.x - startTouch.x;
    const deltaY = touch.y - startTouch.y;
    const distance = Math.sqrt(deltaX * deltaX + deltaY * deltaY);
    
    if (distance > this.config.panMinDistance) {
      this.clearTimer('longPress');
      
      // Calculate velocity
      const deltaTime = touch.timestamp - startTouch.timestamp;
      this.gestureState.velocity = {
        x: deltaX / deltaTime,
        y: deltaY / deltaTime,
      };
      
      this.emitGestureEvent('pan', this.gestureState);
    }
  }

  private updatePinchRotateGesture(touches: TouchPoint[]): void {
    if (!this.gestureState || touches.length !== 2) return;
    
    const currentDistance = this.getDistance(touches[0], touches[1]);
    const currentAngle = this.getAngle(touches[0], touches[1]);
    
    // Calculate scale
    if (this.initialDistance > 0) {
      this.gestureState.scale = currentDistance / this.initialDistance;
    }
    
    // Calculate rotation
    this.gestureState.rotation = currentAngle - this.initialAngle;
    
    // Determine primary gesture
    const scaleChange = Math.abs(this.gestureState.scale - 1);
    const rotationChange = Math.abs(this.gestureState.rotation);
    
    if (scaleChange > this.config.pinchThreshold / 100) {
      this.gestureState.type = 'pinch';
      this.emitGestureEvent('pinch', this.gestureState);
    }
    
    if (rotationChange > this.config.rotationThreshold * Math.PI / 180) {
      this.gestureState.type = 'rotate';
      this.emitGestureEvent('rotate', this.gestureState);
    }
  }

  private emitTapGesture(type: GestureType, touch: TouchPoint): void {
    const gestureState: GestureState = {
      type,
      startTime: touch.timestamp,
      duration: 0,
      touchPoints: [touch],
      center: { x: touch.x, y: touch.y },
      scale: 1,
      rotation: 0,
      velocity: { x: 0, y: 0 },
      isActive: false,
    };
    
    this.emitGestureEvent(type, gestureState);
    this.addToHistory(gestureState);
    
    // Update tap tracking
    if (type === 'tap') {
      this.lastTapTime = touch.timestamp;
      this.lastTapPosition = { x: touch.x, y: touch.y };
    }
  }

  private emitSwipeGesture(gestureState: GestureState): void {
    const swipeState: GestureState = {
      ...gestureState,
      type: 'swipe',
    };
    
    this.emitGestureEvent('swipe', swipeState);
    this.addToHistory(swipeState);
  }

  private emitGestureEvent(type: GestureType, state: GestureState): void {
    const listeners = this.gestureListeners.get(type);
    if (listeners) {
      const event: GestureEvent = {
        type,
        state,
        preventDefault: () => {},
        stopPropagation: () => {},
      };
      
      listeners.forEach(callback => {
        try {
          callback(event);
        } catch (error) {
          console.error('Error in gesture callback:', error);
        }
      });
    }
  }

  private isSwipeGesture(gestureState: GestureState): boolean {
    if (gestureState.touchPoints.length !== 1) return false;
    
    const touch = gestureState.touchPoints[0];
    const startTouch = gestureState.touchPoints[0];
    
    const distance = this.getDistance(touch, startTouch);
    const velocity = Math.sqrt(
      gestureState.velocity.x * gestureState.velocity.x +
      gestureState.velocity.y * gestureState.velocity.y
    );
    
    return distance > this.config.swipeMinDistance &&
           velocity > this.config.swipeMinVelocity &&
           gestureState.duration < this.config.swipeMaxTime;
  }

  private addToHistory(gestureState: GestureState): void {
    this.gestureHistory.push({ ...gestureState });
    
    // Limit history size
    if (this.gestureHistory.length > 50) {
      this.gestureHistory.shift();
    }
  }

  // Utility methods
  private getDistance(point1: { x: number; y: number }, point2: { x: number; y: number }): number {
    const dx = point2.x - point1.x;
    const dy = point2.y - point1.y;
    return Math.sqrt(dx * dx + dy * dy);
  }

  private getAngle(point1: { x: number; y: number }, point2: { x: number; y: number }): number {
    return Math.atan2(point2.y - point1.y, point2.x - point1.x);
  }

  private getCenter(points: { x: number; y: number }[]): { x: number; y: number } {
    if (points.length === 0) return { x: 0, y: 0 };
    
    const sum = points.reduce(
      (acc, point) => ({ x: acc.x + point.x, y: acc.y + point.y }),
      { x: 0, y: 0 }
    );
    
    return {
      x: sum.x / points.length,
      y: sum.y / points.length,
    };
  }

  // Timer management
  private setTimer(name: string, callback: () => void, delay: number): void {
    this.clearTimer(name);
    const timerId = window.setTimeout(callback, delay);
    this.timers.set(name, timerId);
  }

  private clearTimer(name: string): void {
    const timerId = this.timers.get(name);
    if (timerId) {
      clearTimeout(timerId);
      this.timers.delete(name);
    }
  }

  private clearAllTimers(): void {
    for (const [name, timerId] of this.timers) {
      clearTimeout(timerId);
    }
    this.timers.clear();
  }
}

export default AdvancedGestureRecognizer;
