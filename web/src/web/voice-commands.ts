/**
 * Voice Command System for Marco 2.0
 * 
 * Speech recognition and voice-controlled navigation
 */

// TypeScript interfaces for Web Speech API
interface SpeechRecognition extends EventTarget {
  lang: string;
  continuous: boolean;
  interimResults: boolean;
  maxAlternatives: number;
  start(): void;
  stop(): void;
  onstart: ((this: SpeechRecognition, ev: Event) => any) | null;
  onend: ((this: SpeechRecognition, ev: Event) => any) | null;
  onerror: ((this: SpeechRecognition, ev: SpeechRecognitionErrorEvent) => any) | null;
  onresult: ((this: SpeechRecognition, ev: SpeechRecognitionEvent) => any) | null;
  onnomatch: ((this: SpeechRecognition, ev: SpeechRecognitionEvent) => any) | null;
}

interface SpeechRecognitionEvent extends Event {
  results: SpeechRecognitionResultList;
}

interface SpeechRecognitionResultList {
  readonly length: number;
  item(index: number): SpeechRecognitionResult;
  [index: number]: SpeechRecognitionResult;
}

interface SpeechRecognitionResult {
  readonly length: number;
  item(index: number): SpeechRecognitionAlternative;
  [index: number]: SpeechRecognitionAlternative;
  isFinal: boolean;
}

interface SpeechRecognitionAlternative {
  transcript: string;
  confidence: number;
}

interface SpeechRecognitionErrorEvent extends Event {
  error: string;
  message: string;
}

declare global {
  interface Window {
    SpeechRecognition: new () => SpeechRecognition;
    webkitSpeechRecognition: new () => SpeechRecognition;
  }
}

export interface VoiceCommand {
  phrase: string;
  action: string;
  parameters?: Record<string, any>;
  confidence?: number;
  category: 'navigation' | 'editing' | 'system' | 'custom';
}

export interface VoiceSettings {
  language: string;
  continuous: boolean;
  interimResults: boolean;
  maxAlternatives: number;
  confidenceThreshold: number;
  noiseReduction: boolean;
  autoStart: boolean;
}

export interface SpeechResult {
  transcript: string;
  confidence: number;
  isFinal: boolean;
  alternatives: { transcript: string; confidence: number }[];
}

export type VoiceCommandCallback = (command: VoiceCommand, result: SpeechResult) => void;

export class VoiceCommandSystem {
  private recognition: SpeechRecognition | null = null;
  private isListening = false;
  private isEnabled = false;
  private commandMap: Map<string, VoiceCommand> = new Map();
  private listeners: VoiceCommandCallback[] = [];
  private lastResult: SpeechResult | null = null;
  
  // Voice feedback
  private synthesis: SpeechSynthesis | null = null;
  private speechEnabled = true;
  private voiceVolume = 1.0;
  private voiceRate = 1.0;
  private voicePitch = 1.0;

  private readonly DEFAULT_SETTINGS: VoiceSettings = {
    language: 'en-US',
    continuous: true,
    interimResults: true,
    maxAlternatives: 3,
    confidenceThreshold: 0.7,
    noiseReduction: true,
    autoStart: false,
  };

  private settings: VoiceSettings;

  constructor(settings: Partial<VoiceSettings> = {}) {
    this.settings = { ...this.DEFAULT_SETTINGS, ...settings };
    
    this.initializeSpeechRecognition();
    this.initializeSpeechSynthesis();
    this.setupDefaultCommands();
    
    console.log('Voice command system initialized');
  }

  /**
   * Initialize speech recognition
   */
  private initializeSpeechRecognition(): void {
    if (!('webkitSpeechRecognition' in window) && !('SpeechRecognition' in window)) {
      console.warn('Speech recognition not supported in this browser');
      return;
    }

    // Use webkitSpeechRecognition for Chrome or SpeechRecognition for other browsers
    const SpeechRecognition = window.SpeechRecognition || window.webkitSpeechRecognition;
    this.recognition = new SpeechRecognition();

    if (!this.recognition) return;

    this.recognition.lang = this.settings.language;
    this.recognition.continuous = this.settings.continuous;
    this.recognition.interimResults = this.settings.interimResults;
    this.recognition.maxAlternatives = this.settings.maxAlternatives;

    // Event handlers
    this.recognition.onstart = () => {
      this.isListening = true;
      console.log('Voice recognition started');
      this.speakFeedback('Listening...');
    };

    this.recognition.onend = () => {
      this.isListening = false;
      console.log('Voice recognition ended');
      
      // Auto-restart if enabled
      if (this.isEnabled && this.settings.continuous) {
        setTimeout(() => this.startListening(), 100);
      }
    };

    this.recognition.onerror = (event: SpeechRecognitionErrorEvent) => {
      console.error('Speech recognition error:', event.error);
      this.handleRecognitionError(event.error);
    };

    this.recognition.onresult = (event: SpeechRecognitionEvent) => {
      this.handleSpeechResult(event);
    };

    this.recognition.onnomatch = () => {
      console.log('No speech match found');
      this.speakFeedback('Command not recognized');
    };
  }

  /**
   * Initialize speech synthesis
   */
  private initializeSpeechSynthesis(): void {
    if ('speechSynthesis' in window) {
      this.synthesis = window.speechSynthesis;
    } else {
      console.warn('Speech synthesis not supported in this browser');
    }
  }

  /**
   * Setup default voice commands
   */
  private setupDefaultCommands(): void {
    const defaultCommands: VoiceCommand[] = [
      // Navigation commands
      { phrase: 'zoom in', action: 'zoom', parameters: { direction: 'in' }, category: 'navigation' },
      { phrase: 'zoom out', action: 'zoom', parameters: { direction: 'out' }, category: 'navigation' },
      { phrase: 'fit to screen', action: 'fit-to-screen', category: 'navigation' },
      { phrase: 'reset view', action: 'reset-view', category: 'navigation' },
      { phrase: 'pan left', action: 'pan', parameters: { direction: 'left' }, category: 'navigation' },
      { phrase: 'pan right', action: 'pan', parameters: { direction: 'right' }, category: 'navigation' },
      { phrase: 'pan up', action: 'pan', parameters: { direction: 'up' }, category: 'navigation' },
      { phrase: 'pan down', action: 'pan', parameters: { direction: 'down' }, category: 'navigation' },
      
      // Editing commands
      { phrase: 'create node', action: 'create-node', category: 'editing' },
      { phrase: 'delete selected', action: 'delete-selected', category: 'editing' },
      { phrase: 'copy', action: 'copy', category: 'editing' },
      { phrase: 'paste', action: 'paste', category: 'editing' },
      { phrase: 'undo', action: 'undo', category: 'editing' },
      { phrase: 'redo', action: 'redo', category: 'editing' },
      { phrase: 'select all', action: 'select-all', category: 'editing' },
      { phrase: 'deselect all', action: 'deselect-all', category: 'editing' },
      
      // System commands
      { phrase: 'save project', action: 'save', category: 'system' },
      { phrase: 'load project', action: 'load', category: 'system' },
      { phrase: 'new project', action: 'new-project', category: 'system' },
      { phrase: 'toggle debug', action: 'toggle-debug', category: 'system' },
      { phrase: 'show help', action: 'show-help', category: 'system' },
      { phrase: 'toggle fullscreen', action: 'toggle-fullscreen', category: 'system' },
      
      // Voice system commands
      { phrase: 'stop listening', action: 'stop-voice', category: 'system' },
      { phrase: 'mute voice', action: 'mute-voice', category: 'system' },
      { phrase: 'unmute voice', action: 'unmute-voice', category: 'system' },
    ];

    defaultCommands.forEach(command => {
      this.addCommand(command);
    });
  }

  /**
   * Start voice recognition
   */
  public startListening(): boolean {
    if (!this.recognition) {
      console.error('Speech recognition not available');
      return false;
    }

    if (this.isListening) {
      console.log('Already listening');
      return true;
    }

    try {
      this.isEnabled = true;
      this.recognition.start();
      return true;
    } catch (error) {
      console.error('Failed to start voice recognition:', error);
      return false;
    }
  }

  /**
   * Stop voice recognition
   */
  public stopListening(): void {
    if (!this.recognition) return;

    this.isEnabled = false;
    if (this.isListening) {
      this.recognition.stop();
    }
  }

  /**
   * Toggle voice recognition
   */
  public toggleListening(): boolean {
    if (this.isListening) {
      this.stopListening();
      return false;
    } else {
      return this.startListening();
    }
  }

  /**
   * Add a voice command
   */
  public addCommand(command: VoiceCommand): void {
    const normalizedPhrase = this.normalizePhrase(command.phrase);
    this.commandMap.set(normalizedPhrase, command);
    console.log(`Added voice command: "${command.phrase}"`);
  }

  /**
   * Remove a voice command
   */
  public removeCommand(phrase: string): void {
    const normalizedPhrase = this.normalizePhrase(phrase);
    this.commandMap.delete(normalizedPhrase);
    console.log(`Removed voice command: "${phrase}"`);
  }

  /**
   * Get all registered commands
   */
  public getCommands(): VoiceCommand[] {
    return Array.from(this.commandMap.values());
  }

  /**
   * Add voice command listener
   */
  public addListener(callback: VoiceCommandCallback): void {
    this.listeners.push(callback);
  }

  /**
   * Remove voice command listener
   */
  public removeListener(callback: VoiceCommandCallback): void {
    const index = this.listeners.indexOf(callback);
    if (index !== -1) {
      this.listeners.splice(index, 1);
    }
  }

  /**
   * Update voice settings
   */
  public updateSettings(settings: Partial<VoiceSettings>): void {
    this.settings = { ...this.settings, ...settings };
    
    if (this.recognition) {
      this.recognition.lang = this.settings.language;
      this.recognition.continuous = this.settings.continuous;
      this.recognition.interimResults = this.settings.interimResults;
      this.recognition.maxAlternatives = this.settings.maxAlternatives;
    }
  }

  /**
   * Get current settings
   */
  public getSettings(): VoiceSettings {
    return { ...this.settings };
  }

  /**
   * Check if voice recognition is supported
   */
  public isSupported(): boolean {
    return this.recognition !== null;
  }

  /**
   * Check if currently listening
   */
  public getIsListening(): boolean {
    return this.isListening;
  }

  /**
   * Get last recognition result
   */
  public getLastResult(): SpeechResult | null {
    return this.lastResult;
  }

  /**
   * Speak feedback text
   */
  public speakFeedback(text: string, options: Partial<SpeechSynthesisUtterance> = {}): void {
    if (!this.synthesis || !this.speechEnabled) return;

    const utterance = new SpeechSynthesisUtterance(text);
    utterance.volume = options.volume ?? this.voiceVolume;
    utterance.rate = options.rate ?? this.voiceRate;
    utterance.pitch = options.pitch ?? this.voicePitch;
    utterance.lang = options.lang ?? this.settings.language;

    this.synthesis.speak(utterance);
  }

  /**
   * Set voice feedback enabled/disabled
   */
  public setSpeechEnabled(enabled: boolean): void {
    this.speechEnabled = enabled;
    if (!enabled && this.synthesis) {
      this.synthesis.cancel();
    }
  }

  /**
   * Set voice properties
   */
  public setVoiceProperties(volume: number, rate: number, pitch: number): void {
    this.voiceVolume = Math.max(0, Math.min(1, volume));
    this.voiceRate = Math.max(0.1, Math.min(10, rate));
    this.voicePitch = Math.max(0, Math.min(2, pitch));
  }

  /**
   * Test voice recognition with a sample phrase
   */
  public testRecognition(): void {
    this.speakFeedback('Testing voice recognition. Say "create node" to test.');
  }

  /**
   * Private methods
   */

  private handleSpeechResult(event: SpeechRecognitionEvent): void {
    const lastResultIndex = event.results.length - 1;
    const result = event.results[lastResultIndex];
    
    if (!result) return;

    const speechResult: SpeechResult = {
      transcript: result[0].transcript,
      confidence: result[0].confidence,
      isFinal: result.isFinal,
      alternatives: Array.from(result).map((alt: SpeechRecognitionAlternative) => ({
        transcript: alt.transcript,
        confidence: alt.confidence,
      })),
    };

    this.lastResult = speechResult;

    // Only process final results with sufficient confidence
    if (speechResult.isFinal && speechResult.confidence >= this.settings.confidenceThreshold) {
      this.processVoiceCommand(speechResult);
    }
  }

  private processVoiceCommand(result: SpeechResult): void {
    const normalizedTranscript = this.normalizePhrase(result.transcript);
    
    // Try exact match first
    let command = this.commandMap.get(normalizedTranscript);
    
    // If no exact match, try fuzzy matching
    if (!command) {
      const matchedCommand = this.findBestMatch(normalizedTranscript);
      command = matchedCommand || undefined;
    }

    if (command) {
      console.log(`Executing voice command: "${command.phrase}"`);
      
      // Set confidence from speech recognition
      command.confidence = result.confidence;
      
      // Notify listeners
      this.listeners.forEach(callback => {
        try {
          callback(command!, result);
        } catch (error) {
          console.error('Error in voice command callback:', error);
        }
      });
      
      // Provide feedback
      this.speakFeedback(`Executing ${command.phrase}`);
      
      // Handle built-in system commands
      this.handleSystemCommand(command);
    } else {
      console.log(`No matching command for: "${result.transcript}"`);
      this.speakFeedback('Command not recognized');
    }
  }

  private handleSystemCommand(command: VoiceCommand): void {
    switch (command.action) {
      case 'stop-voice':
        this.stopListening();
        this.speakFeedback('Voice recognition stopped');
        break;
        
      case 'mute-voice':
        this.setSpeechEnabled(false);
        break;
        
      case 'unmute-voice':
        this.setSpeechEnabled(true);
        this.speakFeedback('Voice feedback enabled');
        break;
    }
  }

  private normalizePhrase(phrase: string): string {
    return phrase.toLowerCase()
      .trim()
      .replace(/[^\w\s]/g, '') // Remove punctuation
      .replace(/\s+/g, ' ');   // Normalize whitespace
  }

  private findBestMatch(transcript: string): VoiceCommand | null {
    let bestMatch: VoiceCommand | null = null;
    let bestScore = 0;

    for (const [phrase, command] of this.commandMap) {
      const score = this.calculateSimilarity(transcript, phrase);
      if (score > bestScore && score > 0.6) { // 60% similarity threshold
        bestScore = score;
        bestMatch = command;
      }
    }

    return bestMatch;
  }

  private calculateSimilarity(str1: string, str2: string): number {
    // Simple Levenshtein distance-based similarity
    const longer = str1.length > str2.length ? str1 : str2;
    const shorter = str1.length > str2.length ? str2 : str1;
    
    if (longer.length === 0) return 1.0;
    
    const distance = this.levenshteinDistance(longer, shorter);
    return (longer.length - distance) / longer.length;
  }

  private levenshteinDistance(str1: string, str2: string): number {
    const matrix: number[][] = [];
    
    for (let i = 0; i <= str2.length; i++) {
      matrix[i] = [i];
    }
    
    for (let j = 0; j <= str1.length; j++) {
      matrix[0][j] = j;
    }
    
    for (let i = 1; i <= str2.length; i++) {
      for (let j = 1; j <= str1.length; j++) {
        if (str2.charAt(i - 1) === str1.charAt(j - 1)) {
          matrix[i][j] = matrix[i - 1][j - 1];
        } else {
          matrix[i][j] = Math.min(
            matrix[i - 1][j - 1] + 1,
            matrix[i][j - 1] + 1,
            matrix[i - 1][j] + 1
          );
        }
      }
    }
    
    return matrix[str2.length][str1.length];
  }

  private handleRecognitionError(error: string): void {
    console.error('Speech recognition error:', error);
    
    switch (error) {
      case 'not-allowed':
        this.speakFeedback('Microphone access denied');
        break;
      case 'no-speech':
        console.log('No speech detected');
        break;
      case 'audio-capture':
        this.speakFeedback('Audio capture failed');
        break;
      case 'network':
        this.speakFeedback('Network error');
        break;
      default:
        this.speakFeedback('Voice recognition error');
    }
  }

  /**
   * Destroy voice command system
   */
  public destroy(): void {
    this.stopListening();
    if (this.synthesis) {
      this.synthesis.cancel();
    }
    this.listeners = [];
    this.commandMap.clear();
    console.log('Voice command system destroyed');
  }
}

export default VoiceCommandSystem;
