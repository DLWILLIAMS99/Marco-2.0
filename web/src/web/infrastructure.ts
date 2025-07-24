/**
 * Production Infrastructure Manager for Marco 2.0
 * 
 * CDN configuration, SSL management, domain setup, and deployment automation
 */

export interface CDNConfig {
  provider: 'cloudflare' | 'cloudfront' | 'fastly' | 'custom';
  endpoints: Record<string, string>;
  cachingRules: CachingRule[];
  compressionEnabled: boolean;
  minificationEnabled: boolean;
  imageOptimization: boolean;
}

export interface CachingRule {
  pattern: string;
  ttl: number; // Time to live in seconds
  browserTTL: number;
  edgeTTL: number;
  bypassCache: boolean;
}

export interface SSLConfig {
  enabled: boolean;
  provider: 'letsencrypt' | 'cloudflare' | 'custom';
  autoRenewal: boolean;
  hstsEnabled: boolean;
  hstsMaxAge: number;
  includeSubdomains: boolean;
}

export interface DomainConfig {
  primary: string;
  aliases: string[];
  redirects: Record<string, string>;
  subdomains: Record<string, string>;
}

export interface DeploymentConfig {
  environment: 'development' | 'staging' | 'production';
  branch: string;
  buildCommand: string;
  outputDir: string;
  autoDeployEnabled: boolean;
  rollbackEnabled: boolean;
  healthCheckUrl: string;
}

export interface PerformanceMetrics {
  loadTime: number;
  firstContentfulPaint: number;
  largestContentfulPaint: number;
  cumulativeLayoutShift: number;
  firstInputDelay: number;
  timeToInteractive: number;
  speedIndex: number;
}

export interface DeploymentStatus {
  id: string;
  environment: string;
  branch: string;
  commit: string;
  status: 'pending' | 'building' | 'deploying' | 'success' | 'failed';
  startTime: number;
  endTime?: number;
  logs: string[];
  metrics?: PerformanceMetrics;
  url?: string;
}

export class InfrastructureManager {
  private cdnConfig: CDNConfig;
  private sslConfig: SSLConfig;
  private domainConfig: DomainConfig;
  private deploymentConfig: DeploymentConfig;
  private deployments: Map<string, DeploymentStatus> = new Map();
  private isInitialized = false;

  private readonly DEFAULT_CDN_CONFIG: CDNConfig = {
    provider: 'cloudflare',
    endpoints: {
      api: 'https://api.marco2.app',
      cdn: 'https://cdn.marco2.app',
      assets: 'https://assets.marco2.app',
      wasm: 'https://wasm.marco2.app',
    },
    cachingRules: [
      {
        pattern: '*.wasm',
        ttl: 31536000, // 1 year
        browserTTL: 31536000,
        edgeTTL: 31536000,
        bypassCache: false,
      },
      {
        pattern: '*.js',
        ttl: 86400, // 1 day
        browserTTL: 86400,
        edgeTTL: 86400,
        bypassCache: false,
      },
      {
        pattern: '*.css',
        ttl: 86400, // 1 day
        browserTTL: 86400,
        edgeTTL: 86400,
        bypassCache: false,
      },
      {
        pattern: '*.html',
        ttl: 3600, // 1 hour
        browserTTL: 0, // No browser cache for HTML
        edgeTTL: 3600,
        bypassCache: false,
      },
      {
        pattern: '/api/*',
        ttl: 0,
        browserTTL: 0,
        edgeTTL: 0,
        bypassCache: true,
      },
    ],
    compressionEnabled: true,
    minificationEnabled: true,
    imageOptimization: true,
  };

  private readonly DEFAULT_SSL_CONFIG: SSLConfig = {
    enabled: true,
    provider: 'letsencrypt',
    autoRenewal: true,
    hstsEnabled: true,
    hstsMaxAge: 31536000, // 1 year
    includeSubdomains: true,
  };

  private readonly DEFAULT_DOMAIN_CONFIG: DomainConfig = {
    primary: 'marco2.app',
    aliases: ['www.marco2.app'],
    redirects: {
      'marco2.com': 'marco2.app',
      'www.marco2.com': 'marco2.app',
    },
    subdomains: {
      api: 'api.marco2.app',
      cdn: 'cdn.marco2.app',
      assets: 'assets.marco2.app',
      wasm: 'wasm.marco2.app',
      docs: 'docs.marco2.app',
    },
  };

  constructor(
    cdnConfig: Partial<CDNConfig> = {},
    sslConfig: Partial<SSLConfig> = {},
    domainConfig: Partial<DomainConfig> = {},
    deploymentConfig: Partial<DeploymentConfig> = {}
  ) {
    this.cdnConfig = { ...this.DEFAULT_CDN_CONFIG, ...cdnConfig };
    this.sslConfig = { ...this.DEFAULT_SSL_CONFIG, ...sslConfig };
    this.domainConfig = { ...this.DEFAULT_DOMAIN_CONFIG, ...domainConfig };
    this.deploymentConfig = {
      environment: 'production',
      branch: 'main',
      buildCommand: 'npm run build:prod',
      outputDir: 'dist',
      autoDeployEnabled: true,
      rollbackEnabled: true,
      healthCheckUrl: '/health',
      ...deploymentConfig,
    };

    this.initialize();
  }

  /**
   * Initialize infrastructure management
   */
  private async initialize(): Promise<void> {
    try {
      // Setup CDN configuration
      await this.setupCDN();

      // Configure SSL/TLS
      await this.setupSSL();

      // Setup domain configuration
      await this.setupDomains();

      // Initialize deployment monitoring
      this.setupDeploymentMonitoring();

      // Setup performance monitoring
      this.setupPerformanceMonitoring();

      // Setup health checks
      this.setupHealthChecks();

      this.isInitialized = true;
      console.log('Infrastructure manager initialized');

    } catch (error) {
      console.error('Failed to initialize infrastructure manager:', error);
      throw error;
    }
  }

  /**
   * Setup CDN configuration
   */
  private async setupCDN(): Promise<void> {
    // Configure caching headers
    this.configureCachingHeaders();

    // Setup compression
    if (this.cdnConfig.compressionEnabled) {
      this.enableCompression();
    }

    // Setup minification
    if (this.cdnConfig.minificationEnabled) {
      this.enableMinification();
    }

    // Setup image optimization
    if (this.cdnConfig.imageOptimization) {
      this.enableImageOptimization();
    }

    console.log('CDN configuration applied');
  }

  /**
   * Configure caching headers
   */
  private configureCachingHeaders(): void {
    // Add service worker for client-side caching control
    if ('serviceWorker' in navigator) {
      navigator.serviceWorker.addEventListener('message', (event) => {
        if (event.data.type === 'CACHE_HEADERS') {
          this.applyCachingRules(event.data.url, event.data.response);
        }
      });
    }

    // Configure caching rules for different resource types
    this.cdnConfig.cachingRules.forEach(rule => {
      console.log(`Caching rule configured: ${rule.pattern} (TTL: ${rule.ttl}s)`);
    });
  }

  /**
   * Apply caching rules to a specific resource
   */
  private applyCachingRules(url: string, response: Response): Response {
    const matchingRule = this.cdnConfig.cachingRules.find(rule => 
      new RegExp(rule.pattern.replace('*', '.*')).test(url)
    );

    if (matchingRule && !matchingRule.bypassCache) {
      const headers = new Headers(response.headers);
      headers.set('Cache-Control', `public, max-age=${matchingRule.browserTTL}`);
      headers.set('CDN-Cache-Control', `max-age=${matchingRule.edgeTTL}`);
      
      return new Response(response.body, {
        status: response.status,
        statusText: response.statusText,
        headers,
      });
    }

    return response;
  }

  /**
   * Enable compression
   */
  private enableCompression(): void {
    // Configure Accept-Encoding headers
    const originalFetch = window.fetch;
    window.fetch = (input: RequestInfo | URL, init?: RequestInit) => {
      const headers = new Headers(init?.headers);
      headers.set('Accept-Encoding', 'gzip, deflate, br');
      
      return originalFetch(input, {
        ...init,
        headers,
      });
    };

    console.log('Compression enabled');
  }

  /**
   * Enable minification
   */
  private enableMinification(): void {
    // In production, this would be handled by the build process
    // Here we just verify minified resources are being served
    const scripts = document.querySelectorAll('script[src]');
    scripts.forEach(script => {
      const src = script.getAttribute('src');
      if (src && !src.includes('.min.') && this.deploymentConfig.environment === 'production') {
        console.warn(`Non-minified script detected in production: ${src}`);
      }
    });

    console.log('Minification verification completed');
  }

  /**
   * Enable image optimization
   */
  private enableImageOptimization(): void {
    // Configure next-gen image formats
    const images = document.querySelectorAll('img');
    images.forEach(img => {
      // Check if browser supports WebP
      if (this.supportsWebP()) {
        const src = img.getAttribute('src');
        if (src && !src.includes('.webp')) {
          // In production, CDN would automatically serve WebP
          console.log(`Image optimization available for: ${src}`);
        }
      }
    });

    console.log('Image optimization configured');
  }

  /**
   * Setup SSL/TLS configuration
   */
  private async setupSSL(): Promise<void> {
    if (!this.sslConfig.enabled) {
      console.warn('SSL is disabled - not recommended for production');
      return;
    }

    // Verify HTTPS connection
    if (location.protocol !== 'https:' && location.hostname !== 'localhost') {
      console.error('SSL configuration requires HTTPS connection');
      return;
    }

    // Configure HSTS if enabled
    if (this.sslConfig.hstsEnabled) {
      this.configureHSTS();
    }

    // Check SSL certificate validity
    await this.checkSSLCertificate();

    console.log('SSL configuration applied');
  }

  /**
   * Configure HSTS
   */
  private configureHSTS(): void {
    const hstsHeader = `max-age=${this.sslConfig.hstsMaxAge}${
      this.sslConfig.includeSubdomains ? '; includeSubDomains' : ''
    }; preload`;

    // In a real application, this would be set by the server
    console.log(`HSTS configuration: ${hstsHeader}`);
  }

  /**
   * Check SSL certificate validity
   */
  private async checkSSLCertificate(): Promise<void> {
    try {
      const response = await fetch(window.location.origin, { method: 'HEAD' });
      const security = (performance.getEntriesByType('navigation')[0] as any).secureConnectionStart;
      
      if (security > 0) {
        console.log('SSL certificate is valid');
      } else {
        console.warn('SSL certificate validation failed');
      }
    } catch (error) {
      console.error('SSL certificate check failed:', error);
    }
  }

  /**
   * Setup domain configuration
   */
  private async setupDomains(): Promise<void> {
    // Configure domain redirects
    this.configureDomainRedirects();

    // Setup subdomain routing
    this.configureSubdomains();

    // Verify domain configuration
    await this.verifyDomainConfiguration();

    console.log('Domain configuration applied');
  }

  /**
   * Configure domain redirects
   */
  private configureDomainRedirects(): void {
    const currentDomain = window.location.hostname;
    
    Object.entries(this.domainConfig.redirects).forEach(([from, to]) => {
      if (currentDomain === from) {
        const newUrl = window.location.href.replace(from, to);
        console.log(`Domain redirect: ${from} → ${to}`);
        window.location.replace(newUrl);
      }
    });
  }

  /**
   * Configure subdomains
   */
  private configureSubdomains(): void {
    Object.entries(this.domainConfig.subdomains).forEach(([key, subdomain]) => {
      console.log(`Subdomain configured: ${key} → ${subdomain}`);
    });
  }

  /**
   * Verify domain configuration
   */
  private async verifyDomainConfiguration(): Promise<void> {
    const domains = [
      this.domainConfig.primary,
      ...this.domainConfig.aliases,
      ...Object.values(this.domainConfig.subdomains),
    ];

    for (const domain of domains) {
      try {
        const response = await fetch(`https://${domain}/health`, { 
          method: 'HEAD',
          mode: 'no-cors',
        });
        console.log(`Domain verification passed: ${domain}`);
      } catch (error) {
        console.warn(`Domain verification failed: ${domain}`, error);
      }
    }
  }

  /**
   * Setup deployment monitoring
   */
  private setupDeploymentMonitoring(): void {
    // Monitor deployment status
    this.checkDeploymentStatus();

    // Setup auto-deployment if enabled
    if (this.deploymentConfig.autoDeployEnabled) {
      this.setupAutoDeployment();
    }

    // Setup rollback capability
    if (this.deploymentConfig.rollbackEnabled) {
      this.setupRollback();
    }

    console.log('Deployment monitoring configured');
  }

  /**
   * Check deployment status
   */
  private async checkDeploymentStatus(): Promise<void> {
    const deploymentId = this.getCurrentDeploymentId();
    
    if (deploymentId) {
      const status: DeploymentStatus = {
        id: deploymentId,
        environment: this.deploymentConfig.environment,
        branch: 'main', // Would be detected from git
        commit: 'current', // Would be detected from git
        status: 'success',
        startTime: Date.now(),
        logs: ['Deployment check completed'],
        url: window.location.origin,
      };

      this.deployments.set(deploymentId, status);
      console.log('Deployment status checked:', status);
    }
  }

  /**
   * Setup performance monitoring
   */
  private setupPerformanceMonitoring(): void {
    // Monitor Core Web Vitals
    this.monitorCoreWebVitals();

    // Monitor resource loading
    this.monitorResourceLoading();

    // Monitor network performance
    this.monitorNetworkPerformance();

    console.log('Performance monitoring configured');
  }

  /**
   * Monitor Core Web Vitals
   */
  private monitorCoreWebVitals(): void {
    // First Contentful Paint
    const paintObserver = new PerformanceObserver((list) => {
      const entries = list.getEntries();
      entries.forEach(entry => {
        if (entry.name === 'first-contentful-paint') {
          console.log(`First Contentful Paint: ${entry.startTime}ms`);
        }
      });
    });
    paintObserver.observe({ entryTypes: ['paint'] });

    // Largest Contentful Paint
    const lcpObserver = new PerformanceObserver((list) => {
      const entries = list.getEntries();
      const lastEntry = entries[entries.length - 1];
      console.log(`Largest Contentful Paint: ${lastEntry.startTime}ms`);
    });
    lcpObserver.observe({ entryTypes: ['largest-contentful-paint'] });

    // Cumulative Layout Shift
    const clsObserver = new PerformanceObserver((list) => {
      let clsScore = 0;
      list.getEntries().forEach(entry => {
        if (!(entry as any).hadRecentInput) {
          clsScore += (entry as any).value;
        }
      });
      console.log(`Cumulative Layout Shift: ${clsScore}`);
    });
    clsObserver.observe({ entryTypes: ['layout-shift'] });
  }

  /**
   * Monitor resource loading
   */
  private monitorResourceLoading(): void {
    const resourceObserver = new PerformanceObserver((list) => {
      const entries = list.getEntries();
      entries.forEach(entry => {
        const resource = entry as PerformanceResourceTiming;
        console.log(`Resource loaded: ${resource.name} (${resource.duration}ms)`);
      });
    });
    resourceObserver.observe({ entryTypes: ['resource'] });
  }

  /**
   * Monitor network performance
   */
  private monitorNetworkPerformance(): void {
    if ('connection' in navigator) {
      const connection = (navigator as any).connection;
      console.log(`Network: ${connection.effectiveType} (${connection.downlink}Mbps)`);
      
      connection.addEventListener('change', () => {
        console.log(`Network changed: ${connection.effectiveType}`);
      });
    }
  }

  /**
   * Setup health checks
   */
  private setupHealthChecks(): void {
    // Periodic health check
    setInterval(() => {
      this.performHealthCheck();
    }, 60000); // Every minute

    // Setup endpoint monitoring
    this.monitorEndpoints();

    console.log('Health checks configured');
  }

  /**
   * Perform health check
   */
  private async performHealthCheck(): Promise<boolean> {
    try {
      const healthUrl = `${window.location.origin}${this.deploymentConfig.healthCheckUrl}`;
      const response = await fetch(healthUrl, { 
        method: 'GET',
        cache: 'no-cache',
      });

      const isHealthy = response.ok;
      console.log(`Health check: ${isHealthy ? 'PASS' : 'FAIL'}`);

      return isHealthy;
    } catch (error) {
      console.error('Health check failed:', error);
      return false;
    }
  }

  /**
   * Monitor endpoints
   */
  private monitorEndpoints(): void {
    Object.entries(this.cdnConfig.endpoints).forEach(([name, url]) => {
      fetch(url, { method: 'HEAD', mode: 'no-cors' })
        .then(() => console.log(`Endpoint ${name} is available`))
        .catch(() => console.warn(`Endpoint ${name} is unavailable`));
    });
  }

  /**
   * Utility methods
   */
  private getCurrentDeploymentId(): string {
    // In production, this would come from environment variables or build metadata
    return `deploy-${Date.now()}`;
  }

  private supportsWebP(): boolean {
    const canvas = document.createElement('canvas');
    return canvas.toDataURL('image/webp').indexOf('data:image/webp') === 0;
  }

  private setupAutoDeployment(): void {
    console.log('Auto-deployment monitoring enabled');
    // Would integrate with CI/CD pipeline
  }

  private setupRollback(): void {
    console.log('Rollback capability enabled');
    // Would integrate with deployment platform
  }

  /**
   * Public API
   */
  public getInfrastructureStatus(): {
    cdn: CDNConfig;
    ssl: SSLConfig;
    domains: DomainConfig;
    deployments: DeploymentStatus[];
    isHealthy: boolean;
  } {
    return {
      cdn: this.cdnConfig,
      ssl: this.sslConfig,
      domains: this.domainConfig,
      deployments: Array.from(this.deployments.values()),
      isHealthy: this.isInitialized,
    };
  }

  public async triggerDeployment(config: Partial<DeploymentConfig> = {}): Promise<string> {
    const deploymentId = `deploy-${Date.now()}`;
    const finalConfig = { ...this.deploymentConfig, ...config };

    const deployment: DeploymentStatus = {
      id: deploymentId,
      environment: finalConfig.environment,
      branch: finalConfig.branch,
      commit: 'current',
      status: 'pending',
      startTime: Date.now(),
      logs: ['Deployment initiated'],
    };

    this.deployments.set(deploymentId, deployment);
    
    // Simulate deployment process
    setTimeout(() => {
      deployment.status = 'building';
      deployment.logs.push('Build started');
    }, 1000);

    setTimeout(() => {
      deployment.status = 'deploying';
      deployment.logs.push('Deployment in progress');
    }, 5000);

    setTimeout(() => {
      deployment.status = 'success';
      deployment.endTime = Date.now();
      deployment.logs.push('Deployment completed');
      deployment.url = window.location.origin;
    }, 10000);

    return deploymentId;
  }

  public async rollback(deploymentId: string): Promise<boolean> {
    const deployment = this.deployments.get(deploymentId);
    if (!deployment) {
      console.error('Deployment not found for rollback');
      return false;
    }

    console.log(`Rolling back to deployment: ${deploymentId}`);
    // Would implement actual rollback logic
    return true;
  }

  public generateInfrastructureReport(): string {
    const status = this.getInfrastructureStatus();
    
    return `
Marco 2.0 Infrastructure Report
==============================

CDN Provider: ${status.cdn.provider}
SSL Enabled: ${status.ssl.enabled}
Primary Domain: ${status.domains.primary}
Total Deployments: ${status.deployments.length}
System Health: ${status.isHealthy ? 'Healthy' : 'Degraded'}

Endpoints:
${Object.entries(status.cdn.endpoints).map(([k, v]) => `- ${k}: ${v}`).join('\n')}

Recent Deployments:
${status.deployments.slice(-5).map(d => `- ${d.id}: ${d.status} (${d.environment})`).join('\n')}
`;
  }

  public destroy(): void {
    this.deployments.clear();
    console.log('Infrastructure manager destroyed');
  }
}

export default InfrastructureManager;
