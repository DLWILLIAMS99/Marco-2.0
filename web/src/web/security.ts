/**
 * Security Hardening for Marco 2.0 Production
 * 
 * Content Security Policy, secure headers, and security best practices
 */

export interface SecurityConfig {
  enableCSP: boolean;
  enableHSTS: boolean;
  enableCORS: boolean;
  allowedOrigins: string[];
  trustedDomains: string[];
  reportingEndpoint?: string;
  enforceSecureContext: boolean;
  enableIntegrityChecks: boolean;
}

export interface SecurityViolation {
  type: 'csp' | 'cors' | 'mixed-content' | 'integrity' | 'protocol';
  message: string;
  source: string;
  timestamp: number;
  userAgent: string;
  url: string;
}

export interface IntegrityCheck {
  resource: string;
  expectedHash: string;
  algorithm: 'sha256' | 'sha384' | 'sha512';
  verified: boolean;
  error?: string;
}

export class SecurityManager {
  private config: SecurityConfig;
  private violations: SecurityViolation[] = [];
  private integrityChecks: Map<string, IntegrityCheck> = new Map();
  private isInitialized = false;

  private readonly DEFAULT_CONFIG: SecurityConfig = {
    enableCSP: true,
    enableHSTS: true,
    enableCORS: true,
    allowedOrigins: ['https://marco2.app', 'https://api.marco2.app'],
    trustedDomains: ['marco2.app', 'api.marco2.app', 'cdn.marco2.app'],
    enforceSecureContext: true,
    enableIntegrityChecks: true,
  };

  constructor(config: Partial<SecurityConfig> = {}) {
    this.config = { ...this.DEFAULT_CONFIG, ...config };
    this.initialize();
  }

  /**
   * Initialize security measures
   */
  private async initialize(): Promise<void> {
    try {
      // Check secure context
      if (this.config.enforceSecureContext) {
        this.enforceSecureContext();
      }

      // Setup Content Security Policy
      if (this.config.enableCSP) {
        this.setupCSP();
      }

      // Setup security headers validation
      if (this.config.enableHSTS || this.config.enableCORS) {
        this.validateSecurityHeaders();
      }

      // Setup integrity checks
      if (this.config.enableIntegrityChecks) {
        this.setupIntegrityChecks();
      }

      // Setup security event listeners
      this.setupSecurityEventListeners();

      // Setup input sanitization
      this.setupInputSanitization();

      // Setup secure storage
      this.setupSecureStorage();

      this.isInitialized = true;
      console.log('Security manager initialized');

    } catch (error) {
      console.error('Failed to initialize security manager:', error);
      this.reportViolation('protocol', 'Security initialization failed', window.location.href);
    }
  }

  /**
   * Enforce secure context (HTTPS)
   */
  private enforceSecureContext(): void {
    if (!window.isSecureContext && location.protocol !== 'https:' && location.hostname !== 'localhost') {
      const httpsUrl = window.location.href.replace('http:', 'https:');
      console.warn('Redirecting to secure context:', httpsUrl);
      window.location.replace(httpsUrl);
      return;
    }

    // Check for mixed content
    if (window.isSecureContext) {
      this.detectMixedContent();
    }
  }

  /**
   * Setup Content Security Policy
   */
  private setupCSP(): void {
    const cspDirectives: Record<string, string[]> = {
      'default-src': ["'self'"],
      'script-src': [
        "'self'",
        "'wasm-unsafe-eval'", // Required for WASM
        ...this.config.trustedDomains.map(domain => `https://${domain}`),
      ],
      'style-src': [
        "'self'",
        "'unsafe-inline'", // Required for dynamic styling
        ...this.config.trustedDomains.map(domain => `https://${domain}`),
      ],
      'img-src': [
        "'self'",
        'data:', // For base64 images
        'blob:', // For generated images
        ...this.config.trustedDomains.map(domain => `https://${domain}`),
      ],
      'connect-src': [
        "'self'",
        'wss:', // For WebSocket connections
        ...this.config.allowedOrigins,
      ],
      'font-src': [
        "'self'",
        'data:', // For inline fonts
        ...this.config.trustedDomains.map(domain => `https://${domain}`),
      ],
      'media-src': ["'self'", 'blob:'],
      'worker-src': ["'self'", 'blob:'],
      'frame-src': ["'none'"],
      'object-src': ["'none'"],
      'base-uri': ["'self'"],
      'form-action': ["'self'"],
    };

    // Add reporting if endpoint is configured
    if (this.config.reportingEndpoint) {
      cspDirectives['report-uri'] = [this.config.reportingEndpoint];
    }

    const cspString = Object.entries(cspDirectives)
      .map(([directive, sources]) => `${directive} ${sources.join(' ')}`)
      .join('; ');

    // Set CSP meta tag if not already set by server
    if (!document.querySelector('meta[http-equiv="Content-Security-Policy"]')) {
      const meta = document.createElement('meta');
      meta.httpEquiv = 'Content-Security-Policy';
      meta.content = cspString;
      document.head.appendChild(meta);
    }

    console.log('Content Security Policy configured');
  }

  /**
   * Validate security headers
   */
  private validateSecurityHeaders(): void {
    // Check if running in production
    if (window.location.hostname === 'localhost' || window.location.hostname === '127.0.0.1') {
      return; // Skip header validation in development
    }

    fetch(window.location.href, { method: 'HEAD' })
      .then(response => {
        const headers = response.headers;
        
        // Check HSTS
        if (this.config.enableHSTS && !headers.has('strict-transport-security')) {
          this.reportViolation('protocol', 'Missing HSTS header', window.location.href);
        }

        // Check other security headers
        const requiredHeaders = [
          'x-content-type-options',
          'x-frame-options',
          'x-xss-protection',
          'referrer-policy',
        ];

        requiredHeaders.forEach(header => {
          if (!headers.has(header)) {
            console.warn(`Missing security header: ${header}`);
          }
        });
      })
      .catch(error => {
        console.warn('Could not validate security headers:', error);
      });
  }

  /**
   * Setup integrity checks for critical resources
   */
  private setupIntegrityChecks(): void {
    const criticalResources = [
      { selector: 'script[src]', type: 'script' },
      { selector: 'link[rel="stylesheet"]', type: 'stylesheet' },
    ];

    criticalResources.forEach(({ selector, type }) => {
      document.querySelectorAll(selector).forEach((element: Element) => {
        const src = element.getAttribute('src') || element.getAttribute('href');
        if (src && this.isExternalResource(src)) {
          this.checkResourceIntegrity(element as HTMLElement, src, type);
        }
      });
    });
  }

  /**
   * Check resource integrity
   */
  private async checkResourceIntegrity(element: HTMLElement, src: string, type: string): Promise<void> {
    const integrity = element.getAttribute('integrity');
    
    if (!integrity) {
      console.warn(`Missing integrity attribute for ${type}: ${src}`);
      return;
    }

    try {
      const response = await fetch(src);
      const content = await response.text();
      const hash = await this.calculateHash(content, 'sha384');
      const expectedHash = integrity.replace('sha384-', '');

      const check: IntegrityCheck = {
        resource: src,
        expectedHash,
        algorithm: 'sha384',
        verified: hash === expectedHash,
      };

      if (!check.verified) {
        check.error = 'Hash mismatch';
        this.reportViolation('integrity', `Integrity check failed for ${src}`, src);
      }

      this.integrityChecks.set(src, check);
    } catch (error) {
      console.error(`Integrity check failed for ${src}:`, error);
      this.reportViolation('integrity', `Integrity check error for ${src}`, src);
    }
  }

  /**
   * Setup security event listeners
   */
  private setupSecurityEventListeners(): void {
    // CSP violation reporting
    document.addEventListener('securitypolicyviolation', (event) => {
      this.reportViolation('csp', event.violatedDirective, event.sourceFile || event.documentURI);
    });

    // Mixed content detection
    if ('SecurityPolicyViolationEvent' in window) {
      window.addEventListener('securitypolicyviolation', (event) => {
        if (event.violatedDirective.includes('mixed-content')) {
          this.reportViolation('mixed-content', 'Mixed content detected', event.sourceFile || '');
        }
      });
    }

    // Monitor for dangerous APIs
    this.monitorDangerousAPIs();
  }

  /**
   * Setup input sanitization
   */
  private setupInputSanitization(): void {
    // Override innerHTML to prevent XSS
    const originalInnerHTML = Object.getOwnPropertyDescriptor(Element.prototype, 'innerHTML')!;
    
    Object.defineProperty(Element.prototype, 'innerHTML', {
      set(value: string) {
        const sanitized = this.sanitizeHTML(value);
        if (sanitized !== value) {
          console.warn('Potentially unsafe HTML sanitized');
        }
        originalInnerHTML.set!.call(this, sanitized);
      },
      get: originalInnerHTML.get,
    });

    // Add input validation helpers
    this.addInputValidationHelpers();
  }

  /**
   * Setup secure storage
   */
  private setupSecureStorage(): void {
    // Encrypt sensitive data in localStorage/sessionStorage
    this.wrapStorageAPIs();
    
    // Clear sensitive data on page unload
    window.addEventListener('beforeunload', () => {
      this.clearSensitiveData();
    });
  }

  /**
   * Detect mixed content
   */
  private detectMixedContent(): void {
    const insecureElements = [
      { selector: 'script[src^="http:"]', type: 'script' },
      { selector: 'link[href^="http:"]', type: 'stylesheet' },
      { selector: 'img[src^="http:"]', type: 'image' },
      { selector: 'video[src^="http:"]', type: 'video' },
      { selector: 'audio[src^="http:"]', type: 'audio' },
    ];

    insecureElements.forEach(({ selector, type }) => {
      const elements = document.querySelectorAll(selector);
      if (elements.length > 0) {
        this.reportViolation('mixed-content', `Insecure ${type} resource detected`, selector);
      }
    });
  }

  /**
   * Monitor dangerous APIs
   */
  private monitorDangerousAPIs(): void {
    const dangerousAPIs = ['eval', 'Function', 'setTimeout', 'setInterval'];
    
    dangerousAPIs.forEach(api => {
      const original = (window as any)[api];
      (window as any)[api] = (...args: any[]) => {
        if (typeof args[0] === 'string') {
          console.warn(`Potentially dangerous API usage: ${api} with string argument`);
          this.reportViolation('protocol', `Dangerous API usage: ${api}`, window.location.href);
        }
        return original.apply(window, args);
      };
    });
  }

  /**
   * Sanitize HTML content
   */
  private sanitizeHTML(html: string): string {
    // Basic XSS prevention - in production, use a library like DOMPurify
    const dangerousPatterns = [
      /<script\b[^<]*(?:(?!<\/script>)<[^<]*)*<\/script>/gi,
      /javascript:/gi,
      /on\w+\s*=/gi,
      /<iframe\b[^<]*(?:(?!<\/iframe>)<[^<]*)*<\/iframe>/gi,
      /<object\b[^<]*(?:(?!<\/object>)<[^<]*)*<\/object>/gi,
      /<embed\b[^<]*(?:(?!<\/embed>)<[^<]*)*<\/embed>/gi,
    ];

    let sanitized = html;
    dangerousPatterns.forEach(pattern => {
      sanitized = sanitized.replace(pattern, '');
    });

    return sanitized;
  }

  /**
   * Add input validation helpers
   */
  private addInputValidationHelpers(): void {
    // URL validation
    (window as any).validateURL = (url: string): boolean => {
      try {
        const parsed = new URL(url);
        return this.config.trustedDomains.some(domain => 
          parsed.hostname === domain || parsed.hostname.endsWith(`.${domain}`)
        );
      } catch {
        return false;
      }
    };

    // HTML validation
    (window as any).validateHTML = (html: string): boolean => {
      return this.sanitizeHTML(html) === html;
    };
  }

  /**
   * Wrap storage APIs for encryption
   */
  private wrapStorageAPIs(): void {
    const storages = [localStorage, sessionStorage];
    
    storages.forEach(storage => {
      const originalSetItem = storage.setItem.bind(storage);
      const originalGetItem = storage.getItem.bind(storage);

      storage.setItem = (key: string, value: string) => {
        if (this.isSensitiveKey(key)) {
          value = this.encryptValue(value);
        }
        originalSetItem(key, value);
      };

      storage.getItem = (key: string) => {
        const value = originalGetItem(key);
        if (value && this.isSensitiveKey(key)) {
          return this.decryptValue(value);
        }
        return value;
      };
    });
  }

  /**
   * Check if resource is external
   */
  private isExternalResource(src: string): boolean {
    try {
      const url = new URL(src, window.location.href);
      return url.origin !== window.location.origin;
    } catch {
      return false;
    }
  }

  /**
   * Calculate hash for integrity checking
   */
  private async calculateHash(content: string, algorithm: string): Promise<string> {
    const encoder = new TextEncoder();
    const data = encoder.encode(content);
    const hashBuffer = await crypto.subtle.digest(algorithm.toUpperCase(), data);
    const hashArray = Array.from(new Uint8Array(hashBuffer));
    return btoa(String.fromCharCode(...hashArray));
  }

  /**
   * Check if storage key contains sensitive data
   */
  private isSensitiveKey(key: string): boolean {
    const sensitivePatterns = ['token', 'password', 'secret', 'key', 'auth', 'session'];
    return sensitivePatterns.some(pattern => key.toLowerCase().includes(pattern));
  }

  /**
   * Basic encryption for storage (in production, use a proper crypto library)
   */
  private encryptValue(value: string): string {
    // Simple XOR encryption for demo - use proper encryption in production
    const key = 'marco2-security-key';
    let encrypted = '';
    
    for (let i = 0; i < value.length; i++) {
      encrypted += String.fromCharCode(value.charCodeAt(i) ^ key.charCodeAt(i % key.length));
    }
    
    return btoa(encrypted);
  }

  /**
   * Basic decryption for storage
   */
  private decryptValue(encryptedValue: string): string {
    try {
      const key = 'marco2-security-key';
      const encrypted = atob(encryptedValue);
      let decrypted = '';
      
      for (let i = 0; i < encrypted.length; i++) {
        decrypted += String.fromCharCode(encrypted.charCodeAt(i) ^ key.charCodeAt(i % key.length));
      }
      
      return decrypted;
    } catch {
      return encryptedValue; // Return as-is if decryption fails
    }
  }

  /**
   * Clear sensitive data
   */
  private clearSensitiveData(): void {
    const storages = [localStorage, sessionStorage];
    
    storages.forEach(storage => {
      Object.keys(storage).forEach(key => {
        if (this.isSensitiveKey(key)) {
          storage.removeItem(key);
        }
      });
    });
  }

  /**
   * Report security violation
   */
  private reportViolation(type: SecurityViolation['type'], message: string, source: string): void {
    const violation: SecurityViolation = {
      type,
      message,
      source,
      timestamp: Date.now(),
      userAgent: navigator.userAgent,
      url: window.location.href,
    };

    this.violations.push(violation);
    console.error('Security violation:', violation);

    // Report to analytics if available
    if ((window as any).analytics) {
      (window as any).analytics.trackEvent({
        type: 'error',
        category: 'security',
        action: type,
        label: message,
        metadata: violation,
      });
    }

    // Send to reporting endpoint if configured
    if (this.config.reportingEndpoint) {
      this.sendViolationReport(violation);
    }
  }

  /**
   * Send violation report to endpoint
   */
  private async sendViolationReport(violation: SecurityViolation): Promise<void> {
    try {
      await fetch(this.config.reportingEndpoint!, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(violation),
      });
    } catch (error) {
      console.error('Failed to send violation report:', error);
    }
  }

  /**
   * Get security status
   */
  public getSecurityStatus(): {
    isSecure: boolean;
    violations: SecurityViolation[];
    integrityChecks: IntegrityCheck[];
    config: SecurityConfig;
  } {
    return {
      isSecure: this.violations.length === 0 && window.isSecureContext,
      violations: [...this.violations],
      integrityChecks: Array.from(this.integrityChecks.values()),
      config: this.config,
    };
  }

  /**
   * Generate security report
   */
  public generateSecurityReport(): string {
    const status = this.getSecurityStatus();
    
    return `
Marco 2.0 Security Report
========================

Secure Context: ${window.isSecureContext ? 'Yes' : 'No'}
Total Violations: ${status.violations.length}
Integrity Checks: ${status.integrityChecks.length}

Violations:
${status.violations.map(v => `- ${v.type}: ${v.message}`).join('\n')}

Integrity Checks:
${status.integrityChecks.map(c => `- ${c.resource}: ${c.verified ? 'PASS' : 'FAIL'}`).join('\n')}

Configuration:
${Object.entries(status.config).map(([k, v]) => `- ${k}: ${v}`).join('\n')}
`;
  }

  /**
   * Cleanup
   */
  public destroy(): void {
    // Clear violation history
    this.violations = [];
    this.integrityChecks.clear();
    
    console.log('Security manager destroyed');
  }
}

export default SecurityManager;
