/**
 * Enhanced Marco 2.0 Web Application Entry Point
 * 
 * Week 2 Sprint 2 implementation with performance optimization, offline support, and PWA functionality
 */

import './styles/main.css';
import { Marco2WebApp } from './web/index';
import { TouchManager } from './web/touch';
import { PerformanceOptimizer } from './web/performance-optimizer';
import { PWAManager } from './web/pwa';
import OfflineManager from './web/offline-manager';

/**
 * Enhanced Marco 2.0 Web Application with Week 2 features
 */
class EnhancedMarco2WebApp extends Marco2WebApp {
  private performanceOptimizer: PerformanceOptimizer;
  private offlineManager: OfflineManager;
  private updatePrompt: HTMLElement | null = null;

  constructor(canvasId: string) {
    super(canvasId);
    
    // Initialize performance optimizer
    this.performanceOptimizer = new PerformanceOptimizer({
      targetFps: 60,
      maxMemoryMB: 100,
      enableAdaptiveQuality: true,
      qualityLevel: 'auto',
    });

    // Initialize offline manager
    this.offlineManager = new OfflineManager({
      maxStorageMB: 50,
      autoSync: true,
      conflictResolution: 'manual',
    });

    this.setupEnhancedFeatures();
  }

  public async init(): Promise<void> {
    try {
      // Initialize base app
      await super.init();
      
      // Initialize enhanced features
      await this.offlineManager.initialize();
      await this.pwaManager.init();
      
      // Start performance monitoring
      this.performanceOptimizer.startMonitoring();
      
      // Setup enhanced event handlers
      this.setupUpdateHandling();
      this.setupPerformanceHandling();
      this.setupOfflineHandling();
      
      console.log('Enhanced Marco 2.0 Web App initialized successfully');
    } catch (error) {
      console.error('Failed to initialize enhanced app:', error);
      throw error;
    }
  }

  /**
   * Enhanced touch handling with performance optimization
   */
  protected setupTouchHandling(): void {
    super.setupTouchHandling();
    
    // Measure touch latency for performance optimization
    const originalHandleTouch = this.touchManager.handleTouchStart.bind(this.touchManager);
    this.touchManager.handleTouchStart = (event: TouchEvent) => {
      const startTime = performance.now();
      const result = originalHandleTouch(event);
      this.performanceOptimizer.measureTouchLatency(startTime);
      return result;
    };
  }

  /**
   * Enhanced render loop with performance monitoring
   */
  protected renderLoop(): void {
    const renderOperation = () => {
      super.renderLoop();
    };
    
    this.performanceOptimizer.measureWasmOperation(renderOperation, 'renderLoop');
  }

  /**
   * Project management with offline support
   */
  public async saveProject(projectData: any): Promise<string> {
    try {
      // Try online save first
      if (navigator.onLine) {
        const response = await fetch('/api/projects/save', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(projectData),
        });
        
        if (response.ok) {
          const result = await response.json();
          console.log('Project saved online:', result.id);
          return result.id;
        }
      }
      
      // Fallback to offline save
      const projectId = await this.offlineManager.saveProject({
        id: projectData.id || this.generateProjectId(),
        name: projectData.name || 'Untitled Project',
        data: projectData,
      });
      
      console.log('Project saved offline:', projectId);
      this.showOfflineNotification('Project saved offline. Will sync when online.');
      return projectId;
      
    } catch (error) {
      console.error('Failed to save project:', error);
      throw error;
    }
  }

  public async loadProject(projectId: string): Promise<any> {
    try {
      // Try online load first
      if (navigator.onLine) {
        const response = await fetch(`/api/projects/${projectId}`);
        if (response.ok) {
          return await response.json();
        }
      }
      
      // Fallback to offline load
      const offlineProject = await this.offlineManager.loadProject(projectId);
      if (offlineProject) {
        this.showOfflineNotification('Loaded offline version of project.');
        return offlineProject.data;
      }
      
      throw new Error('Project not found online or offline');
    } catch (error) {
      console.error('Failed to load project:', error);
      throw error;
    }
  }

  public async listProjects(): Promise<any[]> {
    try {
      // Try online list first
      if (navigator.onLine) {
        const response = await fetch('/api/projects');
        if (response.ok) {
          return await response.json();
        }
      }
      
      // Fallback to offline list
      const offlineProjects = await this.offlineManager.listProjects();
      return offlineProjects.map(p => ({
        id: p.id,
        name: p.name,
        lastModified: p.lastModified,
        syncStatus: p.syncStatus,
      }));
    } catch (error) {
      console.error('Failed to list projects:', error);
      return [];
    }
  }

  private setupEnhancedFeatures(): void {
    // Optimization event handling
    window.addEventListener('marco2-optimization', (event: CustomEvent) => {
      const { type, data } = event.detail;
      console.log(`Performance optimization: ${type}`, data);
      
      switch (type) {
        case 'quality-reduced':
          this.showPerformanceNotification('Graphics quality reduced to improve performance');
          break;
        case 'memory-cleanup':
          this.cleanupResources();
          break;
        case 'touch-optimized':
          this.optimizeTouchHandling(data);
          break;
      }
    });

    // Keyboard shortcuts for enhanced features
    document.addEventListener('keydown', (event) => {
      if (event.ctrlKey || event.metaKey) {
        switch (event.key) {
          case 'p':
            event.preventDefault();
            this.togglePerformancePanel();
            break;
          case 'o':
            event.preventDefault();
            this.toggleOfflinePanel();
            break;
          case 's':
            if (event.shiftKey) {
              event.preventDefault();
              this.syncAllProjects();
            }
            break;
        }
      }
    });
  }

  private setupUpdateHandling(): void {
    this.pwaManager.onUpdateAvailable((updateInfo) => {
      this.showUpdatePrompt(updateInfo);
    });
  }

  private setupPerformanceHandling(): void {
    // Monitor performance and show warnings
    setInterval(() => {
      const metrics = this.performanceOptimizer.getMetrics();
      
      if (metrics.fps < 20) {
        this.showPerformanceWarning('Low frame rate detected. Consider reducing quality.');
      }
      
      if (metrics.memoryUsage > 80) {
        this.showPerformanceWarning('High memory usage detected. Consider clearing cache.');
      }
    }, 5000);
  }

  private setupOfflineHandling(): void {
    this.pwaManager.onOfflineStatusChange((status) => {
      if (status.isOffline) {
        this.showOfflineNotification('You are now offline. Changes will be saved locally.');
      } else {
        this.showOfflineNotification('Back online! Syncing your changes...');
        this.syncAllProjects();
      }
    });
  }

  private async syncAllProjects(): Promise<void> {
    try {
      const results = await this.offlineManager.syncAll();
      
      if (results.success > 0) {
        this.showOfflineNotification(`Synced ${results.success} projects successfully.`);
      }
      
      if (results.conflicts > 0) {
        this.showConflictDialog(results.conflicts);
      }
      
      if (results.failed > 0) {
        this.showOfflineNotification(`Failed to sync ${results.failed} projects.`);
      }
    } catch (error) {
      console.error('Sync failed:', error);
      this.showOfflineNotification('Sync failed. Will retry later.');
    }
  }

  private showUpdatePrompt(updateInfo: any): void {
    if (this.updatePrompt) {
      this.updatePrompt.remove();
    }

    this.updatePrompt = document.createElement('div');
    this.updatePrompt.className = 'update-prompt';
    this.updatePrompt.innerHTML = `
      <div class="update-content">
        <h3>Update Available</h3>
        <p>A new version of Marco 2.0 is available!</p>
        ${updateInfo.releaseNotes ? `<div class="release-notes">${updateInfo.releaseNotes}</div>` : ''}
        <div class="update-actions">
          <button class="update-btn" onclick="this.applyUpdate()">Update Now</button>
          <button class="dismiss-btn" onclick="this.parentElement.parentElement.parentElement.remove()">Later</button>
        </div>
      </div>
    `;

    document.body.appendChild(this.updatePrompt);
  }

  private async applyUpdate(): Promise<void> {
    try {
      await this.pwaManager.applyUpdate();
    } catch (error) {
      console.error('Update failed:', error);
      this.showOfflineNotification('Update failed. Please try again later.');
    }
  }

  private showPerformanceNotification(message: string): void {
    this.showNotification(message, 'performance');
  }

  private showOfflineNotification(message: string): void {
    this.showNotification(message, 'offline');
  }

  private showPerformanceWarning(message: string): void {
    this.showNotification(message, 'warning');
  }

  private showNotification(message: string, type: string): void {
    const notification = document.createElement('div');
    notification.className = `notification notification-${type}`;
    notification.textContent = message;
    
    document.body.appendChild(notification);
    
    setTimeout(() => {
      notification.remove();
    }, 5000);
  }

  private showConflictDialog(conflictCount: number): void {
    const dialog = document.createElement('div');
    dialog.className = 'conflict-dialog';
    dialog.innerHTML = `
      <div class="dialog-content">
        <h3>Sync Conflicts Detected</h3>
        <p>${conflictCount} projects have sync conflicts that need resolution.</p>
        <div class="dialog-actions">
          <button onclick="this.showConflictResolution()">Resolve Now</button>
          <button onclick="this.parentElement.parentElement.parentElement.remove()">Later</button>
        </div>
      </div>
    `;
    
    document.body.appendChild(dialog);
  }

  private togglePerformancePanel(): void {
    const existingPanel = document.querySelector('.performance-panel');
    if (existingPanel) {
      existingPanel.remove();
      return;
    }

    const panel = document.createElement('div');
    panel.className = 'performance-panel debug-panel';
    const metrics = this.performanceOptimizer.getMetrics();
    
    panel.innerHTML = `
      <div class="panel-header">
        <h3>Performance Monitor</h3>
        <button class="close-btn" onclick="this.parentElement.parentElement.remove()">×</button>
      </div>
      <div class="panel-content">
        <div class="metric">FPS: <span>${metrics.fps.toFixed(1)}</span></div>
        <div class="metric">Frame Time: <span>${metrics.frameTime.toFixed(2)}ms</span></div>
        <div class="metric">Memory: <span>${metrics.memoryUsage.toFixed(1)}MB</span></div>
        <div class="metric">Touch Latency: <span>${metrics.touchLatency.toFixed(2)}ms</span></div>
        <div class="recommendations">
          <h4>Recommendations:</h4>
          <ul>
            ${this.performanceOptimizer.getRecommendations().map(r => `<li>${r}</li>`).join('')}
          </ul>
        </div>
      </div>
    `;
    
    document.body.appendChild(panel);
  }

  private async toggleOfflinePanel(): Promise<void> {
    const existingPanel = document.querySelector('.offline-panel');
    if (existingPanel) {
      existingPanel.remove();
      return;
    }

    const panel = document.createElement('div');
    panel.className = 'offline-panel debug-panel';
    const stats = await this.offlineManager.getStorageStats();
    const offlineStatus = this.pwaManager.getOfflineStatus();
    
    panel.innerHTML = `
      <div class="panel-header">
        <h3>Offline Status</h3>
        <button class="close-btn" onclick="this.parentElement.parentElement.remove()">×</button>
      </div>
      <div class="panel-content">
        <div class="status">Online: <span>${!offlineStatus.isOffline ? 'Yes' : 'No'}</span></div>
        <div class="status">Sync Status: <span>${offlineStatus.syncStatus}</span></div>
        <div class="status">Projects: <span>${stats.projectCount}</span></div>
        <div class="status">Pending Sync: <span>${stats.pendingSync}</span></div>
        <div class="status">Storage Used: <span>${(stats.used / 1024 / 1024).toFixed(1)}MB</span></div>
        <div class="actions">
          <button onclick="this.syncAllProjects()">Sync All</button>
          <button onclick="this.clearOfflineData()">Clear Cache</button>
        </div>
      </div>
    `;
    
    document.body.appendChild(panel);
  }

  private cleanupResources(): void {
    // Clean up canvas buffers, unused textures, etc.
    console.log('Cleaning up resources for memory optimization');
  }

  private optimizeTouchHandling(options: any): void {
    // Apply touch optimization settings
    if (options.throttle) {
      this.touchManager.setThrottleMode(true);
    }
    
    if (options.passive) {
      this.touchManager.setPassiveMode(true);
    }
  }

  private generateProjectId(): string {
    return `project_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
  }

  private async clearOfflineData(): Promise<void> {
    try {
      await this.offlineManager.clearAllData();
      this.showOfflineNotification('Offline data cleared successfully.');
    } catch (error) {
      console.error('Failed to clear offline data:', error);
      this.showOfflineNotification('Failed to clear offline data.');
    }
  }
}

/**
 * Initialize the enhanced application
 */
async function initializeApp(): Promise<void> {
  try {
    const app = new EnhancedMarco2WebApp('marco2-canvas');
    await app.init();
    
    // Make app globally available for debugging
    (window as any).marco2App = app;
    
    console.log('Marco 2.0 Enhanced Web App ready!');
  } catch (error) {
    console.error('Failed to initialize Marco 2.0:', error);
    
    // Show error to user
    const errorDiv = document.createElement('div');
    errorDiv.className = 'init-error';
    errorDiv.innerHTML = `
      <div class="error-content">
        <h2>Failed to Load Marco 2.0</h2>
        <p>Sorry, there was an error loading the application. Please refresh the page or check your connection.</p>
        <div class="error-details">
          <details>
            <summary>Error Details</summary>
            <pre>${error}</pre>
          </details>
        </div>
        <button onclick="location.reload()">Refresh</button>
      </div>
    `;
    document.body.appendChild(errorDiv);
  }
}

// Start the application when DOM is ready
if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', initializeApp);
} else {
  initializeApp();
}

export { EnhancedMarco2WebApp, PerformanceOptimizer, OfflineManager };
