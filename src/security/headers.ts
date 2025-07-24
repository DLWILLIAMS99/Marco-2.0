/**
 * Marco 2.0 - Security Headers Configuration
 * Comprehensive security headers for production deployment
 */

export interface SecurityHeadersConfig {
  enableHSTS: boolean;
  enableFrameOptions: boolean;
  enableContentTypeOptions: boolean;
  enableReferrerPolicy: boolean;
  enablePermissionsPolicy: boolean;
  customHeaders: Record<string, string>;
}

export class SecurityHeaders {
  private config: SecurityHeadersConfig;

  constructor(config?: Partial<SecurityHeadersConfig>) {
    this.config = {
      enableHSTS: true,
      enableFrameOptions: true,
      enableContentTypeOptions: true,
      enableReferrerPolicy: true,
      enablePermissionsPolicy: true,
      customHeaders: {},
      ...config
    };
  }

  public getHeaders(): Record<string, string> {
    const headers: Record<string, string> = {};

    // HTTP Strict Transport Security (HSTS)
    if (this.config.enableHSTS) {
      headers['Strict-Transport-Security'] = 'max-age=63072000; includeSubDomains; preload';
    }

    // X-Frame-Options (Clickjacking protection)
    if (this.config.enableFrameOptions) {
      headers['X-Frame-Options'] = 'DENY';
    }

    // X-Content-Type-Options (MIME sniffing protection)
    if (this.config.enableContentTypeOptions) {
      headers['X-Content-Type-Options'] = 'nosniff';
    }

    // X-XSS-Protection (Legacy XSS protection)
    headers['X-XSS-Protection'] = '1; mode=block';

    // Referrer Policy
    if (this.config.enableReferrerPolicy) {
      headers['Referrer-Policy'] = 'strict-origin-when-cross-origin';
    }

    // Permissions Policy (Feature Policy replacement)
    if (this.config.enablePermissionsPolicy) {
      headers['Permissions-Policy'] = this.generatePermissionsPolicy();
    }

    // Cross-Origin headers for WASM and SharedArrayBuffer
    headers['Cross-Origin-Embedder-Policy'] = 'require-corp';
    headers['Cross-Origin-Opener-Policy'] = 'same-origin';
    headers['Cross-Origin-Resource-Policy'] = 'same-origin';

    // API versioning
    headers['X-API-Version'] = '4.3.0';
    headers['X-Marco2-Version'] = 'Phase-4-Sprint-3';

    // Security contact
    headers['X-Security-Contact'] = 'security@marco2.com';

    // Custom headers
    Object.assign(headers, this.config.customHeaders);

    return headers;
  }

  private generatePermissionsPolicy(): string {
    const policies = [
      'camera=(self)',           // WebRTC camera access
      'microphone=(self)',       // WebRTC microphone access
      'display-capture=(self)',  // Screen sharing
      'geolocation=()',          // Disable geolocation
      'payment=()',              // Disable payment API
      'usb=()',                  // Disable USB API
      'bluetooth=()',            // Disable Bluetooth API
      'magnetometer=()',         // Disable magnetometer
      'gyroscope=()',           // Disable gyroscope
      'accelerometer=()',       // Disable accelerometer
      'ambient-light-sensor=()', // Disable ambient light sensor
      'autoplay=(self)',        // Allow autoplay for WebRTC
      'encrypted-media=(self)',  // Allow encrypted media
      'fullscreen=(self)',      // Allow fullscreen for canvas
      'picture-in-picture=(self)', // Allow PiP for collaboration
      'web-share=(self)'        // Allow web sharing
    ];

    return policies.join(', ');
  }

  public generateMetaTags(): string[] {
    const headers = this.getHeaders();
    const metaTags: string[] = [];

    // Generate meta tags for headers that can be set via HTML
    const metaCompatibleHeaders = [
      'X-XSS-Protection',
      'X-Content-Type-Options',
      'X-Frame-Options',
      'Referrer-Policy',
      'Permissions-Policy',
      'Cross-Origin-Embedder-Policy',
      'Cross-Origin-Opener-Policy'
    ];

    metaCompatibleHeaders.forEach(headerName => {
      if (headers[headerName]) {
        metaTags.push(`<meta http-equiv="${headerName}" content="${headers[headerName]}">`);
      }
    });

    return metaTags;
  }

  public validateHeaders(responseHeaders: Headers): SecurityValidationResult {
    const requiredHeaders = this.getHeaders();
    const validation: SecurityValidationResult = {
      isValid: true,
      missingHeaders: [],
      incorrectHeaders: [],
      recommendations: []
    };

    // Check for missing required headers
    Object.entries(requiredHeaders).forEach(([headerName, expectedValue]) => {
      const actualValue = responseHeaders.get(headerName);
      
      if (!actualValue) {
        validation.missingHeaders.push(headerName);
        validation.isValid = false;
      } else if (actualValue !== expectedValue) {
        validation.incorrectHeaders.push({
          header: headerName,
          expected: expectedValue,
          actual: actualValue
        });
      }
    });

    // Additional security recommendations
    this.generateSecurityRecommendations(responseHeaders, validation);

    return validation;
  }

  private generateSecurityRecommendations(
    headers: Headers, 
    validation: SecurityValidationResult
  ): void {
    // Check for HTTPS
    const protocol = typeof window !== 'undefined' ? window.location.protocol : 'https:';
    if (protocol !== 'https:') {
      validation.recommendations.push('Use HTTPS for all traffic');
    }

    // Check for weak CSP
    const csp = headers.get('Content-Security-Policy');
    if (!csp || csp.includes("'unsafe-eval'") && !csp.includes("'wasm-unsafe-eval'")) {
      validation.recommendations.push('Strengthen Content Security Policy');
    }

    // Check for HSTS preload
    const hsts = headers.get('Strict-Transport-Security');
    if (hsts && !hsts.includes('preload')) {
      validation.recommendations.push('Consider HSTS preloading');
    }

    // Check for security.txt
    if (!headers.get('X-Security-Contact')) {
      validation.recommendations.push('Implement security.txt for vulnerability disclosure');
    }
  }

  public setupExpressMiddleware(): (req: any, res: any, next: any) => void {
    const headers = this.getHeaders();
    
    return (req: any, res: any, next: any) => {
      // Apply security headers to all responses
      Object.entries(headers).forEach(([name, value]) => {
        res.setHeader(name, value);
      });

      // Special handling for API routes
      if (req.path.startsWith('/api/')) {
        res.setHeader('X-Content-Type-Options', 'nosniff');
        res.setHeader('Cache-Control', 'no-store, no-cache, must-revalidate');
      }

      // Special handling for WebSocket upgrades
      if (req.headers.upgrade === 'websocket') {
        res.setHeader('Cross-Origin-Resource-Policy', 'cross-origin');
      }

      next();
    };
  }

  public setupServiceWorkerHeaders(): Record<string, string> {
    return {
      'Cache-Control': 'no-cache, no-store, must-revalidate',
      'Pragma': 'no-cache',
      'Expires': '0',
      'X-Content-Type-Options': 'nosniff',
      'X-Frame-Options': 'DENY'
    };
  }

  public setupWASMHeaders(): Record<string, string> {
    return {
      'Content-Type': 'application/wasm',
      'Cross-Origin-Embedder-Policy': 'require-corp',
      'Cross-Origin-Opener-Policy': 'same-origin',
      'Cache-Control': 'public, max-age=31536000, immutable',
      'Vary': 'Accept-Encoding'
    };
  }

  public setupAPIHeaders(): Record<string, string> {
    return {
      'X-Content-Type-Options': 'nosniff',
      'X-Frame-Options': 'DENY',
      'Cache-Control': 'no-store, no-cache, must-revalidate',
      'Pragma': 'no-cache',
      'X-API-Version': '4.3.0',
      'Access-Control-Allow-Origin': 'https://marco2.com',
      'Access-Control-Allow-Methods': 'GET, POST, PUT, DELETE, OPTIONS',
      'Access-Control-Allow-Headers': 'Content-Type, Authorization, X-Requested-With',
      'Access-Control-Max-Age': '86400'
    };
  }
}

export interface SecurityValidationResult {
  isValid: boolean;
  missingHeaders: string[];
  incorrectHeaders: Array<{
    header: string;
    expected: string;
    actual: string;
  }>;
  recommendations: string[];
}

// CORS Configuration for Marco 2.0
export class CORSPolicy {
  private allowedOrigins: string[];
  private allowedMethods: string[];
  private allowedHeaders: string[];
  private maxAge: number;

  constructor() {
    this.allowedOrigins = [
      'https://marco2.com',
      'https://www.marco2.com',
      'https://api.marco2.com',
      'https://cdn.marco2.com'
    ];

    this.allowedMethods = [
      'GET',
      'POST',
      'PUT',
      'DELETE',
      'OPTIONS',
      'PATCH'
    ];

    this.allowedHeaders = [
      'Content-Type',
      'Authorization',
      'X-Requested-With',
      'X-API-Version',
      'X-Client-Version',
      'Accept',
      'Origin',
      'DNT',
      'User-Agent',
      'If-Modified-Since',
      'Cache-Control',
      'Range'
    ];

    this.maxAge = 86400; // 24 hours
  }

  public generateCORSHeaders(origin?: string): Record<string, string> {
    const headers: Record<string, string> = {};

    // Check if origin is allowed
    if (origin && this.isOriginAllowed(origin)) {
      headers['Access-Control-Allow-Origin'] = origin;
    } else if (this.allowedOrigins.length === 1) {
      headers['Access-Control-Allow-Origin'] = this.allowedOrigins[0];
    }

    headers['Access-Control-Allow-Methods'] = this.allowedMethods.join(', ');
    headers['Access-Control-Allow-Headers'] = this.allowedHeaders.join(', ');
    headers['Access-Control-Max-Age'] = this.maxAge.toString();
    headers['Access-Control-Allow-Credentials'] = 'true';
    headers['Vary'] = 'Origin';

    return headers;
  }

  private isOriginAllowed(origin: string): boolean {
    return this.allowedOrigins.includes(origin) || 
           (origin.startsWith('http://localhost:') && this.isDevelopment());
  }

  private isDevelopment(): boolean {
    return typeof window !== 'undefined' && 
           (window.location.hostname === 'localhost' || 
            window.location.hostname === '127.0.0.1');
  }

  public handlePreflightRequest(): Record<string, string> {
    return {
      ...this.generateCORSHeaders(),
      'Content-Length': '0',
      'Content-Type': 'text/plain charset=UTF-8'
    };
  }
}

// Security monitoring and alerting
export class SecurityMonitor {
  private violations: Map<string, number> = new Map();
  private alertThreshold: number = 10;
  private timeWindow: number = 60000; // 1 minute

  public recordViolation(type: string): void {
    const current = this.violations.get(type) || 0;
    this.violations.set(type, current + 1);

    // Check if threshold exceeded
    if (current + 1 >= this.alertThreshold) {
      this.triggerSecurityAlert(type, current + 1);
      this.violations.set(type, 0); // Reset counter
    }

    // Clean up old violations
    setTimeout(() => {
      const count = this.violations.get(type) || 0;
      if (count > 0) {
        this.violations.set(type, count - 1);
      }
    }, this.timeWindow);
  }

  private triggerSecurityAlert(violationType: string, count: number): void {
    console.error(`Security Alert: ${count} ${violationType} violations in ${this.timeWindow}ms`);
    
    // Send alert to monitoring system
    if (typeof fetch !== 'undefined') {
      fetch('/api/security/alert', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          type: violationType,
          count,
          timestamp: Date.now(),
          userAgent: typeof navigator !== 'undefined' ? navigator.userAgent : 'unknown',
          url: typeof window !== 'undefined' ? window.location.href : 'unknown'
        })
      }).catch(error => {
        console.error('Failed to send security alert:', error);
      });
    }
  }

  public getViolationStats(): Record<string, number> {
    return Object.fromEntries(this.violations);
  }
}

// Global instances
export const securityHeaders = new SecurityHeaders();
export const corsPolicy = new CORSPolicy();
export const securityMonitor = new SecurityMonitor();
