/**
 * Marco 2.0 - Content Security Policy Configuration
 * Strict CSP for WASM, WebRTC, and production security
 */

export interface CSPConfig {
  directives: Record<string, string[]>;
  reportUri?: string;
  enforceMode: boolean;
  nonce?: string;
}

export class ContentSecurityPolicy {
  private config: CSPConfig;
  private nonce: string;

  constructor(config?: Partial<CSPConfig>) {
    this.nonce = this.generateNonce();
    
    this.config = {
      enforceMode: true,
      reportUri: '/api/security/csp-report',
      nonce: this.nonce,
      directives: {
        'default-src': ["'self'"],
        'script-src': [
          "'self'",
          "'unsafe-inline'", // Required for some WASM initialization
          "'wasm-unsafe-eval'", // Required for WASM execution
          `'nonce-${this.nonce}'`
        ],
        'worker-src': [
          "'self'",
          "blob:" // Required for Web Workers with WASM
        ],
        'connect-src': [
          "'self'",
          "wss:",
          "ws:",
          "https:", // WebRTC STUN/TURN servers
          "*.marco2.com",
          "api.marco2.com",
          "ws.marco2.com"
        ],
        'media-src': [
          "'self'",
          "blob:", // Required for WebRTC media streams
          "mediastream:" // Required for getUserMedia
        ],
        'img-src': [
          "'self'",
          "data:",
          "blob:", // Required for canvas image exports
          "https:"
        ],
        'style-src': [
          "'self'",
          "'unsafe-inline'" // Required for dynamic Canvas styling
        ],
        'font-src': [
          "'self'",
          "data:",
          "https://fonts.gstatic.com"
        ],
        'object-src': ["'none'"],
        'base-uri': ["'self'"],
        'form-action': ["'self'"],
        'frame-ancestors': ["'none'"],
        'manifest-src': ["'self'"],
        'child-src': ["'self'", "blob:"],
        'frame-src': ["'none'"]
      },
      ...config
    };
  }

  private generateNonce(): string {
    const array = new Uint8Array(16);
    crypto.getRandomValues(array);
    return btoa(String.fromCharCode(...array)).replace(/[+/=]/g, '');
  }

  public generateCSPHeader(): string {
    const directives = Object.entries(this.config.directives)
      .map(([key, values]) => `${key} ${values.join(' ')}`)
      .join('; ');

    const reportDirective = this.config.reportUri 
      ? `; report-uri ${this.config.reportUri}` 
      : '';

    return `${directives}${reportDirective}`;
  }

  public getMetaTag(): string {
    const content = this.generateCSPHeader();
    const httpEquiv = this.config.enforceMode 
      ? 'Content-Security-Policy' 
      : 'Content-Security-Policy-Report-Only';

    return `<meta http-equiv="${httpEquiv}" content="${content}">`;
  }

  public getNonce(): string {
    return this.nonce;
  }

  public validateScript(scriptContent: string): boolean {
    // Basic validation for inline scripts
    const dangerousPatterns = [
      /eval\s*\(/,
      /Function\s*\(/,
      /document\.write/,
      /innerHTML\s*=/,
      /outerHTML\s*=/,
      /insertAdjacentHTML/,
      /execScript/,
      /setTimeout\s*\(\s*["']/,
      /setInterval\s*\(\s*["']/
    ];

    return !dangerousPatterns.some(pattern => pattern.test(scriptContent));
  }

  public addDynamicSource(directive: string, source: string): void {
    if (this.config.directives[directive]) {
      if (!this.config.directives[directive].includes(source)) {
        this.config.directives[directive].push(source);
      }
    }
  }

  public removeDynamicSource(directive: string, source: string): void {
    if (this.config.directives[directive]) {
      this.config.directives[directive] = this.config.directives[directive]
        .filter(s => s !== source);
    }
  }

  public handleViolation(violationReport: any): void {
    console.warn('CSP Violation:', violationReport);
    
    // Send violation report to analytics
    if (typeof window !== 'undefined' && typeof fetch !== 'undefined') {
      fetch('/api/security/csp-violation', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          violation: violationReport,
          timestamp: Date.now(),
          userAgent: navigator.userAgent,
          url: window.location.href
        })
      }).catch(error => {
        console.error('Failed to report CSP violation:', error);
      });
    }
  }

  public enableReporting(): void {
    if (typeof document !== 'undefined') {
      document.addEventListener('securitypolicyviolation', (event) => {
        this.handleViolation({
          blockedURI: event.blockedURI,
          documentURI: event.documentURI,
          effectiveDirective: event.effectiveDirective,
          originalPolicy: event.originalPolicy,
          referrer: event.referrer,
          statusCode: event.statusCode,
          violatedDirective: event.violatedDirective,
          lineNumber: event.lineNumber,
          columnNumber: event.columnNumber,
          sourceFile: event.sourceFile
        });
      });
    }
  }

  public updateForWebRTC(): void {
    // Add WebRTC-specific CSP directives
    this.addDynamicSource('connect-src', 'stun:');
    this.addDynamicSource('connect-src', 'turn:');
    this.addDynamicSource('connect-src', 'turns:');
    
    // Allow specific STUN/TURN servers
    const stunServers = [
      'stun.l.google.com:19302',
      'stun1.l.google.com:19302',
      'stun2.l.google.com:19302',
      'stun3.l.google.com:19302',
      'stun4.l.google.com:19302'
    ];

    stunServers.forEach(server => {
      this.addDynamicSource('connect-src', `stun:${server}`);
    });
  }

  public updateForWASM(): void {
    // Ensure WASM-specific directives are properly configured
    this.addDynamicSource('script-src', "'wasm-unsafe-eval'");
    
    // Allow WASM module imports
    this.addDynamicSource('script-src', "'unsafe-inline'");
    
    // Allow SharedArrayBuffer if needed
    if (typeof SharedArrayBuffer !== 'undefined') {
      this.addDynamicSource('script-src', "'unsafe-eval'");
    }
  }

  public generateTrustedTypesPolicy(): any | null {
    if (typeof window !== 'undefined' && (window as any).trustedTypes) {
      return (window as any).trustedTypes.createPolicy('marco2-policy', {
        createHTML: (input: string) => {
          // Sanitize HTML input for Marco 2.0
          return input
            .replace(/<script[^>]*>.*?<\/script>/gi, '')
            .replace(/on\w+\s*=/gi, '')
            .replace(/javascript:/gi, '');
        },
        createScript: (input: string) => {
          // Validate scripts before execution
          if (this.validateScript(input)) {
            return input;
          }
          throw new Error('Script content violates security policy');
        },
        createScriptURL: (input: string) => {
          // Validate script URLs
          const url = new URL(input, window.location.origin);
          if (url.origin === window.location.origin || 
              url.origin === 'https://cdn.marco2.com') {
            return input;
          }
          throw new Error('Script URL not allowed by security policy');
        }
      });
    }
    return null;
  }
}

// WASM-specific CSP configuration
export class WASMSecurityPolicy {
  private csp: ContentSecurityPolicy;
  
  constructor(csp: ContentSecurityPolicy) {
    this.csp = csp;
  }

  public configureForWASM(): void {
    // Configure CSP for WASM module loading
    this.csp.updateForWASM();
    
    // Set up Cross-Origin-Embedder-Policy headers (required for SharedArrayBuffer)
    this.setupCrossOriginHeaders();
  }

  private setupCrossOriginHeaders(): void {
    if (typeof document !== 'undefined') {
      const meta1 = document.createElement('meta');
      meta1.httpEquiv = 'Cross-Origin-Embedder-Policy';
      meta1.content = 'require-corp';
      document.head.appendChild(meta1);

      const meta2 = document.createElement('meta');
      meta2.httpEquiv = 'Cross-Origin-Opener-Policy';
      meta2.content = 'same-origin';
      document.head.appendChild(meta2);
    }
  }

  public validateWASMModule(wasmBytes: ArrayBuffer): boolean {
    // Basic WASM module validation
    const magicNumber = new Uint32Array(wasmBytes.slice(0, 4))[0];
    const version = new Uint32Array(wasmBytes.slice(4, 8))[0];
    
    return magicNumber === 0x6d736100 && version === 0x1; // WASM magic number and version
  }
}

// WebRTC-specific CSP configuration
export class WebRTCSecurityPolicy {
  private csp: ContentSecurityPolicy;
  
  constructor(csp: ContentSecurityPolicy) {
    this.csp = csp;
  }

  public configureForWebRTC(): void {
    this.csp.updateForWebRTC();
    
    // Add media capture permissions
    this.setupMediaPermissions();
  }

  private setupMediaPermissions(): void {
    // Configure Permissions Policy for WebRTC
    if (typeof document !== 'undefined') {
      const meta = document.createElement('meta');
      meta.httpEquiv = 'Permissions-Policy';
      meta.content = 'camera=(self), microphone=(self), display-capture=(self)';
      document.head.appendChild(meta);
    }
  }

  public validateICEServer(server: RTCIceServer): boolean {
    const allowedUrls = server.urls;
    const urlArray = Array.isArray(allowedUrls) ? allowedUrls : [allowedUrls];
    
    return urlArray.every(url => {
      const urlObj = new URL(url);
      // Allow Google STUN servers and our own TURN servers
      return urlObj.protocol === 'stun:' || 
             urlObj.protocol === 'turn:' || 
             urlObj.protocol === 'turns:' ||
             urlObj.hostname.endsWith('.google.com') ||
             urlObj.hostname.endsWith('.marco2.com');
    });
  }
}

// Global CSP instance for Marco 2.0
export const marco2CSP = new ContentSecurityPolicy({
  enforceMode: typeof window !== 'undefined' ? window.location.hostname !== 'localhost' : true,
  reportUri: '/api/security/csp-report'
});

// Initialize specialized security policies
export const wasmSecurity = new WASMSecurityPolicy(marco2CSP);
export const webrtcSecurity = new WebRTCSecurityPolicy(marco2CSP);

// Setup CSP on page load
if (typeof document !== 'undefined') {
  document.addEventListener('DOMContentLoaded', () => {
    marco2CSP.enableReporting();
    wasmSecurity.configureForWASM();
    webrtcSecurity.configureForWebRTC();
    
    // Create Trusted Types policy
    marco2CSP.generateTrustedTypesPolicy();
    
    console.log('Marco 2.0 Security Policies initialized');
  });
}
