/**
 * Progressive Web App Manager for Marco 2.0
 * 
 * Enhanced PWA functionality with service worker integration and offline support
 */

export interface PWAUpdateInfo {
  hasUpdate: boolean;
  version?: string;
  size?: number;
  releaseNotes?: string;
  isRequired?: boolean;
  downloadProgress?: number;
}

export interface PWAInstallPrompt {
  prompt(): Promise<void>;
  userChoice: Promise<{ outcome: 'accepted' | 'dismissed' }>;
}

export interface PWAOfflineStatus {
  isOffline: boolean;
  hasOfflineData: boolean;
  syncStatus: 'synced' | 'pending' | 'failed';
  pendingActions: number;
}

export class PWAManager {
  private installPrompt: PWAInstallPrompt | null = null;
  private isInstalled: boolean = false;
  private serviceWorker: ServiceWorkerRegistration | null = null;
  private updateInfo: PWAUpdateInfo = { hasUpdate: false };
  private offlineStatus: PWAOfflineStatus = {
    isOffline: false,
    hasOfflineData: false,
    syncStatus: 'synced',
    pendingActions: 0,
  };
  private updateCallbacks: Array<(info: PWAUpdateInfo) => void> = [];
  private offlineCallbacks: Array<(status: PWAOfflineStatus) => void> = [];

  constructor() {
    this.detectInstallation();
    this.setupInstallPromptListener();
    this.setupOfflineListener();
    this.setupServiceWorkerListener();
  }

  async init(): Promise<void> {
    await this.registerServiceWorker();
    this.checkForUpdates();
    console.log('PWA Manager initialized');
  }

  isInstallable(): boolean {
    return this.installPrompt !== null;
  }

  isPWAInstalled(): boolean {
    return this.isInstalled;
  }

  async installPWA(): Promise<boolean> {
    if (!this.installPrompt) {
      console.warn('PWA installation prompt not available');
      return false;
    }

    try {
      await this.installPrompt.prompt();
      const choice = await this.installPrompt.userChoice;
      
      if (choice.outcome === 'accepted') {
        console.log('PWA installation accepted');
        this.isInstalled = true;
        return true;
      } else {
        console.log('PWA installation dismissed');
        return false;
      }
    } catch (error) {
      console.error('PWA installation failed:', error);
      return false;
    }
  }

  async registerServiceWorker(): Promise<void> {
    if (!('serviceWorker' in navigator)) {
      console.warn('Service workers not supported');
      return;
    }

    try {
      this.serviceWorker = await navigator.serviceWorker.register('/service-worker.js');
      console.log('Service worker registered successfully');

      // Listen for updates
      this.serviceWorker.addEventListener('updatefound', () => {
        console.log('Service worker update found');
        this.handleServiceWorkerUpdate();
      });

    } catch (error) {
      console.error('Service worker registration failed:', error);
    }
  }

  private detectInstallation(): void {
    // Check if running as installed PWA
    this.isInstalled = window.matchMedia('(display-mode: standalone)').matches ||
                      (window.navigator as any).standalone === true ||
                      document.referrer.includes('android-app://');

    if (this.isInstalled) {
      console.log('Running as installed PWA');
    }
  }

  private setupInstallPromptListener(): void {
    window.addEventListener('beforeinstallprompt', (event) => {
      event.preventDefault();
      this.installPrompt = event as any;
      console.log('PWA install prompt available');
      
      // Dispatch custom event to notify app
      window.dispatchEvent(new CustomEvent('pwa-installable'));
    });

    window.addEventListener('appinstalled', () => {
      console.log('PWA installed successfully');
      this.isInstalled = true;
      this.installPrompt = null;
      
      // Dispatch custom event
      window.dispatchEvent(new CustomEvent('pwa-installed'));
    });
  }

  private handleServiceWorkerUpdate(): void {
    const newWorker = this.serviceWorker?.installing;
    if (!newWorker) return;

    newWorker.addEventListener('statechange', () => {
      if (newWorker.state === 'installed' && navigator.serviceWorker.controller) {
        // New service worker installed, show update notification
        console.log('New service worker version available');
        
        // Dispatch custom event for update notification
        window.dispatchEvent(new CustomEvent('pwa-update-available'));
      }
    });
  }

  async activateUpdate(): Promise<void> {
    if (!this.serviceWorker?.waiting) {
      return;
    }

    // Tell the waiting service worker to become active
    this.serviceWorker.waiting.postMessage({ type: 'SKIP_WAITING' });

    // Reload the page after activation
    navigator.serviceWorker.addEventListener('controllerchange', () => {
      window.location.reload();
    });
  }

  // Offline detection
  isOnline(): boolean {
    return navigator.onLine;
  }

  // Listen for online/offline events
  setupOfflineDetection(
    onOnline?: () => void,
    onOffline?: () => void
  ): void {
    window.addEventListener('online', () => {
      console.log('App is online');
      onOnline?.();
    });

    window.addEventListener('offline', () => {
      console.log('App is offline');
      onOffline?.();
    });
  }

  // Get PWA capabilities
  getCapabilities(): {
    canInstall: boolean;
    isInstalled: boolean;
    hasServiceWorker: boolean;
    isOnline: boolean;
    supportsNotifications: boolean;
  } {
    return {
      canInstall: this.isInstallable(),
      isInstalled: this.isPWAInstalled(),
      hasServiceWorker: this.serviceWorker !== null,
      isOnline: this.isOnline(),
      supportsNotifications: 'Notification' in window,
    };
  }

  // Show install banner (custom implementation)
  showInstallBanner(): HTMLElement | null {
    if (!this.isInstallable() || this.isInstalled) {
      return null;
    }

    const banner = document.createElement('div');
    banner.className = 'pwa-install-banner';
    banner.innerHTML = `
      <div class="pwa-banner-content">
        <div class="pwa-banner-text">
          <h3>Install Marco 2.0</h3>
          <p>Get the full experience with our app!</p>
        </div>
        <div class="pwa-banner-actions">
          <button class="pwa-install-btn">Install</button>
          <button class="pwa-dismiss-btn">×</button>
        </div>
      </div>
    `;

    // Add event listeners
    const installBtn = banner.querySelector('.pwa-install-btn');
    const dismissBtn = banner.querySelector('.pwa-dismiss-btn');

    installBtn?.addEventListener('click', async () => {
      const installed = await this.installPWA();
      if (installed) {
        banner.remove();
      }
    });

    dismissBtn?.addEventListener('click', () => {
      banner.remove();
      localStorage.setItem('pwa-install-dismissed', 'true');
    });

    // Add to document
    document.body.appendChild(banner);

    return banner;
  }

  /**
   * Enhanced update management
   */
  public onUpdateAvailable(callback: (info: PWAUpdateInfo) => void): void {
    this.updateCallbacks.push(callback);
  }

  public async checkForUpdates(): Promise<PWAUpdateInfo> {
    if (!this.serviceWorker) {
      return this.updateInfo;
    }

    try {
      await this.serviceWorker.update();
      
      if (this.serviceWorker.waiting) {
        this.updateInfo = {
          hasUpdate: true,
          version: await this.getVersion(),
          isRequired: false,
        };
        
        this.notifyUpdateCallbacks();
      }
    } catch (error) {
      console.error('Update check failed:', error);
    }

    return this.updateInfo;
  }

  public async applyUpdate(): Promise<boolean> {
    if (!this.updateInfo.hasUpdate || !this.serviceWorker?.waiting) {
      return false;
    }

    try {
      // Tell the waiting service worker to become active
      this.serviceWorker.waiting.postMessage({ type: 'SKIP_WAITING' });
      
      // Wait for the new service worker to take control
      await new Promise<void>((resolve) => {
        const handleControllerChange = () => {
          navigator.serviceWorker.removeEventListener('controllerchange', handleControllerChange);
          resolve();
        };
        navigator.serviceWorker.addEventListener('controllerchange', handleControllerChange);
      });

      // Reload the page to get the new version
      window.location.reload();
      return true;
    } catch (error) {
      console.error('Update application failed:', error);
      return false;
    }
  }

  /**
   * Offline status management
   */
  public onOfflineStatusChange(callback: (status: PWAOfflineStatus) => void): void {
    this.offlineCallbacks.push(callback);
  }

  public getOfflineStatus(): PWAOfflineStatus {
    return { ...this.offlineStatus };
  }

  public async syncOfflineData(): Promise<boolean> {
    if (this.offlineStatus.isOffline) {
      return false;
    }

    try {
      // Trigger background sync
      if ('serviceWorker' in navigator && this.serviceWorker) {
        const registration = await navigator.serviceWorker.ready;
        // Type assertion for background sync support
        if ('sync' in registration) {
          await (registration as any).sync.register('data-sync');
          
          this.offlineStatus.syncStatus = 'pending';
          this.notifyOfflineCallbacks();
          return true;
        }
      }
    } catch (error) {
      console.error('Background sync failed:', error);
      this.offlineStatus.syncStatus = 'failed';
      this.notifyOfflineCallbacks();
    }

    return false;
  }

  /**
   * Enhanced service worker communication
   */
  public async sendMessageToServiceWorker(message: any): Promise<any> {
    if (!navigator.serviceWorker.controller) {
      throw new Error('No active service worker');
    }

    return new Promise((resolve, reject) => {
      const messageChannel = new MessageChannel();
      
      messageChannel.port1.onmessage = (event) => {
        if (event.data.error) {
          reject(new Error(event.data.error));
        } else {
          resolve(event.data);
        }
      };

      navigator.serviceWorker.controller!.postMessage(message, [messageChannel.port2]);
    });
  }

  private setupOfflineListener(): void {
    const updateOfflineStatus = () => {
      const wasOffline = this.offlineStatus.isOffline;
      this.offlineStatus.isOffline = !navigator.onLine;
      
      if (wasOffline && !this.offlineStatus.isOffline) {
        // Just came back online
        this.syncOfflineData();
      }
      
      this.notifyOfflineCallbacks();
    };

    window.addEventListener('online', updateOfflineStatus);
    window.addEventListener('offline', updateOfflineStatus);
    updateOfflineStatus(); // Initial check
  }

  private setupServiceWorkerListener(): void {
    if ('serviceWorker' in navigator) {
      navigator.serviceWorker.addEventListener('message', (event) => {
        const { type, data } = event.data;
        
        switch (type) {
          case 'SW_ACTIVATED':
            console.log('Service worker activated, version:', data.version);
            break;
            
          case 'UPDATE_AVAILABLE':
            this.updateInfo = { hasUpdate: true, ...data };
            this.notifyUpdateCallbacks();
            break;
            
          case 'SYNC_COMPLETE':
            this.offlineStatus.syncStatus = 'synced';
            this.offlineStatus.pendingActions = 0;
            this.notifyOfflineCallbacks();
            break;
            
          case 'SYNC_FAILED':
            this.offlineStatus.syncStatus = 'failed';
            this.notifyOfflineCallbacks();
            break;
        }
      });
    }
  }

  private async getVersion(): Promise<string> {
    try {
      const response = await this.sendMessageToServiceWorker({ type: 'GET_VERSION' });
      return response.version || '1.0.0';
    } catch (error) {
      return '1.0.0';
    }
  }

  private notifyUpdateCallbacks(): void {
    this.updateCallbacks.forEach(callback => {
      try {
        callback(this.updateInfo);
      } catch (error) {
        console.error('Update callback error:', error);
      }
    });
  }

  private notifyOfflineCallbacks(): void {
    this.offlineCallbacks.forEach(callback => {
      try {
        callback(this.offlineStatus);
      } catch (error) {
        console.error('Offline callback error:', error);
      }
    });
  }
}
