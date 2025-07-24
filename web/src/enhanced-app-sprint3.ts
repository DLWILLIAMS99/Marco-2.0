/**
 * Enhanced Marco 2.0 Web App with Sprint 3 Features
 * 
 * Integrates collaboration, advanced gestures, and voice commands
 */

import CollaborationManager, { CollaborativeUser, CollaborativeOperation } from './web/collaboration';
import AdvancedGestureRecognizer, { GestureEvent, GestureType } from './web/gesture-recognizer';
import VoiceCommandSystem, { VoiceCommand, SpeechResult } from './web/voice-commands';
import PerformanceOptimizer from './web/performance-optimizer';
import OfflineManager from './web/offline-manager';
import { PWAManager } from './web/pwa';

export interface Marco2WebConfig {
  // Collaboration settings
  enableCollaboration: boolean;
  collaborationServer?: string;
  userName?: string;
  
  // Gesture settings
  enableGestures: boolean;
  gestureConfig?: any;
  
  // Voice settings
  enableVoice: boolean;
  voiceLanguage?: string;
  voiceConfidenceThreshold?: number;
  
  // Performance settings
  enablePerformanceOptimization: boolean;
  targetFPS?: number;
  
  // Offline settings
  enableOfflineMode: boolean;
  
  // Debug settings
  enableDebugMode: boolean;
}

export class Marco2EnhancedApp {
  private canvas: HTMLCanvasElement;
  private ctx: CanvasRenderingContext2D;
  private config: Marco2WebConfig;
  
  // Feature managers
  private collaborationManager: CollaborationManager | null = null;
  private gestureRecognizer: AdvancedGestureRecognizer | null = null;
  private voiceCommandSystem: VoiceCommandSystem | null = null;
  private performanceOptimizer: PerformanceOptimizer | null = null;
  private offlineManager: OfflineManager | null = null;
  private pwaManager: PWAManager | null = null;
  
  // Application state
  private isInitialized = false;
  private activeUsers: Map<string, CollaborativeUser> = new Map();
  private currentProject: any = null;
  private debugPanel: HTMLElement | null = null;
  
  // UI elements
  private userCursors: Map<string, HTMLElement> = new Map();
  private gestureIndicator: HTMLElement | null = null;
  private voiceIndicator: HTMLElement | null = null;
  private collaborationPanel: HTMLElement | null = null;

  private readonly DEFAULT_CONFIG: Marco2WebConfig = {
    enableCollaboration: true,
    enableGestures: true,
    enableVoice: true,
    enablePerformanceOptimization: true,
    enableOfflineMode: true,
    enableDebugMode: false,
    voiceLanguage: 'en-US',
    voiceConfidenceThreshold: 0.7,
    targetFPS: 60,
  };

  constructor(canvas: HTMLCanvasElement, config: Partial<Marco2WebConfig> = {}) {
    this.canvas = canvas;
    this.config = { ...this.DEFAULT_CONFIG, ...config };
    
    const context = canvas.getContext('2d');
    if (!context) {
      throw new Error('Failed to get 2D rendering context');
    }
    this.ctx = context;
    
    this.initializeApp();
  }

  /**
   * Initialize the enhanced application
   */
  private async initializeApp(): Promise<void> {
    try {
      console.log('Initializing Marco 2.0 Enhanced Web App...');
      
      // Initialize core features
      if (this.config.enablePerformanceOptimization) {
        await this.initializePerformanceOptimizer();
      }
      
      if (this.config.enableOfflineMode) {
        await this.initializeOfflineManager();
      }
      
      // Initialize PWA features
      await this.initializePWA();
      
      // Initialize input systems
      if (this.config.enableGestures) {
        await this.initializeGestureRecognizer();
      }
      
      if (this.config.enableVoice) {
        await this.initializeVoiceCommands();
      }
      
      // Initialize collaboration (requires network)
      if (this.config.enableCollaboration) {
        await this.initializeCollaboration();
      }
      
      // Setup UI
      this.setupUI();
      this.setupEventHandlers();
      
      // Setup debug mode
      if (this.config.enableDebugMode) {
        this.setupDebugMode();
      }
      
      this.isInitialized = true;
      console.log('Marco 2.0 Enhanced Web App initialized successfully');
      
    } catch (error) {
      console.error('Failed to initialize enhanced app:', error);
      throw error;
    }
  }

  /**
   * Initialize performance optimization
   */
  private async initializePerformanceOptimizer(): Promise<void> {
    this.performanceOptimizer = new PerformanceOptimizer({
      targetFps: this.config.targetFPS || 60,
      enableAdaptiveQuality: true,
      qualityLevel: 'auto',
    });
    
    this.performanceOptimizer.startMonitoring();
    console.log('Performance optimizer initialized');
  }

  /**
   * Initialize offline management
   */
  private async initializeOfflineManager(): Promise<void> {
    this.offlineManager = new OfflineManager({
      autoSync: true,
      conflictResolution: 'manual',
      maxStorageMB: 100,
    });
    
    await this.offlineManager.initialize();
    console.log('Offline manager initialized');
  }

  /**
   * Initialize PWA features
   */
  private async initializePWA(): Promise<void> {
    this.pwaManager = new PWAManager();
    
    await this.pwaManager.init();
    console.log('PWA manager initialized');
  }

  /**
   * Initialize gesture recognition
   */
  private async initializeGestureRecognizer(): Promise<void> {
    this.gestureRecognizer = new AdvancedGestureRecognizer(this.canvas, {
      enableMultiTouch: true,
      pinchMinScale: 0.1,
      pinchMaxScale: 10,
      ...this.config.gestureConfig,
    });
    
    // Setup gesture event handlers
    this.gestureRecognizer.on('tap', this.handleTapGesture.bind(this));
    this.gestureRecognizer.on('double-tap', this.handleDoubleTapGesture.bind(this));
    this.gestureRecognizer.on('long-press', this.handleLongPressGesture.bind(this));
    this.gestureRecognizer.on('pan', this.handlePanGesture.bind(this));
    this.gestureRecognizer.on('pinch', this.handlePinchGesture.bind(this));
    this.gestureRecognizer.on('rotate', this.handleRotateGesture.bind(this));
    this.gestureRecognizer.on('swipe', this.handleSwipeGesture.bind(this));
    
    console.log('Advanced gesture recognizer initialized');
  }

  /**
   * Initialize voice command system
   */
  private async initializeVoiceCommands(): Promise<void> {
    this.voiceCommandSystem = new VoiceCommandSystem({
      language: this.config.voiceLanguage || 'en-US',
      confidenceThreshold: this.config.voiceConfidenceThreshold || 0.7,
      continuous: true,
      interimResults: true,
    });
    
    // Setup voice command handlers
    this.voiceCommandSystem.addListener(this.handleVoiceCommand.bind(this));
    
    // Add custom Marco 2.0 commands
    this.setupCustomVoiceCommands();
    
    console.log('Voice command system initialized');
  }

  /**
   * Initialize collaboration
   */
  private async initializeCollaboration(): Promise<void> {
    this.collaborationManager = new CollaborationManager(
      undefined, // Auto-generate user ID
      this.config.userName || 'Anonymous User'
    );
    
    // Setup collaboration event handlers
    this.collaborationManager.setEventCallbacks({
      onUserJoined: this.handleUserJoined.bind(this),
      onUserLeft: this.handleUserLeft.bind(this),
      onOperationReceived: this.handleCollaborativeOperation.bind(this),
      onCursorMoved: this.handleRemoteCursor.bind(this),
      onConnectionStateChanged: this.handleConnectionStateChanged.bind(this),
    });
    
    console.log('Collaboration manager initialized');
  }

  /**
   * Setup user interface elements
   */
  private setupUI(): void {
    // Create collaboration panel
    this.createCollaborationPanel();
    
    // Create gesture indicator
    this.createGestureIndicator();
    
    // Create voice indicator
    this.createVoiceIndicator();
    
    // Setup keyboard shortcuts
    this.setupKeyboardShortcuts();
  }

  /**
   * Setup event handlers
   */
  private setupEventHandlers(): void {
    // Canvas resize handling
    window.addEventListener('resize', this.handleResize.bind(this));
    
    // Visibility change handling
    document.addEventListener('visibilitychange', this.handleVisibilityChange.bind(this));
    
    // Mouse move for cursor sharing
    this.canvas.addEventListener('mousemove', this.handleMouseMove.bind(this));
  }

  /**
   * Gesture Event Handlers
   */
  private handleTapGesture(event: GestureEvent): void {
    const { center } = event.state;
    console.log(`Tap gesture at (${center.x}, ${center.y})`);
    
    // Send collaboration event
    if (this.collaborationManager) {
      this.collaborationManager.sendOperation({
        type: 'node-create',
        data: { x: center.x, y: center.y, type: 'default' },
      });
    }
    
    this.showGestureIndicator('tap', center);
  }

  private handleDoubleTapGesture(event: GestureEvent): void {
    const { center } = event.state;
    console.log(`Double-tap gesture at (${center.x}, ${center.y})`);
    
    // Fit to screen or zoom to point
    this.fitToScreen();
    this.showGestureIndicator('double-tap', center);
  }

  private handleLongPressGesture(event: GestureEvent): void {
    const { center } = event.state;
    console.log(`Long press gesture at (${center.x}, ${center.y})`);
    
    // Show context menu
    this.showContextMenu(center.x, center.y);
    this.showGestureIndicator('long-press', center);
  }

  private handlePanGesture(event: GestureEvent): void {
    const { velocity } = event.state;
    
    // Pan the view
    this.panView(velocity.x, velocity.y);
  }

  private handlePinchGesture(event: GestureEvent): void {
    const { scale, center } = event.state;
    
    // Zoom at the pinch center
    this.zoomAtPoint(scale, center.x, center.y);
    this.showGestureIndicator('pinch', center);
  }

  private handleRotateGesture(event: GestureEvent): void {
    const { rotation, center } = event.state;
    
    // Rotate the view
    this.rotateView(rotation);
    this.showGestureIndicator('rotate', center);
  }

  private handleSwipeGesture(event: GestureEvent): void {
    const { velocity } = event.state;
    
    // Navigate or perform quick action based on swipe direction
    const angle = Math.atan2(velocity.y, velocity.x);
    const direction = this.getSwipeDirection(angle);
    
    this.handleSwipeAction(direction);
  }

  /**
   * Voice Command Handlers
   */
  private handleVoiceCommand(command: VoiceCommand, result: SpeechResult): void {
    console.log(`Voice command: ${command.phrase} (confidence: ${result.confidence})`);
    
    this.showVoiceIndicator(command.phrase);
    
    switch (command.action) {
      case 'zoom':
        if (command.parameters?.direction === 'in') {
          this.zoomIn();
        } else {
          this.zoomOut();
        }
        break;
        
      case 'fit-to-screen':
        this.fitToScreen();
        break;
        
      case 'reset-view':
        this.resetView();
        break;
        
      case 'pan':
        this.handleVoicePan(command.parameters?.direction);
        break;
        
      case 'create-node':
        this.createNodeAtCenter();
        break;
        
      case 'save':
        this.saveProject();
        break;
        
      case 'load':
        this.showLoadDialog();
        break;
        
      case 'start-collaboration':
        this.startCollaborativeSession();
        break;
        
      case 'join-collaboration':
        this.showJoinSessionDialog();
        break;
    }
  }

  private setupCustomVoiceCommands(): void {
    if (!this.voiceCommandSystem) return;
    
    const customCommands = [
      { phrase: 'start collaboration', action: 'start-collaboration', category: 'system' as const },
      { phrase: 'join session', action: 'join-collaboration', category: 'system' as const },
      { phrase: 'leave session', action: 'leave-collaboration', category: 'system' as const },
      { phrase: 'show users', action: 'show-users', category: 'system' as const },
      { phrase: 'enable voice chat', action: 'enable-voice-chat', category: 'system' as const },
      { phrase: 'disable voice chat', action: 'disable-voice-chat', category: 'system' as const },
    ];
    
    customCommands.forEach(command => {
      this.voiceCommandSystem!.addCommand(command);
    });
  }

  /**
   * Collaboration Event Handlers
   */
  private handleUserJoined(user: CollaborativeUser): void {
    console.log(`User joined: ${user.name}`);
    this.activeUsers.set(user.id, user);
    this.createUserCursor(user);
    this.updateCollaborationPanel();
  }

  private handleUserLeft(userId: string): void {
    console.log(`User left: ${userId}`);
    this.activeUsers.delete(userId);
    this.removeUserCursor(userId);
    this.updateCollaborationPanel();
  }

  private handleCollaborativeOperation(operation: CollaborativeOperation): void {
    console.log(`Received operation: ${operation.type} from ${operation.userId}`);
    
    // Apply operation to the local state
    this.applyCollaborativeOperation(operation);
  }

  private handleRemoteCursor(userId: string, x: number, y: number): void {
    const cursor = this.userCursors.get(userId);
    if (cursor) {
      cursor.style.left = `${x}px`;
      cursor.style.top = `${y}px`;
    }
  }

  private handleConnectionStateChanged(state: RTCPeerConnectionState): void {
    console.log(`Connection state changed: ${state}`);
    this.updateCollaborationStatus(state);
  }

  /**
   * UI Creation Methods
   */
  private createCollaborationPanel(): void {
    this.collaborationPanel = document.createElement('div');
    this.collaborationPanel.id = 'collaboration-panel';
    this.collaborationPanel.innerHTML = `
      <div class="collaboration-header">
        <h3>Collaboration</h3>
        <button id="start-session-btn">Start Session</button>
        <button id="join-session-btn">Join Session</button>
      </div>
      <div class="active-users">
        <h4>Active Users</h4>
        <div id="users-list"></div>
      </div>
    `;
    
    document.body.appendChild(this.collaborationPanel);
    
    // Add event listeners
    document.getElementById('start-session-btn')?.addEventListener('click', () => {
      this.startCollaborativeSession();
    });
    
    document.getElementById('join-session-btn')?.addEventListener('click', () => {
      this.showJoinSessionDialog();
    });
  }

  private createGestureIndicator(): void {
    this.gestureIndicator = document.createElement('div');
    this.gestureIndicator.id = 'gesture-indicator';
    this.gestureIndicator.className = 'gesture-indicator';
    document.body.appendChild(this.gestureIndicator);
  }

  private createVoiceIndicator(): void {
    this.voiceIndicator = document.createElement('div');
    this.voiceIndicator.id = 'voice-indicator';
    this.voiceIndicator.className = 'voice-indicator';
    this.voiceIndicator.innerHTML = `
      <div class="voice-status">
        <span id="voice-status-text">Voice Ready</span>
        <button id="voice-toggle-btn">🎤</button>
      </div>
      <div id="voice-command-display"></div>
    `;
    
    document.body.appendChild(this.voiceIndicator);
    
    // Add event listeners
    document.getElementById('voice-toggle-btn')?.addEventListener('click', () => {
      this.toggleVoiceRecognition();
    });
  }

  private createUserCursor(user: CollaborativeUser): void {
    const cursor = document.createElement('div');
    cursor.id = `cursor-${user.id}`;
    cursor.className = 'user-cursor';
    cursor.style.backgroundColor = user.color;
    cursor.innerHTML = `
      <div class="cursor-pointer"></div>
      <div class="cursor-label">${user.name}</div>
    `;
    
    document.body.appendChild(cursor);
    this.userCursors.set(user.id, cursor);
  }

  private removeUserCursor(userId: string): void {
    const cursor = this.userCursors.get(userId);
    if (cursor) {
      cursor.remove();
      this.userCursors.delete(userId);
    }
  }

  /**
   * UI Update Methods
   */
  private showGestureIndicator(gestureType: string, position: { x: number; y: number }): void {
    if (!this.gestureIndicator) return;
    
    this.gestureIndicator.textContent = gestureType;
    this.gestureIndicator.style.left = `${position.x}px`;
    this.gestureIndicator.style.top = `${position.y}px`;
    this.gestureIndicator.style.opacity = '1';
    
    setTimeout(() => {
      if (this.gestureIndicator) {
        this.gestureIndicator.style.opacity = '0';
      }
    }, 1000);
  }

  private showVoiceIndicator(command: string): void {
    const display = document.getElementById('voice-command-display');
    if (display) {
      display.textContent = command;
      display.style.opacity = '1';
      
      setTimeout(() => {
        display.style.opacity = '0';
      }, 2000);
    }
  }

  private updateCollaborationPanel(): void {
    const usersList = document.getElementById('users-list');
    if (!usersList) return;
    
    usersList.innerHTML = '';
    this.activeUsers.forEach(user => {
      const userElement = document.createElement('div');
      userElement.className = 'user-item';
      userElement.innerHTML = `
        <div class="user-color" style="background-color: ${user.color}"></div>
        <span class="user-name">${user.name}</span>
        <span class="user-status">${user.isActive ? 'Active' : 'Idle'}</span>
      `;
      usersList.appendChild(userElement);
    });
  }

  /**
   * Application Actions
   */
  private async startCollaborativeSession(): Promise<void> {
    if (!this.collaborationManager) return;
    
    const sessionName = prompt('Enter session name:') || 'Marco 2.0 Session';
    const sessionId = await this.collaborationManager.createSession(sessionName);
    
    alert(`Session created! Share this ID: ${sessionId}`);
  }

  private async showJoinSessionDialog(): Promise<void> {
    if (!this.collaborationManager) return;
    
    const sessionId = prompt('Enter session ID to join:');
    if (sessionId) {
      const success = await this.collaborationManager.joinSession(sessionId);
      if (success) {
        alert('Successfully joined session!');
      } else {
        alert('Failed to join session. Please check the session ID.');
      }
    }
  }

  private toggleVoiceRecognition(): void {
    if (!this.voiceCommandSystem) return;
    
    const isListening = this.voiceCommandSystem.toggleListening();
    const statusText = document.getElementById('voice-status-text');
    const toggleBtn = document.getElementById('voice-toggle-btn');
    
    if (statusText) {
      statusText.textContent = isListening ? 'Listening...' : 'Voice Ready';
    }
    
    if (toggleBtn) {
      toggleBtn.textContent = isListening ? '🔴' : '🎤';
    }
  }

  /**
   * Canvas Operations
   */
  private zoomIn(): void {
    // Implement zoom in logic
    console.log('Zooming in');
  }

  private zoomOut(): void {
    // Implement zoom out logic
    console.log('Zooming out');
  }

  private zoomAtPoint(scale: number, x: number, y: number): void {
    // Implement zoom at point logic
    console.log(`Zooming to scale ${scale} at (${x}, ${y})`);
  }

  private fitToScreen(): void {
    // Implement fit to screen logic
    console.log('Fitting to screen');
  }

  private resetView(): void {
    // Implement reset view logic
    console.log('Resetting view');
  }

  private panView(deltaX: number, deltaY: number): void {
    // Implement pan view logic
    console.log(`Panning by (${deltaX}, ${deltaY})`);
  }

  private rotateView(angle: number): void {
    // Implement rotate view logic
    console.log(`Rotating by ${angle} radians`);
  }

  /**
   * Utility Methods
   */
  private getSwipeDirection(angle: number): string {
    const degrees = (angle * 180) / Math.PI;
    
    if (degrees >= -45 && degrees < 45) return 'right';
    if (degrees >= 45 && degrees < 135) return 'down';
    if (degrees >= 135 || degrees < -135) return 'left';
    return 'up';
  }

  private handleSwipeAction(direction: string): void {
    switch (direction) {
      case 'left':
        console.log('Swipe left - Previous tool');
        break;
      case 'right':
        console.log('Swipe right - Next tool');
        break;
      case 'up':
        console.log('Swipe up - Show menu');
        break;
      case 'down':
        console.log('Swipe down - Hide UI');
        break;
    }
  }

  private handleVoicePan(direction: string): void {
    switch (direction) {
      case 'left':
        this.panView(-50, 0);
        break;
      case 'right':
        this.panView(50, 0);
        break;
      case 'up':
        this.panView(0, -50);
        break;
      case 'down':
        this.panView(0, 50);
        break;
    }
  }

  private createNodeAtCenter(): void {
    const centerX = this.canvas.width / 2;
    const centerY = this.canvas.height / 2;
    
    console.log(`Creating node at center (${centerX}, ${centerY})`);
    
    // Send collaboration event
    if (this.collaborationManager) {
      this.collaborationManager.sendOperation({
        type: 'node-create',
        data: { x: centerX, y: centerY, type: 'voice-created' },
      });
    }
  }

  private saveProject(): void {
    if (this.offlineManager && this.currentProject) {
      const project = {
        id: 'current',
        name: 'Current Project',
        data: this.currentProject,
      };
      this.offlineManager.saveProject(project);
      console.log('Project saved');
    }
  }

  private showLoadDialog(): void {
    // Show load project dialog
    console.log('Showing load dialog');
  }

  private showContextMenu(x: number, y: number): void {
    // Show context menu at position
    console.log(`Showing context menu at (${x}, ${y})`);
  }

  private applyCollaborativeOperation(operation: CollaborativeOperation): void {
    // Apply collaborative operation to local state
    console.log('Applying collaborative operation:', operation);
  }

  private updateCollaborationStatus(state: RTCPeerConnectionState): void {
    // Update UI to reflect collaboration status
    console.log('Updating collaboration status:', state);
  }

  private setupKeyboardShortcuts(): void {
    document.addEventListener('keydown', (event) => {
      if (event.ctrlKey || event.metaKey) {
        switch (event.key) {
          case 'd':
            event.preventDefault();
            this.toggleDebugMode();
            break;
          case 'v':
            event.preventDefault();
            this.toggleVoiceRecognition();
            break;
          case 'c':
            event.preventDefault();
            this.startCollaborativeSession();
            break;
        }
      }
    });
  }

  private setupDebugMode(): void {
    this.debugPanel = document.createElement('div');
    this.debugPanel.id = 'debug-panel';
    this.debugPanel.innerHTML = `
      <h3>Debug Panel</h3>
      <div id="performance-stats"></div>
      <div id="collaboration-stats"></div>
      <div id="gesture-stats"></div>
      <div id="voice-stats"></div>
    `;
    document.body.appendChild(this.debugPanel);
  }

  private toggleDebugMode(): void {
    if (this.debugPanel) {
      this.debugPanel.style.display = 
        this.debugPanel.style.display === 'none' ? 'block' : 'none';
    }
  }

  private handleResize(): void {
    // Handle canvas resize
    const rect = this.canvas.parentElement?.getBoundingClientRect();
    if (rect) {
      this.canvas.width = rect.width;
      this.canvas.height = rect.height;
    }
  }

  private handleVisibilityChange(): void {
    if (document.hidden) {
      // Pause intensive operations when tab is hidden
      this.performanceOptimizer?.stopMonitoring();
    } else {
      // Resume operations when tab becomes visible
      this.performanceOptimizer?.startMonitoring();
    }
  }

  private handleMouseMove(event: MouseEvent): void {
    // Share cursor position for collaboration
    if (this.collaborationManager) {
      this.collaborationManager.updateCursor(event.clientX, event.clientY);
    }
  }

  /**
   * Public API Methods
   */
  public getCollaborationManager(): CollaborationManager | null {
    return this.collaborationManager;
  }

  public getGestureRecognizer(): AdvancedGestureRecognizer | null {
    return this.gestureRecognizer;
  }

  public getVoiceCommandSystem(): VoiceCommandSystem | null {
    return this.voiceCommandSystem;
  }

  public getPerformanceOptimizer(): PerformanceOptimizer | null {
    return this.performanceOptimizer;
  }

  public isReady(): boolean {
    return this.isInitialized;
  }

  public async destroy(): Promise<void> {
    // Cleanup all systems
    if (this.collaborationManager) {
      await this.collaborationManager.leaveSession();
    }
    
    this.gestureRecognizer?.destroy();
    this.voiceCommandSystem?.destroy();
    this.performanceOptimizer?.stopMonitoring();
    
    // Remove UI elements
    this.collaborationPanel?.remove();
    this.gestureIndicator?.remove();
    this.voiceIndicator?.remove();
    this.debugPanel?.remove();
    
    this.userCursors.forEach(cursor => cursor.remove());
    this.userCursors.clear();
    
    console.log('Marco 2.0 Enhanced App destroyed');
  }
}

export default Marco2EnhancedApp;
