/**
 * Real-time Collaboration System for Marco 2.0
 * 
 * WebRTC-based peer-to-peer collaboration with operational transform
 */

export interface CollaborativeUser {
  id: string;
  name: string;
  color: string;
  cursor?: { x: number; y: number };
  isActive: boolean;
  lastSeen: number;
}

export interface CollaborativeOperation {
  id: string;
  type: 'node-create' | 'node-delete' | 'node-move' | 'node-edit' | 'connection-create' | 'connection-delete';
  userId: string;
  timestamp: number;
  data: any;
  transformed?: boolean;
}

export interface CollaborativeSession {
  id: string;
  name: string;
  users: CollaborativeUser[];
  isHost: boolean;
  isConnected: boolean;
  operations: CollaborativeOperation[];
}

export interface RTCConnectionConfig {
  iceServers: RTCIceServer[];
  enableVoice: boolean;
  enableVideo: boolean;
  dataChannelConfig: RTCDataChannelInit;
}

export class CollaborationManager {
  private session: CollaborativeSession | null = null;
  private peerConnections: Map<string, RTCPeerConnection> = new Map();
  private dataChannels: Map<string, RTCDataChannel> = new Map();
  private signalingChannel: WebSocket | null = null;
  private localUser: CollaborativeUser;
  private operationQueue: CollaborativeOperation[] = [];
  private isProcessingOperations = false;
  
  // Event callbacks
  private onUserJoined?: (user: CollaborativeUser) => void;
  private onUserLeft?: (userId: string) => void;
  private onOperationReceived?: (operation: CollaborativeOperation) => void;
  private onCursorMoved?: (userId: string, x: number, y: number) => void;
  private onConnectionStateChanged?: (state: RTCPeerConnectionState) => void;

  private readonly CONFIG: RTCConnectionConfig = {
    iceServers: [
      { urls: 'stun:stun.l.google.com:19302' },
      { urls: 'stun:stun1.l.google.com:19302' },
    ],
    enableVoice: false,
    enableVideo: false,
    dataChannelConfig: {
      ordered: true,
      maxRetransmits: 3,
    },
  };

  constructor(userId?: string, userName?: string) {
    this.localUser = {
      id: userId || this.generateUserId(),
      name: userName || 'Anonymous User',
      color: this.generateUserColor(),
      isActive: true,
      lastSeen: Date.now(),
    };

    this.setupSignalingConnection();
  }

  /**
   * Create a new collaborative session
   */
  public async createSession(sessionName: string): Promise<string> {
    const sessionId = this.generateSessionId();
    
    this.session = {
      id: sessionId,
      name: sessionName,
      users: [this.localUser],
      isHost: true,
      isConnected: true,
      operations: [],
    };

    await this.announceSession();
    console.log(`Created collaboration session: ${sessionId}`);
    
    return sessionId;
  }

  /**
   * Join an existing collaborative session
   */
  public async joinSession(sessionId: string): Promise<boolean> {
    try {
      // Request to join session through signaling server
      await this.sendSignalingMessage({
        type: 'join-session',
        sessionId,
        user: this.localUser,
      });

      // Wait for session information
      const sessionInfo = await this.waitForSessionInfo(sessionId);
      
      this.session = {
        ...sessionInfo,
        isHost: false,
        isConnected: true,
      };

      // Establish peer connections with existing users
      await this.connectToExistingUsers();
      
      console.log(`Joined collaboration session: ${sessionId}`);
      return true;
    } catch (error) {
      console.error('Failed to join session:', error);
      return false;
    }
  }

  /**
   * Leave the current collaborative session
   */
  public async leaveSession(): Promise<void> {
    if (!this.session) return;

    // Close all peer connections
    for (const [userId, connection] of this.peerConnections) {
      connection.close();
      this.peerConnections.delete(userId);
    }

    // Close data channels
    for (const [userId, channel] of this.dataChannels) {
      channel.close();
      this.dataChannels.delete(userId);
    }

    // Notify other users
    await this.sendSignalingMessage({
      type: 'leave-session',
      sessionId: this.session.id,
      userId: this.localUser.id,
    });

    this.session = null;
    console.log('Left collaboration session');
  }

  /**
   * Send an operation to all connected peers
   */
  public async sendOperation(operation: Omit<CollaborativeOperation, 'id' | 'userId' | 'timestamp'>): Promise<void> {
    if (!this.session) return;

    const fullOperation: CollaborativeOperation = {
      id: this.generateOperationId(),
      userId: this.localUser.id,
      timestamp: Date.now(),
      ...operation,
    };

    // Add to local queue for processing
    this.operationQueue.push(fullOperation);
    
    // Send to all connected peers
    const message = JSON.stringify({
      type: 'operation',
      operation: fullOperation,
    });

    for (const [userId, channel] of this.dataChannels) {
      if (channel.readyState === 'open') {
        try {
          channel.send(message);
        } catch (error) {
          console.error(`Failed to send operation to ${userId}:`, error);
        }
      }
    }

    // Process operations locally
    this.processOperationQueue();
  }

  /**
   * Update cursor position for real-time cursor sharing
   */
  public updateCursor(x: number, y: number): void {
    if (!this.session) return;

    this.localUser.cursor = { x, y };
    this.localUser.lastSeen = Date.now();

    const message = JSON.stringify({
      type: 'cursor-update',
      userId: this.localUser.id,
      x,
      y,
    });

    for (const [userId, channel] of this.dataChannels) {
      if (channel.readyState === 'open') {
        try {
          channel.send(message);
        } catch (error) {
          console.error(`Failed to send cursor update to ${userId}:`, error);
        }
      }
    }
  }

  /**
   * Get current session information
   */
  public getSession(): CollaborativeSession | null {
    return this.session;
  }

  /**
   * Get all connected users
   */
  public getConnectedUsers(): CollaborativeUser[] {
    return this.session?.users || [];
  }

  /**
   * Set event callbacks
   */
  public setEventCallbacks(callbacks: {
    onUserJoined?: (user: CollaborativeUser) => void;
    onUserLeft?: (userId: string) => void;
    onOperationReceived?: (operation: CollaborativeOperation) => void;
    onCursorMoved?: (userId: string, x: number, y: number) => void;
    onConnectionStateChanged?: (state: RTCPeerConnectionState) => void;
  }): void {
    this.onUserJoined = callbacks.onUserJoined;
    this.onUserLeft = callbacks.onUserLeft;
    this.onOperationReceived = callbacks.onOperationReceived;
    this.onCursorMoved = callbacks.onCursorMoved;
    this.onConnectionStateChanged = callbacks.onConnectionStateChanged;
  }

  /**
   * Enable voice chat (experimental)
   */
  public async enableVoiceChat(): Promise<boolean> {
    try {
      const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
      
      // Add audio track to all peer connections
      for (const [userId, connection] of this.peerConnections) {
        stream.getAudioTracks().forEach(track => {
          connection.addTrack(track, stream);
        });
      }

      console.log('Voice chat enabled');
      return true;
    } catch (error) {
      console.error('Failed to enable voice chat:', error);
      return false;
    }
  }

  /**
   * Disable voice chat
   */
  public disableVoiceChat(): void {
    for (const [userId, connection] of this.peerConnections) {
      connection.getSenders().forEach(sender => {
        if (sender.track?.kind === 'audio') {
          connection.removeTrack(sender);
        }
      });
    }
    console.log('Voice chat disabled');
  }

  /**
   * Private methods
   */

  private setupSignalingConnection(): void {
    // In a real implementation, this would connect to a signaling server
    // For now, we'll simulate with localStorage for same-origin communication
    window.addEventListener('storage', (event) => {
      if (event.key?.startsWith('marco2-signaling-')) {
        try {
          const message = JSON.parse(event.newValue || '{}');
          this.handleSignalingMessage(message);
        } catch (error) {
          console.error('Failed to parse signaling message:', error);
        }
      }
    });
  }

  private async sendSignalingMessage(message: any): Promise<void> {
    // Simulate signaling server with localStorage
    const key = `marco2-signaling-${Date.now()}-${Math.random()}`;
    localStorage.setItem(key, JSON.stringify(message));
    
    // Clean up old messages
    setTimeout(() => {
      localStorage.removeItem(key);
    }, 5000);
  }

  private async handleSignalingMessage(message: any): Promise<void> {
    switch (message.type) {
      case 'join-session':
        if (this.session?.isHost && message.sessionId === this.session.id) {
          await this.handleUserJoinRequest(message.user);
        }
        break;
        
      case 'offer':
        await this.handleOffer(message);
        break;
        
      case 'answer':
        await this.handleAnswer(message);
        break;
        
      case 'ice-candidate':
        await this.handleIceCandidate(message);
        break;
        
      case 'leave-session':
        this.handleUserLeft(message.userId);
        break;
    }
  }

  private async handleUserJoinRequest(user: CollaborativeUser): Promise<void> {
    if (!this.session) return;

    // Add user to session
    this.session.users.push(user);

    // Create peer connection for new user
    await this.createPeerConnection(user.id);

    // Notify other users
    this.onUserJoined?.(user);
    console.log(`User joined: ${user.name} (${user.id})`);
  }

  private async createPeerConnection(userId: string): Promise<RTCPeerConnection> {
    const connection = new RTCPeerConnection(this.CONFIG);
    
    // Setup event handlers
    connection.onicecandidate = (event) => {
      if (event.candidate) {
        this.sendSignalingMessage({
          type: 'ice-candidate',
          candidate: event.candidate,
          targetUserId: userId,
          fromUserId: this.localUser.id,
        });
      }
    };

    connection.onconnectionstatechange = () => {
      this.onConnectionStateChanged?.(connection.connectionState);
      console.log(`Connection state with ${userId}:`, connection.connectionState);
    };

    connection.ondatachannel = (event) => {
      this.setupDataChannel(userId, event.channel);
    };

    // Create data channel for sending operations
    const dataChannel = connection.createDataChannel('operations', this.CONFIG.dataChannelConfig);
    this.setupDataChannel(userId, dataChannel);

    this.peerConnections.set(userId, connection);
    return connection;
  }

  private setupDataChannel(userId: string, channel: RTCDataChannel): void {
    channel.onopen = () => {
      console.log(`Data channel opened with ${userId}`);
    };

    channel.onmessage = (event) => {
      try {
        const message = JSON.parse(event.data);
        this.handleDataChannelMessage(userId, message);
      } catch (error) {
        console.error('Failed to parse data channel message:', error);
      }
    };

    channel.onclose = () => {
      console.log(`Data channel closed with ${userId}`);
      this.dataChannels.delete(userId);
    };

    this.dataChannels.set(userId, channel);
  }

  private handleDataChannelMessage(userId: string, message: any): void {
    switch (message.type) {
      case 'operation':
        this.handleRemoteOperation(message.operation);
        break;
        
      case 'cursor-update':
        this.handleRemoteCursor(message);
        break;
    }
  }

  private handleRemoteOperation(operation: CollaborativeOperation): void {
    // Apply operational transform if necessary
    const transformedOperation = this.transformOperation(operation);
    
    // Add to operation queue
    this.operationQueue.push(transformedOperation);
    
    // Process operations
    this.processOperationQueue();
  }

  private handleRemoteCursor(message: any): void {
    if (this.session) {
      const user = this.session.users.find(u => u.id === message.userId);
      if (user) {
        user.cursor = { x: message.x, y: message.y };
        user.lastSeen = Date.now();
        this.onCursorMoved?.(message.userId, message.x, message.y);
      }
    }
  }

  private transformOperation(operation: CollaborativeOperation): CollaborativeOperation {
    // Implement operational transform logic here
    // This is a simplified version - real OT is more complex
    
    if (operation.transformed) {
      return operation;
    }

    // Check for conflicts with local operations
    const conflictingOps = this.operationQueue.filter(op => 
      op.timestamp > operation.timestamp && 
      this.operationsConflict(op, operation)
    );

    if (conflictingOps.length > 0) {
      // Apply transformation rules
      const transformed = { ...operation, transformed: true };
      
      // Example transformation for node movement
      if (operation.type === 'node-move' && conflictingOps.some(op => op.type === 'node-move')) {
        // Adjust position based on conflict resolution strategy
        transformed.data = this.resolveMovementConflict(operation.data, conflictingOps);
      }
      
      return transformed;
    }

    return operation;
  }

  private operationsConflict(op1: CollaborativeOperation, op2: CollaborativeOperation): boolean {
    // Check if operations conflict based on type and target
    if (op1.type === op2.type) {
      switch (op1.type) {
        case 'node-edit':
        case 'node-move':
        case 'node-delete':
          return op1.data.nodeId === op2.data.nodeId;
        case 'connection-create':
        case 'connection-delete':
          return op1.data.connectionId === op2.data.connectionId;
      }
    }
    return false;
  }

  private resolveMovementConflict(data: any, conflictingOps: CollaborativeOperation[]): any {
    // Simple conflict resolution - average the positions
    const movements = conflictingOps.filter(op => op.type === 'node-move');
    if (movements.length === 0) return data;

    const avgX = movements.reduce((sum, op) => sum + op.data.x, data.x) / (movements.length + 1);
    const avgY = movements.reduce((sum, op) => sum + op.data.y, data.y) / (movements.length + 1);

    return { ...data, x: avgX, y: avgY };
  }

  private async processOperationQueue(): Promise<void> {
    if (this.isProcessingOperations) return;
    
    this.isProcessingOperations = true;
    
    try {
      // Sort operations by timestamp
      this.operationQueue.sort((a, b) => a.timestamp - b.timestamp);
      
      // Process each operation
      while (this.operationQueue.length > 0) {
        const operation = this.operationQueue.shift()!;
        
        // Skip operations from local user that we've already applied
        if (operation.userId === this.localUser.id) {
          continue;
        }
        
        // Apply operation
        this.onOperationReceived?.(operation);
        
        // Add to session history
        if (this.session) {
          this.session.operations.push(operation);
        }
      }
    } finally {
      this.isProcessingOperations = false;
    }
  }

  private handleUserLeft(userId: string): void {
    // Remove user from session
    if (this.session) {
      this.session.users = this.session.users.filter(u => u.id !== userId);
    }

    // Close connections
    const connection = this.peerConnections.get(userId);
    if (connection) {
      connection.close();
      this.peerConnections.delete(userId);
    }

    const channel = this.dataChannels.get(userId);
    if (channel) {
      channel.close();
      this.dataChannels.delete(userId);
    }

    this.onUserLeft?.(userId);
    console.log(`User left: ${userId}`);
  }

  private async handleOffer(message: any): Promise<void> {
    const connection = await this.createPeerConnection(message.fromUserId);
    await connection.setRemoteDescription(message.offer);
    
    const answer = await connection.createAnswer();
    await connection.setLocalDescription(answer);
    
    this.sendSignalingMessage({
      type: 'answer',
      answer,
      targetUserId: message.fromUserId,
      fromUserId: this.localUser.id,
    });
  }

  private async handleAnswer(message: any): Promise<void> {
    const connection = this.peerConnections.get(message.fromUserId);
    if (connection) {
      await connection.setRemoteDescription(message.answer);
    }
  }

  private async handleIceCandidate(message: any): Promise<void> {
    const connection = this.peerConnections.get(message.fromUserId);
    if (connection) {
      await connection.addIceCandidate(message.candidate);
    }
  }

  private async announceSession(): Promise<void> {
    await this.sendSignalingMessage({
      type: 'session-created',
      session: this.session,
    });
  }

  private async waitForSessionInfo(sessionId: string): Promise<CollaborativeSession> {
    // Simulate waiting for session info
    return new Promise((resolve, reject) => {
      const timeout = setTimeout(() => {
        reject(new Error('Session not found'));
      }, 5000);

      // In real implementation, this would listen for session info from signaling server
      const mockSession: CollaborativeSession = {
        id: sessionId,
        name: 'Mock Session',
        users: [],
        isHost: false,
        isConnected: false,
        operations: [],
      };

      clearTimeout(timeout);
      resolve(mockSession);
    });
  }

  private async connectToExistingUsers(): Promise<void> {
    if (!this.session) return;

    for (const user of this.session.users) {
      if (user.id !== this.localUser.id) {
        const connection = await this.createPeerConnection(user.id);
        
        // Create offer
        const offer = await connection.createOffer();
        await connection.setLocalDescription(offer);
        
        // Send offer through signaling
        this.sendSignalingMessage({
          type: 'offer',
          offer,
          targetUserId: user.id,
          fromUserId: this.localUser.id,
        });
      }
    }
  }

  private generateUserId(): string {
    return `user_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
  }

  private generateSessionId(): string {
    return `session_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
  }

  private generateOperationId(): string {
    return `op_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
  }

  private generateUserColor(): string {
    const colors = ['#FF6B6B', '#4ECDC4', '#45B7D1', '#96CEB4', '#FFEAA7', '#DDA0DD', '#98D8C8'];
    return colors[Math.floor(Math.random() * colors.length)];
  }
}

export default CollaborationManager;
