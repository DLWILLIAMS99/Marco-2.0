/**
 * Marco 2.0 - Plugin Manager
 * Comprehensive plugin architecture for extensible functionality
 */

// Core type definitions for plugin system (simplified for standalone implementation)
export interface MetaValue {
    type: string;
    value: any;
}

export interface DotPath {
    segments: string[];
    toString(): string;
}

export interface ScopeId {
    id: string;
}

export interface LogicNode {
    id: string;
    type: string;
    inputs: Record<string, any>;
    outputs: Record<string, any>;
}

export interface Registry {
    get(path: DotPath): MetaValue | undefined;
    set(path: DotPath, value: MetaValue): void;
    watch(path: DotPath, callback: (value: MetaValue) => void): () => void;
    createScope(id: string): ScopeId;
}

// Simple event emitter implementation
class SimpleEventEmitter {
    private events: Map<string, Function[]> = new Map();

    on(event: string, listener: Function): void {
        if (!this.events.has(event)) {
            this.events.set(event, []);
        }
        this.events.get(event)!.push(listener);
    }

    emit(event: string, ...args: any[]): void {
        const listeners = this.events.get(event);
        if (listeners) {
            listeners.forEach(listener => listener(...args));
        }
    }

    off(event: string, listener: Function): void {
        const listeners = this.events.get(event);
        if (listeners) {
            const index = listeners.indexOf(listener);
            if (index > -1) {
                listeners.splice(index, 1);
            }
        }
    }
}

// Plugin System Types
export interface PluginManifest {
    id: string;
    name: string;
    version: string;
    description: string;
    author: string;
    homepage?: string;
    license: string;
    main: string;
    dependencies: Record<string, string>;
    permissions: PluginPermission[];
    categories: PluginCategory[];
    minMarcoVersion: string;
    maxMarcoVersion?: string;
    icon?: string;
    screenshots?: string[];
    keywords: string[];
}

export enum PluginCategory {
    LOGIC_NODES = 'logic-nodes',
    UI_COMPONENTS = 'ui-components',
    VISUALIZATIONS = 'visualizations',
    DATA_SOURCES = 'data-sources',
    EXPORTERS = 'exporters',
    DEVELOPMENT_TOOLS = 'development-tools',
    INTEGRATIONS = 'integrations',
    THEMES = 'themes'
}

export enum PluginPermission {
    READ_REGISTRY = 'read-registry',
    WRITE_REGISTRY = 'write-registry',
    CREATE_NODES = 'create-nodes',
    MODIFY_UI = 'modify-ui',
    NETWORK_ACCESS = 'network-access',
    FILE_SYSTEM = 'file-system',
    CAMERA_ACCESS = 'camera-access',
    MICROPHONE_ACCESS = 'microphone-access',
    LOCATION_ACCESS = 'location-access',
    CLIPBOARD_ACCESS = 'clipboard-access'
}

export interface PluginAPI {
    registry: PluginRegistryAPI;
    nodes: PluginNodesAPI;
    ui: PluginUIAPI;
    events: PluginEventsAPI;
    storage: PluginStorageAPI;
    utils: PluginUtilsAPI;
}

export interface PluginRegistryAPI {
    get(path: DotPath): Promise<MetaValue | undefined>;
    set(path: DotPath, value: MetaValue): Promise<void>;
    watch(path: DotPath, callback: (value: MetaValue) => void): () => void;
    createScope(id: string): Promise<ScopeId>;
}

export interface PluginNodesAPI {
    register(nodeType: string, factory: NodeFactory): void;
    create(type: string, config: any): LogicNode;
    getRegistered(): string[];
}

export interface PluginUIAPI {
    addPanel(id: string, component: any, options: PanelOptions): void;
    addMenuItem(menu: string, item: MenuItemConfig): void;
    addToolbarButton(button: ToolbarButtonConfig): void;
    showNotification(message: string, type: 'info' | 'warning' | 'error'): void;
}

export interface PluginEventsAPI extends SimpleEventEmitter {
    onNodeCreate(callback: (node: LogicNode) => void): void;
    onNodeDelete(callback: (nodeId: string) => void): void;
    onScopeChange(callback: (scopeId: ScopeId) => void): void;
    onRegistryChange(callback: (path: DotPath, value: MetaValue) => void): void;
}

export interface PluginStorageAPI {
    get(key: string): Promise<any>;
    set(key: string, value: any): Promise<void>;
    delete(key: string): Promise<void>;
    clear(): Promise<void>;
}

export interface PluginUtilsAPI {
    generateId(): string;
    validateDotPath(path: string): boolean;
    formatValue(value: MetaValue): string;
    parseValue(str: string): MetaValue;
    debounce<T extends (...args: any[]) => any>(func: T, delay: number): T;
    throttle<T extends (...args: any[]) => any>(func: T, limit: number): T;
}

// Plugin Instance Management
export interface LoadedPlugin {
    manifest: PluginManifest;
    instance: any;
    api: PluginAPI;
    status: PluginStatus;
    loadTime: number;
    errorCount: number;
    lastError?: Error;
}

export enum PluginStatus {
    LOADING = 'loading',
    ACTIVE = 'active',
    INACTIVE = 'inactive',
    ERROR = 'error',
    DISABLED = 'disabled'
}

// Node Factory Interface
export interface NodeFactory {
    create(config: any): LogicNode;
    getInputSpecs(): InputSpec[];
    getOutputSpecs(): OutputSpec[];
    getConfigSchema(): any;
    validateConfig(config: any): ValidationResult;
}

export interface InputSpec {
    name: string;
    type: string;
    required: boolean;
    defaultValue?: MetaValue;
    description?: string;
    validation?: (value: MetaValue) => boolean;
}

export interface OutputSpec {
    name: string;
    type: string;
    description?: string;
}

export interface ValidationResult {
    valid: boolean;
    errors: string[];
    warnings: string[];
}

// UI Extension Interfaces
export interface PanelOptions {
    title: string;
    position: 'left' | 'right' | 'bottom';
    width?: number;
    height?: number;
    resizable?: boolean;
    collapsible?: boolean;
    defaultVisible?: boolean;
}

export interface MenuItemConfig {
    label: string;
    icon?: string;
    shortcut?: string;
    enabled?: boolean;
    submenu?: MenuItemConfig[];
    action: () => void;
}

export interface ToolbarButtonConfig {
    id: string;
    label: string;
    icon: string;
    tooltip?: string;
    position: 'left' | 'center' | 'right';
    action: () => void;
}

// Plugin Manager Implementation
export class PluginManager extends SimpleEventEmitter {
    private plugins: Map<string, LoadedPlugin> = new Map();
    private registry: Registry;
    private pluginPaths: string[] = [];
    private securityManager: PluginSecurityManager;
    private dependencyResolver: DependencyResolver;

    constructor(registry: Registry) {
        super();
        this.registry = registry;
        this.securityManager = new PluginSecurityManager();
        this.dependencyResolver = new DependencyResolver();
    }

    // Plugin Discovery and Loading
    async discoverPlugins(searchPaths: string[]): Promise<PluginManifest[]> {
        const manifests: PluginManifest[] = [];
        
        for (const searchPath of searchPaths) {
            try {
                const pluginDirs = await this.scanDirectory(searchPath);
                
                for (const dir of pluginDirs) {
                    const manifestPath = `${dir}/plugin.json`;
                    if (await this.fileExists(manifestPath)) {
                        const manifest = await this.loadManifest(manifestPath);
                        if (this.validateManifest(manifest)) {
                            manifests.push(manifest);
                        }
                    }
                }
            } catch (error) {
                console.warn(`Failed to scan plugin directory ${searchPath}:`, error);
            }
        }
        
        return manifests;
    }

    async loadPlugin(manifest: PluginManifest): Promise<LoadedPlugin> {
        // Check if already loaded
        if (this.plugins.has(manifest.id)) {
            throw new Error(`Plugin ${manifest.id} is already loaded`);
        }

        // Validate dependencies
        const depCheck = await this.dependencyResolver.checkDependencies(manifest);
        if (!depCheck.satisfied) {
            throw new Error(`Missing dependencies: ${depCheck.missing.join(', ')}`);
        }

        // Security validation
        const securityCheck = this.securityManager.validatePlugin(manifest);
        if (!securityCheck.allowed) {
            throw new Error(`Security validation failed: ${securityCheck.reason}`);
        }

        const startTime = performance.now();
        
        try {
            // Create sandboxed API
            const api = this.createPluginAPI(manifest);
            
            // Load plugin module
            const pluginModule = await this.loadPluginModule(manifest);
            
            // Initialize plugin
            const instance = new pluginModule.default(api);
            if (instance.initialize) {
                await instance.initialize();
            }

            const loadedPlugin: LoadedPlugin = {
                manifest,
                instance,
                api,
                status: PluginStatus.ACTIVE,
                loadTime: performance.now() - startTime,
                errorCount: 0
            };

            this.plugins.set(manifest.id, loadedPlugin);
            this.emit('plugin-loaded', loadedPlugin);
            
            return loadedPlugin;
        } catch (error) {
            const failedPlugin: LoadedPlugin = {
                manifest,
                instance: null,
                api: this.createPluginAPI(manifest),
                status: PluginStatus.ERROR,
                loadTime: performance.now() - startTime,
                errorCount: 1,
                lastError: error instanceof Error ? error : new Error(String(error))
            };
            
            this.plugins.set(manifest.id, failedPlugin);
            this.emit('plugin-error', failedPlugin, error);
            
            throw error;
        }
    }

    async unloadPlugin(pluginId: string): Promise<void> {
        const plugin = this.plugins.get(pluginId);
        if (!plugin) {
            throw new Error(`Plugin ${pluginId} is not loaded`);
        }

        try {
            // Call plugin cleanup
            if (plugin.instance && plugin.instance.cleanup) {
                await plugin.instance.cleanup();
            }

            // Remove from registry
            this.plugins.delete(pluginId);
            
            this.emit('plugin-unloaded', plugin);
        } catch (error) {
            console.error(`Error unloading plugin ${pluginId}:`, error);
            throw error;
        }
    }

    // Plugin API Creation
    private createPluginAPI(manifest: PluginManifest): PluginAPI {
        return {
            registry: this.createRegistryAPI(manifest),
            nodes: this.createNodesAPI(manifest),
            ui: this.createUIAPI(manifest),
            events: this.createEventsAPI(manifest),
            storage: this.createStorageAPI(manifest),
            utils: this.createUtilsAPI(manifest)
        };
    }

    private createRegistryAPI(manifest: PluginManifest): PluginRegistryAPI {
        return {
            get: async (path: DotPath) => {
                this.checkPermission(manifest, PluginPermission.READ_REGISTRY);
                return this.registry.get(path);
            },
            
            set: async (path: DotPath, value: MetaValue) => {
                this.checkPermission(manifest, PluginPermission.WRITE_REGISTRY);
                this.registry.set(path, value);
            },
            
            watch: (path: DotPath, callback: (value: MetaValue) => void) => {
                this.checkPermission(manifest, PluginPermission.READ_REGISTRY);
                return this.registry.watch(path, callback);
            },
            
            createScope: async (id: string) => {
                this.checkPermission(manifest, PluginPermission.WRITE_REGISTRY);
                return this.registry.createScope(id);
            }
        };
    }

    private createNodesAPI(manifest: PluginManifest): PluginNodesAPI {
        const registeredNodes = new Map<string, NodeFactory>();
        
        return {
            register: (nodeType: string, factory: NodeFactory) => {
                this.checkPermission(manifest, PluginPermission.CREATE_NODES);
                
                if (registeredNodes.has(nodeType)) {
                    throw new Error(`Node type ${nodeType} is already registered`);
                }
                
                registeredNodes.set(nodeType, factory);
                this.emit('node-type-registered', manifest.id, nodeType, factory);
            },
            
            create: (type: string, config: any) => {
                const factory = registeredNodes.get(type);
                if (!factory) {
                    throw new Error(`Unknown node type: ${type}`);
                }
                
                const validation = factory.validateConfig(config);
                if (!validation.valid) {
                    throw new Error(`Invalid config: ${validation.errors.join(', ')}`);
                }
                
                return factory.create(config);
            },
            
            getRegistered: () => Array.from(registeredNodes.keys())
        };
    }

    private createUIAPI(manifest: PluginManifest): PluginUIAPI {
        return {
            addPanel: (id: string, component: any, options: PanelOptions) => {
                this.checkPermission(manifest, PluginPermission.MODIFY_UI);
                this.emit('ui-panel-add', manifest.id, id, component, options);
            },
            
            addMenuItem: (menu: string, item: MenuItemConfig) => {
                this.checkPermission(manifest, PluginPermission.MODIFY_UI);
                this.emit('ui-menu-add', manifest.id, menu, item);
            },
            
            addToolbarButton: (button: ToolbarButtonConfig) => {
                this.checkPermission(manifest, PluginPermission.MODIFY_UI);
                this.emit('ui-toolbar-add', manifest.id, button);
            },
            
            showNotification: (message: string, type: 'info' | 'warning' | 'error') => {
                this.emit('ui-notification', manifest.id, message, type);
            }
        };
    }

    private createEventsAPI(manifest: PluginManifest): PluginEventsAPI {
        const eventEmitter = new SimpleEventEmitter();
        
        // Forward relevant events to plugin
        this.on('node-create', (node) => eventEmitter.emit('node-create', node));
        this.on('node-delete', (nodeId) => eventEmitter.emit('node-delete', nodeId));
        this.on('scope-change', (scopeId) => eventEmitter.emit('scope-change', scopeId));
        this.on('registry-change', (path, value) => eventEmitter.emit('registry-change', path, value));
        
        return Object.assign(eventEmitter, {
            onNodeCreate: (callback: (node: LogicNode) => void) => {
                eventEmitter.on('node-create', callback);
            },
            onNodeDelete: (callback: (nodeId: string) => void) => {
                eventEmitter.on('node-delete', callback);
            },
            onScopeChange: (callback: (scopeId: ScopeId) => void) => {
                eventEmitter.on('scope-change', callback);
            },
            onRegistryChange: (callback: (path: DotPath, value: MetaValue) => void) => {
                eventEmitter.on('registry-change', callback);
            }
        });
    }

    private createStorageAPI(manifest: PluginManifest): PluginStorageAPI {
        const storagePrefix = `plugin-${manifest.id}`;
        
        return {
            get: async (key: string) => {
                const fullKey = `${storagePrefix}-${key}`;
                const stored = localStorage.getItem(fullKey);
                return stored ? JSON.parse(stored) : undefined;
            },
            
            set: async (key: string, value: any) => {
                const fullKey = `${storagePrefix}-${key}`;
                localStorage.setItem(fullKey, JSON.stringify(value));
            },
            
            delete: async (key: string) => {
                const fullKey = `${storagePrefix}-${key}`;
                localStorage.removeItem(fullKey);
            },
            
            clear: async () => {
                const keys = Object.keys(localStorage);
                for (const key of keys) {
                    if (key.startsWith(storagePrefix)) {
                        localStorage.removeItem(key);
                    }
                }
            }
        };
    }

    private createUtilsAPI(manifest: PluginManifest): PluginUtilsAPI {
        return {
            generateId: () => `plugin-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`,
            
            validateDotPath: (path: string) => {
                try {
                    // Simple validation - just check if it's a valid string with dots
                    return typeof path === 'string' && path.length > 0;
                } catch {
                    return false;
                }
            },
            
            formatValue: (value: MetaValue) => {
                // Implementation for formatting MetaValue to string
                return JSON.stringify(value);
            },
            
            parseValue: (str: string) => {
                try {
                    return JSON.parse(str) as MetaValue;
                } catch {
                    return { type: 'string', value: str } as MetaValue;
                }
            },
            
            debounce: <T extends (...args: any[]) => any>(func: T, delay: number): T => {
                let timeoutId: number;
                return ((...args: any[]) => {
                    clearTimeout(timeoutId);
                    timeoutId = setTimeout(() => func(...args), delay);
                }) as T;
            },
            
            throttle: <T extends (...args: any[]) => any>(func: T, limit: number): T => {
                let inThrottle: boolean;
                return ((...args: any[]) => {
                    if (!inThrottle) {
                        func(...args);
                        inThrottle = true;
                        setTimeout(() => inThrottle = false, limit);
                    }
                }) as T;
            }
        };
    }

    // Plugin Management
    getLoadedPlugins(): LoadedPlugin[] {
        return Array.from(this.plugins.values());
    }

    getPlugin(pluginId: string): LoadedPlugin | undefined {
        return this.plugins.get(pluginId);
    }

    async enablePlugin(pluginId: string): Promise<void> {
        const plugin = this.plugins.get(pluginId);
        if (!plugin) {
            throw new Error(`Plugin ${pluginId} is not loaded`);
        }
        
        if (plugin.status === PluginStatus.DISABLED) {
            plugin.status = PluginStatus.ACTIVE;
            if (plugin.instance.enable) {
                await plugin.instance.enable();
            }
            this.emit('plugin-enabled', plugin);
        }
    }

    async disablePlugin(pluginId: string): Promise<void> {
        const plugin = this.plugins.get(pluginId);
        if (!plugin) {
            throw new Error(`Plugin ${pluginId} is not loaded`);
        }
        
        if (plugin.status === PluginStatus.ACTIVE) {
            plugin.status = PluginStatus.DISABLED;
            if (plugin.instance.disable) {
                await plugin.instance.disable();
            }
            this.emit('plugin-disabled', plugin);
        }
    }

    // Utility Methods
    private checkPermission(manifest: PluginManifest, permission: PluginPermission): void {
        if (!manifest.permissions.includes(permission)) {
            throw new Error(`Plugin ${manifest.id} does not have permission: ${permission}`);
        }
    }

    private async scanDirectory(path: string): Promise<string[]> {
        // Implementation would scan filesystem for plugin directories
        // This is a placeholder for the actual implementation
        return [];
    }

    private async fileExists(path: string): Promise<boolean> {
        // Implementation would check if file exists
        // This is a placeholder for the actual implementation
        return false;
    }

    private async loadManifest(path: string): Promise<PluginManifest> {
        // Implementation would load and parse plugin.json
        // This is a placeholder for the actual implementation
        throw new Error('Not implemented');
    }

    private validateManifest(manifest: PluginManifest): boolean {
        return !!(manifest.id && manifest.name && manifest.version && manifest.main);
    }

    private async loadPluginModule(manifest: PluginManifest): Promise<any> {
        // Implementation would dynamically import the plugin module
        // This is a placeholder for the actual implementation
        throw new Error('Not implemented');
    }
}

// Security Manager for Plugin Validation
class PluginSecurityManager {
    validatePlugin(manifest: PluginManifest): { allowed: boolean; reason?: string } {
        // Validate plugin permissions and security requirements
        if (this.hasUnsafePermissions(manifest)) {
            return { allowed: false, reason: 'Plugin requests unsafe permissions' };
        }
        
        if (!this.isFromTrustedSource(manifest)) {
            return { allowed: false, reason: 'Plugin is not from a trusted source' };
        }
        
        return { allowed: true };
    }

    private hasUnsafePermissions(manifest: PluginManifest): boolean {
        const unsafePermissions = [
            PluginPermission.FILE_SYSTEM,
            PluginPermission.NETWORK_ACCESS
        ];
        
        return manifest.permissions.some(p => unsafePermissions.includes(p));
    }

    private isFromTrustedSource(manifest: PluginManifest): boolean {
        // In a real implementation, this would check against a whitelist
        // of trusted publishers or verify digital signatures
        return true;
    }
}

// Dependency Resolution for Plugin Dependencies
class DependencyResolver {
    async checkDependencies(manifest: PluginManifest): Promise<{ satisfied: boolean; missing: string[] }> {
        const missing: string[] = [];
        
        for (const [dep, version] of Object.entries(manifest.dependencies)) {
            if (!await this.isDependencyAvailable(dep, version)) {
                missing.push(`${dep}@${version}`);
            }
        }
        
        return {
            satisfied: missing.length === 0,
            missing
        };
    }

    private async isDependencyAvailable(name: string, version: string): Promise<boolean> {
        // Implementation would check if dependency is available
        // This could check npm registry, local modules, etc.
        return true;
    }
}

export default PluginManager;
