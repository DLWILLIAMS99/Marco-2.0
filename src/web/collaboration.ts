/**
 * Marco 2.0 - Collaboration Engine
 * Real-time collaborative editing with conflict resolution and WebRTC integration
 */

import { analytics } from '../analytics/analytics';
import { authManager, webrtcPeerAuth } from '../security/auth';
import { crashReporter } from '../analytics/crash-reporter';

export interface CollaborationUser {
  id: string;
  name: string;
  avatar?: string;
  cursor: { x: number; y: number } | null;
  selection: { nodeIds: string[]; scopeId?: string } | null;
  color: string;
  status: 'online' | 'away' | 'offline';
  lastSeen: number;
  permissions: {
    canEdit: boolean;
    canDelete: boolean;
    canAddNodes: boolean;
    canModifyConnections: boolean;
    scopeAccess: string[]; // Scope IDs user can access
  };
}

export interface CollaborationUpdate {
  id: string;
  timestamp: number;
  userId: string;
  sessionId: string;
  type: 'node_created' | 'node_updated' | 'node_deleted' | 'connection_created' | 'connection_deleted' | 'cursor_moved' | 'selection_changed' | 'scope_changed' | 'metadata_updated';
  data: any;
  scopeId?: string;
  vectorClock: { [userId: string]: number };
  dependencies?: string[]; // Update IDs this depends on
}

export interface ConflictResolution {
  updateId: string;
  conflictType: 'concurrent_edit' | 'delete_modified' | 'circular_dependency' | 'permission_denied';
  resolution: 'accept' | 'reject' | 'merge' | 'transform';
  transformedData?: any;
  reason: string;
}

export interface CollaborationSession {
  id: string;
  projectId: string;
  users: Map<string, CollaborationUser>;
  updates: CollaborationUpdate[];
  conflictLog: ConflictResolution[];
  createdAt: number;
  lastActivity: number;
  settings: {
    maxUsers: number;
    autoSave: boolean;
    conflictResolution: 'auto' | 'manual';
    enableVoiceChat: boolean;
    enableScreenShare: boolean;
  };
}

export class CollaborationEngine {
  private session: CollaborationSession | null = null;
  private localUser: CollaborationUser | null = null;
  private vectorClock: { [userId: string]: number } = {};
  private pendingUpdates: Map<string, CollaborationUpdate> = new Map();
  private rtcConnections: Map<string, RTCPeerConnection> = new Map();
  private dataChannels: Map<string, RTCDataChannel> = new Map();
  private isHost: boolean = false;
  private reconnectAttempts: number = 0;
  private maxReconnectAttempts: number = 5;
  private heartbeatInterval: number | null = null;

  constructor() {
    this.setupEventHandlers();
    this.startHeartbeat();
  }

  // Session Management
  public async createSession(projectId: string, user: Omit<CollaborationUser, 'id' | 'status' | 'lastSeen'>): Promise<string> {
    try {
      const sessionId = this.generateSessionId();
      const userId = this.generateUserId();
      
      this.localUser = {
        id: userId,
        status: 'online',
        lastSeen: Date.now(),
        ...user
      };

      this.session = {
        id: sessionId,
        projectId,
        users: new Map([[userId, this.localUser]]),
        updates: [],
        conflictLog: [],
        createdAt: Date.now(),
        lastActivity: Date.now(),
        settings: {
          maxUsers: 10,
          autoSave: true,
          conflictResolution: 'auto',
          enableVoiceChat: true,
          enableScreenShare: true
        }
      };

      this.isHost = true;
      this.vectorClock[userId] = 0;

      // Track session creation
      analytics.trackEvent('collaboration_session_created', 'user_action', {
        sessionId,
        projectId,
        isHost: true
      });

      console.log(`🤝 Collaboration session created: ${sessionId}`);
      this.emitSessionEvent('session-created', { sessionId, user: this.localUser });

      return sessionId;
    } catch (error) {
      crashReporter.reportCustomError({
        type: 'webrtc_error',
        message: 'Failed to create collaboration session',
        stack: error instanceof Error ? error.stack : undefined
      });
      throw error;
    }
  }

  public async joinSession(sessionId: string, user: Omit<CollaborationUser, 'id' | 'status' | 'lastSeen'>): Promise<boolean> {
    try {
      const userId = this.generateUserId();
      
      this.localUser = {
        id: userId,
        status: 'online',
        lastSeen: Date.now(),
        ...user
      };

      // Initialize vector clock for this user
      this.vectorClock[userId] = 0;

      // Attempt to connect to session host
      const connected = await this.connectToHost(sessionId);
      
      if (connected) {
        analytics.trackEvent('collaboration_session_joined', 'user_action', {
          sessionId,
          userId
        });

        console.log(`🤝 Joined collaboration session: ${sessionId}`);
        this.emitSessionEvent('session-joined', { sessionId, user: this.localUser });
        return true;
      }

      return false;
    } catch (error) {
      crashReporter.reportCustomError({
        type: 'webrtc_error',
        message: 'Failed to join collaboration session',
        stack: error instanceof Error ? error.stack : undefined
      });
      return false;
    }
  }

  // Real-time Updates
  public broadcastUpdate(update: Omit<CollaborationUpdate, 'id' | 'timestamp' | 'userId' | 'sessionId' | 'vectorClock'>): void {
    if (!this.session || !this.localUser) {
      console.warn('Cannot broadcast update: no active session');
      return;
    }

    // Increment vector clock for local user
    this.vectorClock[this.localUser.id]++;

    const fullUpdate: CollaborationUpdate = {
      id: this.generateUpdateId(),
      timestamp: Date.now(),
      userId: this.localUser.id,
      sessionId: this.session.id,
      vectorClock: { ...this.vectorClock },
      ...update
    };

    // Add to session history
    this.session.updates.push(fullUpdate);
    this.session.lastActivity = Date.now();

    // Check for conflicts
    const conflicts = this.detectConflicts(fullUpdate);
    if (conflicts.length > 0) {
      this.resolveConflicts(fullUpdate, conflicts);
    }

    // Broadcast to all peers
    this.broadcastToPeers(fullUpdate);

    // Track update
    analytics.trackEvent('collaboration_update', 'user_action', {
      updateType: update.type,
      sessionId: this.session.id,
      userCount: this.session.users.size
    });

    // Emit local event
    this.emitSessionEvent('update-broadcasted', fullUpdate);
  }

  public handleRemoteUpdate(update: CollaborationUpdate): void {
    if (!this.session) return;

    // Update vector clock
    Object.entries(update.vectorClock).forEach(([userId, clock]) => {
      this.vectorClock[userId] = Math.max(this.vectorClock[userId] || 0, clock);
    });

    // Check for conflicts
    const conflicts = this.detectConflicts(update);
    if (conflicts.length > 0) {
      this.resolveConflicts(update, conflicts);
      return;
    }

    // Apply update
    this.applyUpdate(update);
    
    // Add to session history
    this.session.updates.push(update);
    this.session.lastActivity = Date.now();

    // Emit event for UI to handle
    this.emitSessionEvent('update-received', update);
  }

  // Conflict Resolution
  private detectConflicts(update: CollaborationUpdate): CollaborationUpdate[] {
    if (!this.session) return [];

    const conflicts: CollaborationUpdate[] = [];
    const recentUpdates = this.session.updates.slice(-50); // Check last 50 updates

    for (const existingUpdate of recentUpdates) {
      if (this.isConflicting(update, existingUpdate)) {
        conflicts.push(existingUpdate);
      }
    }

    return conflicts;
  }

  private isConflicting(update1: CollaborationUpdate, update2: CollaborationUpdate): boolean {
    // Same update ID
    if (update1.id === update2.id) return false;
    
    // Different users
    if (update1.userId === update2.userId) return false;

    // Check for logical conflicts
    switch (update1.type) {
      case 'node_updated':
      case 'node_deleted':
        return update2.type === 'node_updated' || update2.type === 'node_deleted';
      
      case 'connection_created':
      case 'connection_deleted':
        return (update2.type === 'connection_created' || update2.type === 'connection_deleted') &&
               this.sameConnectionTarget(update1.data, update2.data);
      
      case 'metadata_updated':
        return update2.type === 'metadata_updated' && 
               update1.data.path === update2.data.path;
    }

    return false;
  }

  private sameConnectionTarget(data1: any, data2: any): boolean {
    return (data1.sourceId === data2.sourceId && data1.targetId === data2.targetId) ||
           (data1.sourceId === data2.targetId && data1.targetId === data2.sourceId);
  }

  private resolveConflicts(update: CollaborationUpdate, conflicts: CollaborationUpdate[]): void {
    if (!this.session) return;

    for (const conflict of conflicts) {
      let resolution: ConflictResolution;

      if (this.session.settings.conflictResolution === 'auto') {
        resolution = this.autoResolveConflict(update, conflict);
      } else {
        resolution = this.requestManualResolution(update, conflict);
      }

      this.session.conflictLog.push(resolution);
      
      // Apply resolution
      if (resolution.resolution === 'transform' && resolution.transformedData) {
        update.data = resolution.transformedData;
      }

      // Track conflict resolution
      analytics.trackEvent('collaboration_conflict_resolved', 'system', {
        conflictType: resolution.conflictType,
        resolution: resolution.resolution,
        sessionId: this.session.id
      });
    }
  }

  private autoResolveConflict(update: CollaborationUpdate, conflict: CollaborationUpdate): ConflictResolution {
    // Last-write-wins with timestamp tiebreaker
    const winner = update.timestamp > conflict.timestamp ? update : conflict;
    const loser = winner === update ? conflict : update;

    return {
      updateId: update.id,
      conflictType: 'concurrent_edit',
      resolution: winner === update ? 'accept' : 'reject',
      reason: `Last-write-wins: ${winner.timestamp} > ${loser.timestamp}`
    };
  }

  private requestManualResolution(update: CollaborationUpdate, conflict: CollaborationUpdate): ConflictResolution {
    // Emit event for UI to handle manual resolution
    this.emitSessionEvent('conflict-detected', { update, conflict });

    // Default to reject for now
    return {
      updateId: update.id,
      conflictType: 'concurrent_edit',
      resolution: 'reject',
      reason: 'Manual resolution required'
    };
  }

  private applyUpdate(update: CollaborationUpdate): void {
    // Emit specific events based on update type
    switch (update.type) {
      case 'node_created':
        this.emitSessionEvent('node-created', update.data);
        break;
      case 'node_updated':
        this.emitSessionEvent('node-updated', update.data);
        break;
      case 'node_deleted':
        this.emitSessionEvent('node-deleted', update.data);
        break;
      case 'connection_created':
        this.emitSessionEvent('connection-created', update.data);
        break;
      case 'connection_deleted':
        this.emitSessionEvent('connection-deleted', update.data);
        break;
      case 'cursor_moved':
        this.updateUserCursor(update.userId, update.data);
        break;
      case 'selection_changed':
        this.updateUserSelection(update.userId, update.data);
        break;
      case 'metadata_updated':
        this.emitSessionEvent('metadata-updated', update.data);
        break;
    }
  }

  // WebRTC Integration
  private async connectToHost(sessionId: string): Promise<boolean> {
    try {
      // In a real implementation, this would use a signaling server
      // For now, simulate connection
      console.log(`Connecting to session host for ${sessionId}...`);
      
      // Create peer connection
      const peerConnection = this.createPeerConnection('host');
      this.rtcConnections.set('host', peerConnection);

      // Create data channel for collaboration updates
      const dataChannel = peerConnection.createDataChannel('collaboration', {
        ordered: true
      });
      
      this.setupDataChannel(dataChannel, 'host');
      this.dataChannels.set('host', dataChannel);

      return true;
    } catch (error) {
      console.error('Failed to connect to host:', error);
      return false;
    }
  }

  private createPeerConnection(peerId: string): RTCPeerConnection {
    const config: RTCConfiguration = {
      iceServers: [
        { urls: 'stun:stun.l.google.com:19302' },
        { urls: 'stun:stun1.l.google.com:19302' }
      ]
    };

    const peerConnection = new RTCPeerConnection(config);

    peerConnection.oniceconnectionstatechange = () => {
      console.log(`ICE connection state for ${peerId}:`, peerConnection.iceConnectionState);
      
      if (peerConnection.iceConnectionState === 'disconnected' || 
          peerConnection.iceConnectionState === 'failed') {
        this.handlePeerDisconnection(peerId);
      }
    };

    peerConnection.ondatachannel = (event) => {
      this.setupDataChannel(event.channel, peerId);
    };

    return peerConnection;
  }

  private setupDataChannel(channel: RTCDataChannel, peerId: string): void {
    channel.onopen = () => {
      console.log(`Data channel opened with ${peerId}`);
    };

    channel.onmessage = (event) => {
      try {
        const update: CollaborationUpdate = JSON.parse(event.data);
        this.handleRemoteUpdate(update);
      } catch (error) {
        console.error('Failed to parse collaboration update:', error);
      }
    };

    channel.onclose = () => {
      console.log(`Data channel closed with ${peerId}`);
    };

    channel.onerror = (error) => {
      console.error(`Data channel error with ${peerId}:`, error);
      crashReporter.reportCustomError({
        type: 'webrtc_error',
        message: `Data channel error with peer ${peerId}`
      });
    };
  }

  private broadcastToPeers(update: CollaborationUpdate): void {
    const message = JSON.stringify(update);
    
    this.dataChannels.forEach((channel, peerId) => {
      if (channel.readyState === 'open') {
        try {
          channel.send(message);
        } catch (error) {
          console.error(`Failed to send update to ${peerId}:`, error);
        }
      }
    });
  }

  private handlePeerDisconnection(peerId: string): void {
    console.log(`Peer ${peerId} disconnected`);
    
    // Clean up connections
    this.rtcConnections.delete(peerId);
    this.dataChannels.delete(peerId);
    
    // Update user status
    if (this.session) {
      const user = this.session.users.get(peerId);
      if (user) {
        user.status = 'offline';
        user.lastSeen = Date.now();
        
        this.emitSessionEvent('user-disconnected', { userId: peerId, user });
      }
    }

    // Attempt reconnection
    this.attemptReconnection(peerId);
  }

  private async attemptReconnection(peerId: string): Promise<void> {
    if (this.reconnectAttempts >= this.maxReconnectAttempts) {
      console.log(`Max reconnection attempts reached for ${peerId}`);
      return;
    }

    this.reconnectAttempts++;
    const delay = Math.min(1000 * Math.pow(2, this.reconnectAttempts), 30000);
    
    console.log(`Attempting to reconnect to ${peerId} in ${delay}ms...`);
    
    setTimeout(async () => {
      try {
        // Reconnection logic would go here
        console.log(`Reconnection attempt ${this.reconnectAttempts} for ${peerId}`);
      } catch (error) {
        console.error(`Reconnection failed for ${peerId}:`, error);
        this.attemptReconnection(peerId);
      }
    }, delay);
  }

  // User Management
  private updateUserCursor(userId: string, cursorData: { x: number; y: number }): void {
    if (!this.session) return;

    const user = this.session.users.get(userId);
    if (user) {
      user.cursor = cursorData;
      user.lastSeen = Date.now();
      
      this.emitSessionEvent('user-cursor-updated', { userId, cursor: cursorData });
    }
  }

  private updateUserSelection(userId: string, selectionData: { nodeIds: string[]; scopeId?: string }): void {
    if (!this.session) return;

    const user = this.session.users.get(userId);
    if (user) {
      user.selection = selectionData;
      user.lastSeen = Date.now();
      
      this.emitSessionEvent('user-selection-updated', { userId, selection: selectionData });
    }
  }

  // Utility Methods
  private generateSessionId(): string {
    return `session_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
  }

  private generateUserId(): string {
    return `user_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
  }

  private generateUpdateId(): string {
    return `update_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
  }

  private setupEventHandlers(): void {
    // Handle page visibility changes
    document.addEventListener('visibilitychange', () => {
      if (this.localUser) {
        this.localUser.status = document.hidden ? 'away' : 'online';
        this.localUser.lastSeen = Date.now();
        
        if (this.session) {
          this.broadcastUpdate({
            type: 'cursor_moved',
            data: { status: this.localUser.status }
          });
        }
      }
    });

    // Handle before unload
    window.addEventListener('beforeunload', () => {
      this.leaveSession();
    });
  }

  private startHeartbeat(): void {
    this.heartbeatInterval = setInterval(() => {
      if (this.session && this.localUser) {
        this.localUser.lastSeen = Date.now();
        
        // Send heartbeat to peers
        this.broadcastUpdate({
          type: 'cursor_moved',
          data: { heartbeat: true }
        });
      }
    }, 30000); // Every 30 seconds
  }

  private emitSessionEvent(eventName: string, data: any): void {
    document.dispatchEvent(new CustomEvent(`collaboration-${eventName}`, { detail: data }));
  }

  // Public API
  public getSession(): CollaborationSession | null {
    return this.session;
  }

  public getLocalUser(): CollaborationUser | null {
    return this.localUser;
  }

  public getConnectedUsers(): CollaborationUser[] {
    if (!this.session) return [];
    return Array.from(this.session.users.values()).filter(user => user.status === 'online');
  }

  public updateLocalCursor(x: number, y: number): void {
    if (this.localUser) {
      this.localUser.cursor = { x, y };
      this.broadcastUpdate({
        type: 'cursor_moved',
        data: { x, y }
      });
    }
  }

  public updateLocalSelection(nodeIds: string[], scopeId?: string): void {
    if (this.localUser) {
      this.localUser.selection = { nodeIds, scopeId };
      this.broadcastUpdate({
        type: 'selection_changed',
        data: { nodeIds, scopeId }
      });
    }
  }

  public leaveSession(): void {
    if (this.session && this.localUser) {
      // Broadcast disconnection
      this.broadcastUpdate({
        type: 'cursor_moved',
        data: { disconnecting: true }
      });

      // Clean up connections
      this.rtcConnections.forEach(conn => conn.close());
      this.dataChannels.forEach(channel => channel.close());
      
      this.rtcConnections.clear();
      this.dataChannels.clear();

      // Track session end
      analytics.trackEvent('collaboration_session_left', 'user_action', {
        sessionId: this.session.id,
        duration: Date.now() - this.session.createdAt
      });

      this.session = null;
      this.localUser = null;
      this.vectorClock = {};
      this.reconnectAttempts = 0;

      console.log('🤝 Left collaboration session');
      this.emitSessionEvent('session-left', {});
    }
  }

  public destroy(): void {
    this.leaveSession();
    
    if (this.heartbeatInterval) {
      clearInterval(this.heartbeatInterval);
      this.heartbeatInterval = null;
    }
  }
}

// Global collaboration engine
export const collaborationEngine = new CollaborationEngine();
