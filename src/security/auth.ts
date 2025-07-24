/**
 * Marco 2.0 - Authentication System
 * WebRTC peer authentication and session management
 */

export interface UserCredentials {
  userId: string;
  username: string;
  email?: string;
  avatar?: string;
  permissions: string[];
}

export interface SessionToken {
  token: string;
  expiresAt: number;
  userId: string;
  sessionId: string;
  scopes: string[];
}

export interface AuthConfig {
  tokenLifetime: number;
  refreshThreshold: number;
  maxSessions: number;
  requireEmailVerification: boolean;
  enableMFA: boolean;
}

export class AuthenticationManager {
  private config: AuthConfig;
  private activeSessions: Map<string, SessionToken> = new Map();
  private refreshTokens: Map<string, string> = new Map();
  private loginAttempts: Map<string, { count: number; lastAttempt: number }> = new Map();
  private maxLoginAttempts: number = 5;
  private lockoutDuration: number = 300000; // 5 minutes

  constructor(config?: Partial<AuthConfig>) {
    this.config = {
      tokenLifetime: 3600000, // 1 hour
      refreshThreshold: 300000, // 5 minutes before expiry
      maxSessions: 5,
      requireEmailVerification: true,
      enableMFA: false,
      ...config
    };
  }

  public async authenticate(
    username: string, 
    password: string,
    mfaCode?: string
  ): Promise<{ success: boolean; token?: SessionToken; error?: string }> {
    // Check for account lockout
    if (this.isAccountLocked(username)) {
      return {
        success: false,
        error: 'Account temporarily locked due to too many failed attempts'
      };
    }

    try {
      // Validate credentials (this would typically call an external service)
      const credentials = await this.validateCredentials(username, password);
      
      if (!credentials) {
        this.recordFailedAttempt(username);
        return { success: false, error: 'Invalid credentials' };
      }

      // Check MFA if enabled
      if (this.config.enableMFA && !this.validateMFA(credentials.userId, mfaCode)) {
        return { success: false, error: 'Invalid MFA code' };
      }

      // Check email verification
      if (this.config.requireEmailVerification && !credentials.email) {
        return { success: false, error: 'Email verification required' };
      }

      // Create session token
      const token = this.createSessionToken(credentials);
      
      // Clean up old sessions if needed
      this.cleanupUserSessions(credentials.userId);
      
      // Store session
      this.activeSessions.set(token.sessionId, token);
      
      // Clear failed attempts
      this.loginAttempts.delete(username);
      
      return { success: true, token };

    } catch (error) {
      this.recordFailedAttempt(username);
      return { 
        success: false, 
        error: error instanceof Error ? error.message : 'Authentication failed' 
      };
    }
  }

  private async validateCredentials(
    username: string, 
    password: string
  ): Promise<UserCredentials | null> {
    // In a real implementation, this would:
    // 1. Hash the password with salt
    // 2. Query the user database
    // 3. Compare hashed passwords
    // 4. Return user credentials if valid

    // Mock implementation for development
    if (username === 'demo' && password === 'demo123') {
      return {
        userId: 'user_demo_001',
        username: 'demo',
        email: 'demo@marco2.com',
        avatar: '/avatars/demo.png',
        permissions: ['canvas.create', 'canvas.edit', 'collaboration.join']
      };
    }

    return null;
  }

  private validateMFA(userId: string, mfaCode?: string): boolean {
    // Mock MFA validation
    // In production, this would validate TOTP, SMS, or other MFA methods
    return mfaCode === '123456';
  }

  private createSessionToken(credentials: UserCredentials): SessionToken {
    const sessionId = this.generateSessionId();
    const token = this.generateJWT(credentials, sessionId);
    const expiresAt = Date.now() + this.config.tokenLifetime;

    return {
      token,
      expiresAt,
      userId: credentials.userId,
      sessionId,
      scopes: credentials.permissions
    };
  }

  private generateSessionId(): string {
    const array = new Uint8Array(32);
    crypto.getRandomValues(array);
    return btoa(String.fromCharCode(...array)).replace(/[+/=]/g, '');
  }

  private generateJWT(credentials: UserCredentials, sessionId: string): string {
    // Simple JWT implementation (use a proper library in production)
    const header = {
      typ: 'JWT',
      alg: 'HS256'
    };

    const payload = {
      sub: credentials.userId,
      username: credentials.username,
      sid: sessionId,
      iat: Math.floor(Date.now() / 1000),
      exp: Math.floor((Date.now() + this.config.tokenLifetime) / 1000),
      scopes: credentials.permissions
    };

    const encodedHeader = btoa(JSON.stringify(header));
    const encodedPayload = btoa(JSON.stringify(payload));
    
    // In production, use a proper HMAC signature
    const signature = btoa(`${encodedHeader}.${encodedPayload}.SECRET`);
    
    return `${encodedHeader}.${encodedPayload}.${signature}`;
  }

  public validateToken(token: string): { valid: boolean; payload?: any; error?: string } {
    try {
      const parts = token.split('.');
      if (parts.length !== 3) {
        return { valid: false, error: 'Invalid token format' };
      }

      const payload = JSON.parse(atob(parts[1]));
      
      // Check expiration
      if (payload.exp * 1000 < Date.now()) {
        return { valid: false, error: 'Token expired' };
      }

      // Check if session exists
      const session = this.activeSessions.get(payload.sid);
      if (!session) {
        return { valid: false, error: 'Session not found' };
      }

      return { valid: true, payload };

    } catch (error) {
      return { 
        valid: false, 
        error: error instanceof Error ? error.message : 'Token validation failed' 
      };
    }
  }

  public async refreshToken(currentToken: string): Promise<{ success: boolean; token?: SessionToken; error?: string }> {
    const validation = this.validateToken(currentToken);
    
    if (!validation.valid) {
      return { success: false, error: validation.error };
    }

    const payload = validation.payload;
    const session = this.activeSessions.get(payload.sid);
    
    if (!session) {
      return { success: false, error: 'Session not found' };
    }

    // Check if refresh is needed
    const timeUntilExpiry = session.expiresAt - Date.now();
    if (timeUntilExpiry > this.config.refreshThreshold) {
      return { success: true, token: session }; // No refresh needed
    }

    // Create new token with extended expiry
    const newToken: SessionToken = {
      ...session,
      token: this.generateJWT({ 
        userId: session.userId, 
        username: payload.username,
        permissions: session.scopes 
      } as UserCredentials, session.sessionId),
      expiresAt: Date.now() + this.config.tokenLifetime
    };

    this.activeSessions.set(session.sessionId, newToken);
    
    return { success: true, token: newToken };
  }

  public revokeSession(sessionId: string): boolean {
    return this.activeSessions.delete(sessionId);
  }

  public revokeAllUserSessions(userId: string): number {
    let revokedCount = 0;
    
    for (const [sessionId, session] of this.activeSessions.entries()) {
      if (session.userId === userId) {
        this.activeSessions.delete(sessionId);
        revokedCount++;
      }
    }
    
    return revokedCount;
  }

  private cleanupUserSessions(userId: string): void {
    const userSessions = Array.from(this.activeSessions.values())
      .filter(session => session.userId === userId)
      .sort((a, b) => b.expiresAt - a.expiresAt);

    // Remove oldest sessions if over limit
    if (userSessions.length >= this.config.maxSessions) {
      const sessionsToRemove = userSessions.slice(this.config.maxSessions - 1);
      sessionsToRemove.forEach(session => {
        this.activeSessions.delete(session.sessionId);
      });
    }
  }

  private recordFailedAttempt(username: string): void {
    const current = this.loginAttempts.get(username) || { count: 0, lastAttempt: 0 };
    current.count++;
    current.lastAttempt = Date.now();
    this.loginAttempts.set(username, current);
  }

  private isAccountLocked(username: string): boolean {
    const attempts = this.loginAttempts.get(username);
    
    if (!attempts || attempts.count < this.maxLoginAttempts) {
      return false;
    }

    const timeSinceLastAttempt = Date.now() - attempts.lastAttempt;
    
    if (timeSinceLastAttempt > this.lockoutDuration) {
      // Reset attempts after lockout period
      this.loginAttempts.delete(username);
      return false;
    }

    return true;
  }

  public getSessionInfo(sessionId: string): SessionToken | null {
    return this.activeSessions.get(sessionId) || null;
  }

  public getActiveSessionCount(userId?: string): number {
    if (userId) {
      return Array.from(this.activeSessions.values())
        .filter(session => session.userId === userId).length;
    }
    return this.activeSessions.size;
  }

  public cleanupExpiredSessions(): number {
    let cleanedCount = 0;
    const now = Date.now();
    
    for (const [sessionId, session] of this.activeSessions.entries()) {
      if (session.expiresAt < now) {
        this.activeSessions.delete(sessionId);
        cleanedCount++;
      }
    }
    
    return cleanedCount;
  }
}

// WebRTC Peer Authentication
export class WebRTCPeerAuth {
  private authManager: AuthenticationManager;
  private peerTokens: Map<string, { token: string; expiresAt: number }> = new Map();

  constructor(authManager: AuthenticationManager) {
    this.authManager = authManager;
  }

  public generatePeerToken(sessionToken: string, peerId: string): string | null {
    const validation = this.authManager.validateToken(sessionToken);
    
    if (!validation.valid) {
      return null;
    }

    const peerToken = this.generateSecureToken();
    const expiresAt = Date.now() + 300000; // 5 minutes for WebRTC handshake
    
    this.peerTokens.set(peerId, { token: peerToken, expiresAt });
    
    return peerToken;
  }

  public validatePeerToken(peerId: string, token: string): boolean {
    const peerAuth = this.peerTokens.get(peerId);
    
    if (!peerAuth) {
      return false;
    }

    if (peerAuth.expiresAt < Date.now()) {
      this.peerTokens.delete(peerId);
      return false;
    }

    return peerAuth.token === token;
  }

  private generateSecureToken(): string {
    const array = new Uint8Array(16);
    crypto.getRandomValues(array);
    return btoa(String.fromCharCode(...array)).replace(/[+/=]/g, '');
  }

  public revokePeerToken(peerId: string): boolean {
    return this.peerTokens.delete(peerId);
  }

  public cleanupExpiredPeerTokens(): number {
    let cleanedCount = 0;
    const now = Date.now();
    
    for (const [peerId, peerAuth] of this.peerTokens.entries()) {
      if (peerAuth.expiresAt < now) {
        this.peerTokens.delete(peerId);
        cleanedCount++;
      }
    }
    
    return cleanedCount;
  }
}

// Permission-based access control
export class PermissionManager {
  private rolePermissions: Map<string, string[]> = new Map([
    ['viewer', ['canvas.view', 'collaboration.observe']],
    ['editor', ['canvas.view', 'canvas.edit', 'collaboration.join']],
    ['owner', ['canvas.view', 'canvas.edit', 'canvas.delete', 'collaboration.join', 'collaboration.manage']],
    ['admin', ['*']] // All permissions
  ]);

  public hasPermission(userPermissions: string[], requiredPermission: string): boolean {
    // Admin wildcard permission
    if (userPermissions.includes('*')) {
      return true;
    }

    // Direct permission match
    if (userPermissions.includes(requiredPermission)) {
      return true;
    }

    // Hierarchical permission check (e.g., 'canvas.*' includes 'canvas.edit')
    const wildcardPermissions = userPermissions.filter(p => p.endsWith('.*'));
    for (const wildcard of wildcardPermissions) {
      const prefix = wildcard.slice(0, -1); // Remove '*'
      if (requiredPermission.startsWith(prefix)) {
        return true;
      }
    }

    return false;
  }

  public getRolePermissions(role: string): string[] {
    return this.rolePermissions.get(role) || [];
  }

  public addRole(role: string, permissions: string[]): void {
    this.rolePermissions.set(role, permissions);
  }

  public addPermissionToRole(role: string, permission: string): boolean {
    const currentPermissions = this.rolePermissions.get(role);
    if (currentPermissions) {
      currentPermissions.push(permission);
      return true;
    }
    return false;
  }
}

// Global authentication instances
export const authManager = new AuthenticationManager();
export const webrtcPeerAuth = new WebRTCPeerAuth(authManager);
export const permissionManager = new PermissionManager();

// Auto-cleanup expired sessions and peer tokens
if (typeof setInterval !== 'undefined') {
  setInterval(() => {
    authManager.cleanupExpiredSessions();
    webrtcPeerAuth.cleanupExpiredPeerTokens();
  }, 60000); // Every minute
}
