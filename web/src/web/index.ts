/**
 * Marco 2.0 Web Application Entry Point
 * 
 * This file initializes the Marco 2.0 visual IDE in a web browser with full
 * touch support, responsive design, and progressive web app capabilities.
 */

import './styles/main.css';
import { TouchEventHandler, TouchData } from './touch';
import { PerformanceMonitor } from './performance';
import { PWAManager } from './pwa';

// Import WASM module (will be available after build)
let wasmModule: any = null;

interface Marco2Config {
  canvasId: string;
  enableTouch: boolean;
  enableDebug: boolean;
  performanceMode: 'auto' | 'high' | 'balanced' | 'power-saver';
  mobileMode?: boolean;
}

interface IMarco2WebApp {
  start(): Promise<void>;
  stop(): void;
  resize(width: number, height: number): void;
  handleTouchStart(touchData: any): boolean;
  handleTouchMove(touchData: any): boolean;
  handleTouchEnd(touchData: any): boolean;
  handleMouseDown(x: number, y: number, button: number): boolean;
  handleMouseMove(x: number, y: number): boolean;
  handleMouseUp(x: number, y: number, button: number): boolean;
  getUiConfig(): any;
  isMobileMode(): boolean;
  toggleMobileMode(): void;
  getPerformanceMetrics(): any;
}

/**
 * Main Marco 2.0 Web Application Class
 */
export class Marco2WebApp implements IMarco2WebApp {
  private wasmApp: any = null;
  private canvas: HTMLCanvasElement;
  private touchHandler: TouchEventHandler;
  private performanceMonitor: PerformanceMonitor;
  private pwaManager: PWAManager;
  private config: Marco2Config;
  private animationFrameId: number | null = null;
  private isRunning: boolean = false;

  constructor(config: Partial<Marco2Config> = {}) {
    this.config = {
      canvasId: 'marco2-canvas',
      enableTouch: true,
      enableDebug: false,
      performanceMode: 'auto',
      ...config,
    };

    // Get canvas element
    this.canvas = document.getElementById(this.config.canvasId) as HTMLCanvasElement;
    if (!this.canvas) {
      throw new Error(`Canvas element with id "${this.config.canvasId}" not found`);
    }

    // Initialize subsystems
    this.touchHandler = new TouchEventHandler(this.canvas);
    this.performanceMonitor = new PerformanceMonitor();
    this.pwaManager = new PWAManager();

    this.setupCanvas();
    this.setupEventListeners();
  }

  /**
   * Initialize and start the application
   */
  async start(): Promise<void> {
    if (this.isRunning) {
      return;
    }

    try {
      // Load WASM module
      if (!wasmModule) {
        console.log('Loading Marco 2.0 WASM module...');
        wasmModule = await import('../pkg/marco_2_web.js');
        await wasmModule.default(); // Initialize WASM
        wasmModule.init_marco2_logging();
      }

      // Create WASM application instance
      this.wasmApp = new wasmModule.Marco2Web(this.config);

      // Start the WASM application
      await this.wasmApp.start();

      // Start performance monitoring
      this.performanceMonitor.start();

      // Start animation loop
      this.startAnimationLoop();

      // Initialize PWA features
      this.pwaManager.init();

      this.isRunning = true;
      console.log('Marco 2.0 Web application started successfully');

    } catch (error) {
      console.error('Failed to start Marco 2.0 Web application:', error);
      throw error;
    }
  }

  /**
   * Stop the application
   */
  stop(): void {
    if (!this.isRunning) {
      return;
    }

    if (this.animationFrameId) {
      cancelAnimationFrame(this.animationFrameId);
      this.animationFrameId = null;
    }

    if (this.wasmApp) {
      this.wasmApp.stop();
    }

    this.performanceMonitor.stop();
    this.isRunning = false;

    console.log('Marco 2.0 Web application stopped');
  }

  /**
   * Resize the application
   */
  resize(width: number, height: number): void {
    // Update canvas size
    this.canvas.width = width;
    this.canvas.height = height;
    
    // Update CSS size (for proper DPI handling)
    this.canvas.style.width = `${width}px`;
    this.canvas.style.height = `${height}px`;

    // Notify WASM application
    if (this.wasmApp) {
      this.wasmApp.resize(width, height);
    }

    console.debug(`Canvas resized to ${width}x${height}`);
  }

  /**
   * Handle touch start
   */
  handleTouchStart(touchData: any): boolean {
    return this.wasmApp?.handleTouchStart(touchData) ?? false;
  }

  /**
   * Handle touch move
   */
  handleTouchMove(touchData: any): boolean {
    return this.wasmApp?.handleTouchMove(touchData) ?? false;
  }

  /**
   * Handle touch end
   */
  handleTouchEnd(touchData: any): boolean {
    return this.wasmApp?.handleTouchEnd(touchData) ?? false;
  }

  /**
   * Handle mouse down
   */
  handleMouseDown(x: number, y: number, button: number): boolean {
    return this.wasmApp?.handleMouseDown(x, y, button) ?? false;
  }

  /**
   * Handle mouse move
   */
  handleMouseMove(x: number, y: number): boolean {
    return this.wasmApp?.handleMouseMove(x, y) ?? false;
  }

  /**
   * Handle mouse up
   */
  handleMouseUp(x: number, y: number, button: number): boolean {
    return this.wasmApp?.handleMouseUp(x, y, button) ?? false;
  }

  /**
   * Get current UI configuration
   */
  getUiConfig(): any {
    return this.wasmApp?.getUiConfig();
  }

  /**
   * Check if in mobile mode
   */
  isMobileMode(): boolean {
    return this.wasmApp?.isMobileMode() ?? false;
  }

  /**
   * Toggle mobile mode
   */
  toggleMobileMode(): void {
    this.wasmApp?.toggleMobileMode();
  }

  /**
   * Get performance metrics
   */
  getPerformanceMetrics(): any {
    const wasmMetrics = this.wasmApp?.getPerformanceMetrics();
    const webMetrics = this.performanceMonitor.getMetrics();
    
    return {
      wasm: wasmMetrics,
      web: webMetrics,
    };
  }

  /**
   * Setup canvas properties
   */
  private setupCanvas(): void {
    // Set canvas size to match container
    this.resizeCanvas();

    // Prevent context menu on right click
    this.canvas.addEventListener('contextmenu', (e) => e.preventDefault());

    // Disable text selection
    this.canvas.style.userSelect = 'none';
    this.canvas.style.webkitUserSelect = 'none';
    this.canvas.style.touchAction = 'none';
  }

  /**
   * Setup event listeners
   */
  private setupEventListeners(): void {
    // Window resize
    window.addEventListener('resize', () => this.resizeCanvas());
    window.addEventListener('orientationchange', () => {
      setTimeout(() => this.resizeCanvas(), 100);
    });

    // Touch events
    if (this.config.enableTouch) {
      this.touchHandler.onTouchStart = (touchData: TouchData[]) => {
        return this.wasmApp?.handleTouchStart(touchData) ?? false;
      };

      this.touchHandler.onTouchMove = (touchData: TouchData[]) => {
        return this.wasmApp?.handleTouchMove(touchData) ?? false;
      };

      this.touchHandler.onTouchEnd = (touchData: TouchData[]) => {
        return this.wasmApp?.handleTouchEnd(touchData) ?? false;
      };
    }

    // Mouse events (for desktop)
    this.canvas.addEventListener('mousedown', (e) => {
      const rect = this.canvas.getBoundingClientRect();
      const x = e.clientX - rect.left;
      const y = e.clientY - rect.top;
      this.wasmApp?.handleMouseDown(x, y, e.button);
    });

    this.canvas.addEventListener('mousemove', (e) => {
      const rect = this.canvas.getBoundingClientRect();
      const x = e.clientX - rect.left;
      const y = e.clientY - rect.top;
      this.wasmApp?.handleMouseMove(x, y);
    });

    this.canvas.addEventListener('mouseup', (e) => {
      const rect = this.canvas.getBoundingClientRect();
      const x = e.clientX - rect.left;
      const y = e.clientY - rect.top;
      this.wasmApp?.handleMouseUp(x, y, e.button);
    });

    // Keyboard events
    window.addEventListener('keydown', this.handleKeyDown.bind(this));
    window.addEventListener('keyup', this.handleKeyUp.bind(this));

    // Visibility change (for performance optimization)
    document.addEventListener('visibilitychange', () => {
      if (document.hidden) {
        this.performanceMonitor.pause();
      } else {
        this.performanceMonitor.resume();
      }
    });
  }

  /**
   * Resize canvas to match container
   */
  private resizeCanvas(): void {
    const container = this.canvas.parentElement;
    if (!container) return;

    const { clientWidth, clientHeight } = container;
    
    // Update canvas size
    this.canvas.width = clientWidth;
    this.canvas.height = clientHeight;
    
    // Update CSS size (for proper DPI handling)
    this.canvas.style.width = `${clientWidth}px`;
    this.canvas.style.height = `${clientHeight}px`;

    // Notify WASM application
    if (this.wasmApp) {
      this.wasmApp.resize(clientWidth, clientHeight);
    }

    console.debug(`Canvas resized to ${clientWidth}x${clientHeight}`);
  }

  /**
   * Start the animation loop
   */
  private startAnimationLoop(): void {
    const animate = (timestamp: number) => {
      if (!this.isRunning) return;

      this.performanceMonitor.update(timestamp);
      
      // Update WASM application
      // (WASM handles its own update cycle)

      this.animationFrameId = requestAnimationFrame(animate);
    };

    this.animationFrameId = requestAnimationFrame(animate);
  }

  /**
   * Handle keyboard events
   */
  private handleKeyDown(event: KeyboardEvent): void {
    // Handle keyboard shortcuts
    if (event.ctrlKey || event.metaKey) {
      switch (event.key) {
        case 'm':
          event.preventDefault();
          this.toggleMobileMode();
          break;
        case 'd':
          event.preventDefault();
          // Toggle debug mode
          break;
      }
    }
  }

  private handleKeyUp(event: KeyboardEvent): void {
    // Handle key up events if needed
  }
}

/**
 * Initialize Marco 2.0 when DOM is ready
 */
export function initMarco2(config?: Partial<Marco2Config>): Promise<Marco2WebApp> {
  return new Promise((resolve, reject) => {
    if (document.readyState === 'loading') {
      document.addEventListener('DOMContentLoaded', () => {
        try {
          const app = new Marco2WebApp(config);
          resolve(app);
        } catch (error) {
          reject(error);
        }
      });
    } else {
      try {
        const app = new Marco2WebApp(config);
        resolve(app);
      } catch (error) {
        reject(error);
      }
    }
  });
}

// Auto-initialize if canvas exists
if (typeof window !== 'undefined' && document.getElementById('marco2-canvas')) {
  initMarco2().then(app => app.start()).catch(console.error);
}

// Export for manual initialization
export default Marco2WebApp;
