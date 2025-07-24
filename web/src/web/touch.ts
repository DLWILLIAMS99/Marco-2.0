/**
 * Touch Event Handler for Marco 2.0 Web
 * 
 * Handles touch events and converts them to the format expected by WASM
 */

export interface TouchData {
  identifier: number;
  clientX: number;
  clientY: number;
  pageX: number;
  pageY: number;
  screenX: number;
  screenY: number;
  radiusX?: number;
  radiusY?: number;
  rotationAngle?: number;
  force?: number;
}

export type TouchEventCallback = (touchData: TouchData[]) => boolean;

export class TouchEventHandler {
  private canvas: HTMLCanvasElement;
  public onTouchStart: TouchEventCallback | null = null;
  public onTouchMove: TouchEventCallback | null = null;
  public onTouchEnd: TouchEventCallback | null = null;

  constructor(canvas: HTMLCanvasElement) {
    this.canvas = canvas;
    this.setupEventListeners();
  }

  private setupEventListeners(): void {
    // Touch events
    this.canvas.addEventListener('touchstart', this.handleTouchStart.bind(this), { passive: false });
    this.canvas.addEventListener('touchmove', this.handleTouchMove.bind(this), { passive: false });
    this.canvas.addEventListener('touchend', this.handleTouchEnd.bind(this), { passive: false });
    this.canvas.addEventListener('touchcancel', this.handleTouchEnd.bind(this), { passive: false });

    // Pointer events (for better compatibility)
    this.canvas.addEventListener('pointerdown', this.handlePointerDown.bind(this));
    this.canvas.addEventListener('pointermove', this.handlePointerMove.bind(this));
    this.canvas.addEventListener('pointerup', this.handlePointerUp.bind(this));
    this.canvas.addEventListener('pointercancel', this.handlePointerUp.bind(this));
  }

  private handleTouchStart(event: TouchEvent): void {
    event.preventDefault();
    const touchData = this.convertTouches(event.changedTouches);
    if (this.onTouchStart) {
      this.onTouchStart(touchData);
    }
  }

  private handleTouchMove(event: TouchEvent): void {
    event.preventDefault();
    const touchData = this.convertTouches(event.changedTouches);
    if (this.onTouchMove) {
      this.onTouchMove(touchData);
    }
  }

  private handleTouchEnd(event: TouchEvent): void {
    event.preventDefault();
    const touchData = this.convertTouches(event.changedTouches);
    if (this.onTouchEnd) {
      this.onTouchEnd(touchData);
    }
  }

  private handlePointerDown(event: PointerEvent): void {
    if (event.pointerType === 'touch') {
      const touchData = this.convertPointerEvent(event);
      if (this.onTouchStart) {
        this.onTouchStart([touchData]);
      }
    }
  }

  private handlePointerMove(event: PointerEvent): void {
    if (event.pointerType === 'touch') {
      const touchData = this.convertPointerEvent(event);
      if (this.onTouchMove) {
        this.onTouchMove([touchData]);
      }
    }
  }

  private handlePointerUp(event: PointerEvent): void {
    if (event.pointerType === 'touch') {
      const touchData = this.convertPointerEvent(event);
      if (this.onTouchEnd) {
        this.onTouchEnd([touchData]);
      }
    }
  }

  private convertTouches(touches: TouchList): TouchData[] {
    const result: TouchData[] = [];
    for (let i = 0; i < touches.length; i++) {
      const touch = touches[i];
      result.push(this.convertTouch(touch));
    }
    return result;
  }

  private convertTouch(touch: Touch): TouchData {
    return {
      identifier: touch.identifier,
      clientX: touch.clientX,
      clientY: touch.clientY,
      pageX: touch.pageX,
      pageY: touch.pageY,
      screenX: touch.screenX,
      screenY: touch.screenY,
      radiusX: touch.radiusX,
      radiusY: touch.radiusY,
      rotationAngle: touch.rotationAngle,
      force: touch.force,
    };
  }

  private convertPointerEvent(event: PointerEvent): TouchData {
    return {
      identifier: event.pointerId,
      clientX: event.clientX,
      clientY: event.clientY,
      pageX: event.pageX,
      pageY: event.pageY,
      screenX: event.screenX,
      screenY: event.screenY,
      radiusX: event.width / 2,
      radiusY: event.height / 2,
      rotationAngle: 0,
      force: event.pressure,
    };
  }
}
