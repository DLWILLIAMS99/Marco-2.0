/**
 * Offline Manager for Marco 2.0
 * 
 * Handles offline data storage, synchronization, and conflict resolution
 */

export interface OfflineProject {
  id: string;
  name: string;
  data: any;
  lastModified: number;
  version: number;
  syncStatus: 'local' | 'syncing' | 'synced' | 'conflict';
  conflictData?: any;
}

export interface SyncConflict {
  projectId: string;
  localVersion: number;
  remoteVersion: number;
  localData: any;
  remoteData: any;
  conflictType: 'data' | 'version' | 'deletion';
}

export interface OfflineSettings {
  maxStorageMB: number;
  autoSync: boolean;
  conflictResolution: 'local' | 'remote' | 'manual';
  compressionEnabled: boolean;
  encryptionEnabled: boolean;
}

export class OfflineManager {
  private db: IDBDatabase | null = null;
  private settings: OfflineSettings;
  private storageQuota = 0;
  private usedStorage = 0;
  private syncQueue: string[] = [];
  private isOnline = navigator.onLine;

  private readonly DB_NAME = 'marco2-offline';
  private readonly DB_VERSION = 1;
  private readonly STORES = {
    PROJECTS: 'projects',
    SETTINGS: 'settings',
    ASSETS: 'assets',
    SYNC_QUEUE: 'syncQueue',
  };

  constructor(settings: Partial<OfflineSettings> = {}) {
    this.settings = {
      maxStorageMB: 50,
      autoSync: true,
      conflictResolution: 'manual',
      compressionEnabled: true,
      encryptionEnabled: false,
      ...settings,
    };

    this.setupEventListeners();
  }

  /**
   * Initialize the offline manager
   */
  public async initialize(): Promise<void> {
    try {
      await this.openDatabase();
      await this.checkStorageQuota();
      await this.loadSyncQueue();
      
      console.log('Offline manager initialized');
      
      if (this.isOnline && this.settings.autoSync) {
        this.processSyncQueue();
      }
    } catch (error) {
      console.error('Failed to initialize offline manager:', error);
      throw error;
    }
  }

  /**
   * Save project offline
   */
  public async saveProject(project: Omit<OfflineProject, 'lastModified' | 'version' | 'syncStatus'>): Promise<string> {
    const offlineProject: OfflineProject = {
      ...project,
      lastModified: Date.now(),
      version: await this.getNextVersion(project.id),
      syncStatus: 'local',
    };

    try {
      await this.storeProject(offlineProject);
      
      // Add to sync queue if online
      if (this.isOnline && this.settings.autoSync) {
        this.addToSyncQueue(project.id);
      }
      
      console.log(`Project ${project.id} saved offline`);
      return project.id;
    } catch (error) {
      console.error('Failed to save project offline:', error);
      throw error;
    }
  }

  /**
   * Load project from offline storage
   */
  public async loadProject(projectId: string): Promise<OfflineProject | null> {
    try {
      const transaction = this.db!.transaction([this.STORES.PROJECTS], 'readonly');
      const store = transaction.objectStore(this.STORES.PROJECTS);
      const request = store.get(projectId);

      return new Promise((resolve, reject) => {
        request.onsuccess = () => resolve(request.result || null);
        request.onerror = () => reject(request.error);
      });
    } catch (error) {
      console.error('Failed to load project:', error);
      return null;
    }
  }

  /**
   * List all offline projects
   */
  public async listProjects(): Promise<OfflineProject[]> {
    try {
      const transaction = this.db!.transaction([this.STORES.PROJECTS], 'readonly');
      const store = transaction.objectStore(this.STORES.PROJECTS);
      const request = store.getAll();

      return new Promise((resolve, reject) => {
        request.onsuccess = () => resolve(request.result || []);
        request.onerror = () => reject(request.error);
      });
    } catch (error) {
      console.error('Failed to list projects:', error);
      return [];
    }
  }

  /**
   * Delete project from offline storage
   */
  public async deleteProject(projectId: string): Promise<void> {
    try {
      const transaction = this.db!.transaction([this.STORES.PROJECTS], 'readwrite');
      const store = transaction.objectStore(this.STORES.PROJECTS);
      await store.delete(projectId);
      
      // Remove from sync queue
      this.removeFromSyncQueue(projectId);
      
      console.log(`Project ${projectId} deleted from offline storage`);
    } catch (error) {
      console.error('Failed to delete project:', error);
      throw error;
    }
  }

  /**
   * Sync specific project with server
   */
  public async syncProject(projectId: string): Promise<boolean> {
    if (!this.isOnline) {
      console.warn('Cannot sync project while offline');
      return false;
    }

    try {
      const localProject = await this.loadProject(projectId);
      if (!localProject) {
        console.warn(`Project ${projectId} not found locally`);
        return false;
      }

      // Update sync status
      localProject.syncStatus = 'syncing';
      await this.storeProject(localProject);

      // Attempt to sync with server
      const syncResult = await this.performServerSync(localProject);
      
      if (syncResult.success) {
        localProject.syncStatus = 'synced';
        await this.storeProject(localProject);
        this.removeFromSyncQueue(projectId);
        return true;
      } else if (syncResult.conflict) {
        localProject.syncStatus = 'conflict';
        localProject.conflictData = syncResult.remoteData;
        await this.storeProject(localProject);
        return false;
      } else {
        localProject.syncStatus = 'local';
        await this.storeProject(localProject);
        return false;
      }
    } catch (error) {
      console.error(`Failed to sync project ${projectId}:`, error);
      return false;
    }
  }

  /**
   * Sync all pending projects
   */
  public async syncAll(): Promise<{ success: number; failed: number; conflicts: number }> {
    if (!this.isOnline) {
      return { success: 0, failed: 0, conflicts: 0 };
    }

    const results = { success: 0, failed: 0, conflicts: 0 };
    const projects = await this.listProjects();
    const unsyncedProjects = projects.filter(p => p.syncStatus !== 'synced');

    for (const project of unsyncedProjects) {
      const syncResult = await this.syncProject(project.id);
      if (syncResult) {
        results.success++;
      } else {
        const updatedProject = await this.loadProject(project.id);
        if (updatedProject?.syncStatus === 'conflict') {
          results.conflicts++;
        } else {
          results.failed++;
        }
      }
    }

    console.log('Sync results:', results);
    return results;
  }

  /**
   * Resolve sync conflict
   */
  public async resolveConflict(projectId: string, resolution: 'local' | 'remote' | 'merge'): Promise<boolean> {
    try {
      const project = await this.loadProject(projectId);
      if (!project || project.syncStatus !== 'conflict') {
        return false;
      }

      let resolvedData;
      switch (resolution) {
        case 'local':
          resolvedData = project.data;
          break;
        case 'remote':
          resolvedData = project.conflictData;
          break;
        case 'merge':
          resolvedData = await this.mergeProjectData(project.data, project.conflictData);
          break;
        default:
          return false;
      }

      project.data = resolvedData;
      project.syncStatus = 'local';
      project.conflictData = undefined;
      project.version++;
      project.lastModified = Date.now();

      await this.storeProject(project);
      
      if (this.settings.autoSync) {
        this.addToSyncQueue(projectId);
      }

      return true;
    } catch (error) {
      console.error('Failed to resolve conflict:', error);
      return false;
    }
  }

  /**
   * Get storage statistics
   */
  public async getStorageStats(): Promise<{
    used: number;
    available: number;
    quota: number;
    projectCount: number;
    pendingSync: number;
  }> {
    const projects = await this.listProjects();
    const pendingSync = projects.filter(p => p.syncStatus !== 'synced').length;

    return {
      used: this.usedStorage,
      available: this.storageQuota - this.usedStorage,
      quota: this.storageQuota,
      projectCount: projects.length,
      pendingSync,
    };
  }

  /**
   * Clear all offline data
   */
  public async clearAllData(): Promise<void> {
    try {
      const transaction = this.db!.transaction(Object.values(this.STORES), 'readwrite');
      
      for (const storeName of Object.values(this.STORES)) {
        const store = transaction.objectStore(storeName);
        await store.clear();
      }
      
      this.syncQueue = [];
      console.log('All offline data cleared');
    } catch (error) {
      console.error('Failed to clear offline data:', error);
      throw error;
    }
  }

  /**
   * Private methods
   */

  private async openDatabase(): Promise<void> {
    return new Promise((resolve, reject) => {
      const request = indexedDB.open(this.DB_NAME, this.DB_VERSION);

      request.onerror = () => reject(request.error);
      request.onsuccess = () => {
        this.db = request.result;
        resolve();
      };

      request.onupgradeneeded = (event) => {
        const db = (event.target as IDBOpenDBRequest).result;

        // Projects store
        if (!db.objectStoreNames.contains(this.STORES.PROJECTS)) {
          const projectStore = db.createObjectStore(this.STORES.PROJECTS, { keyPath: 'id' });
          projectStore.createIndex('syncStatus', 'syncStatus', { unique: false });
          projectStore.createIndex('lastModified', 'lastModified', { unique: false });
        }

        // Settings store
        if (!db.objectStoreNames.contains(this.STORES.SETTINGS)) {
          db.createObjectStore(this.STORES.SETTINGS, { keyPath: 'key' });
        }

        // Assets store
        if (!db.objectStoreNames.contains(this.STORES.ASSETS)) {
          db.createObjectStore(this.STORES.ASSETS, { keyPath: 'id' });
        }

        // Sync queue store
        if (!db.objectStoreNames.contains(this.STORES.SYNC_QUEUE)) {
          db.createObjectStore(this.STORES.SYNC_QUEUE, { keyPath: 'id' });
        }
      };
    });
  }

  private async checkStorageQuota(): Promise<void> {
    if ('storage' in navigator && 'estimate' in navigator.storage) {
      try {
        const estimate = await navigator.storage.estimate();
        this.storageQuota = estimate.quota || 50 * 1024 * 1024; // 50MB default
        this.usedStorage = estimate.usage || 0;
        
        console.log(`Storage quota: ${(this.storageQuota / 1024 / 1024).toFixed(2)}MB`);
        console.log(`Storage used: ${(this.usedStorage / 1024 / 1024).toFixed(2)}MB`);
      } catch (error) {
        console.warn('Could not estimate storage quota:', error);
        this.storageQuota = 50 * 1024 * 1024; // 50MB default
      }
    }
  }

  private async storeProject(project: OfflineProject): Promise<void> {
    const transaction = this.db!.transaction([this.STORES.PROJECTS], 'readwrite');
    const store = transaction.objectStore(this.STORES.PROJECTS);
    
    // Compress data if enabled
    if (this.settings.compressionEnabled) {
      project.data = await this.compressData(project.data);
    }

    return new Promise((resolve, reject) => {
      const request = store.put(project);
      request.onsuccess = () => resolve();
      request.onerror = () => reject(request.error);
    });
  }

  private async getNextVersion(projectId: string): Promise<number> {
    const existing = await this.loadProject(projectId);
    return existing ? existing.version + 1 : 1;
  }

  private async loadSyncQueue(): Promise<void> {
    try {
      const transaction = this.db!.transaction([this.STORES.SYNC_QUEUE], 'readonly');
      const store = transaction.objectStore(this.STORES.SYNC_QUEUE);
      const request = store.getAll();

      return new Promise((resolve, reject) => {
        request.onsuccess = () => {
          this.syncQueue = request.result.map(item => item.id);
          resolve();
        };
        request.onerror = () => reject(request.error);
      });
    } catch (error) {
      console.error('Failed to load sync queue:', error);
      this.syncQueue = [];
    }
  }

  private async addToSyncQueue(projectId: string): Promise<void> {
    if (!this.syncQueue.includes(projectId)) {
      this.syncQueue.push(projectId);
      await this.persistSyncQueue();
    }
  }

  private async removeFromSyncQueue(projectId: string): Promise<void> {
    const index = this.syncQueue.indexOf(projectId);
    if (index > -1) {
      this.syncQueue.splice(index, 1);
      await this.persistSyncQueue();
    }
  }

  private async persistSyncQueue(): Promise<void> {
    try {
      const transaction = this.db!.transaction([this.STORES.SYNC_QUEUE], 'readwrite');
      const store = transaction.objectStore(this.STORES.SYNC_QUEUE);
      
      await store.clear();
      for (const projectId of this.syncQueue) {
        await store.add({ id: projectId, timestamp: Date.now() });
      }
    } catch (error) {
      console.error('Failed to persist sync queue:', error);
    }
  }

  private async processSyncQueue(): Promise<void> {
    if (this.syncQueue.length === 0) return;

    console.log(`Processing sync queue: ${this.syncQueue.length} items`);
    
    for (const projectId of [...this.syncQueue]) {
      await this.syncProject(projectId);
      // Small delay to avoid overwhelming the server
      await new Promise(resolve => setTimeout(resolve, 100));
    }
  }

  private async performServerSync(project: OfflineProject): Promise<{
    success: boolean;
    conflict: boolean;
    remoteData?: any;
  }> {
    try {
      // This would be replaced with actual server API calls
      const response = await fetch(`/api/projects/${project.id}/sync`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          id: project.id,
          data: project.data,
          version: project.version,
          lastModified: project.lastModified,
        }),
      });

      if (response.status === 409) {
        // Conflict detected
        const conflictData = await response.json();
        return { success: false, conflict: true, remoteData: conflictData };
      }

      if (response.ok) {
        return { success: true, conflict: false };
      }

      return { success: false, conflict: false };
    } catch (error) {
      console.error('Server sync failed:', error);
      return { success: false, conflict: false };
    }
  }

  private async mergeProjectData(localData: any, remoteData: any): Promise<any> {
    // Simple merge strategy - this would be more sophisticated in practice
    return {
      ...remoteData,
      ...localData,
      lastModified: Math.max(localData.lastModified || 0, remoteData.lastModified || 0),
    };
  }

  private async compressData(data: any): Promise<any> {
    if (!this.settings.compressionEnabled) return data;
    
    try {
      // Simple JSON compression - in practice, would use actual compression
      const jsonString = JSON.stringify(data);
      return { compressed: true, data: jsonString };
    } catch (error) {
      console.warn('Failed to compress data:', error);
      return data;
    }
  }

  private setupEventListeners(): void {
    window.addEventListener('online', () => {
      this.isOnline = true;
      console.log('Back online - processing sync queue');
      if (this.settings.autoSync) {
        this.processSyncQueue();
      }
    });

    window.addEventListener('offline', () => {
      this.isOnline = false;
      console.log('Gone offline - sync disabled');
    });
  }
}

export default OfflineManager;
