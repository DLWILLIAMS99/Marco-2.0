/**
 * Marco 2.0 - AI Code Assistant
 * AI-powered code generation and development assistance
 */

import { analytics } from '../analytics/analytics';
import { crashReporter } from '../analytics/crash-reporter';

// Core type definitions for AI assistant
export interface MetaValue {
  type: 'scalar' | 'bool' | 'string' | 'color' | 'list' | 'object';
  value: any;
}

export interface LogicNode {
  id: string;
  type: string;
  inputs?: Record<string, any>;
  outputs?: Record<string, any>;
  properties?: Record<string, any>;
}

export interface AIPrompt {
  id: string;
  text: string;
  context: {
    currentScope?: string;
    selectedNodes?: string[];
    recentActions?: string[];
    userPreferences?: Record<string, any>;
  };
  metadata: {
    timestamp: number;
    sessionId: string;
    userId?: string;
    intentType?: IntentType;
  };
}

export interface AIResponse {
  id: string;
  promptId: string;
  type: 'code_generation' | 'suggestion' | 'explanation' | 'optimization' | 'documentation';
  content: {
    primary: string;
    alternatives?: string[];
    confidence: number;
    reasoning: string;
  };
  actionable: {
    nodes?: LogicNode[];
    connections?: ConnectionSuggestion[];
    modifications?: NodeModification[];
    code?: GeneratedCode;
  };
  metadata: {
    processingTime: number;
    tokensUsed: number;
    modelVersion: string;
    timestamp: number;
  };
}

export interface ConnectionSuggestion {
  id: string;
  sourceNodeId: string;
  targetNodeId: string;
  outputPort: string;
  inputPort: string;
  confidence: number;
  reasoning: string;
  dataType: string;
  transformRequired?: DataTransform;
}

export interface OptimizationSuggestion {
  id: string;
  type: 'performance' | 'memory' | 'readability' | 'maintainability';
  severity: 'low' | 'medium' | 'high' | 'critical';
  description: string;
  currentImplementation: string;
  suggestedImplementation: string;
  estimatedImpact: {
    performanceGain?: number;
    memoryReduction?: number;
    complexityReduction?: number;
  };
  effort: 'low' | 'medium' | 'high';
  applicableNodes: string[];
}

export interface NodeModification {
  nodeId: string;
  type: 'property_update' | 'connection_add' | 'connection_remove' | 'replace';
  changes: Record<string, any>;
  reasoning: string;
}

export interface GeneratedCode {
  language: 'rust' | 'typescript' | 'python' | 'javascript';
  code: string;
  dependencies: string[];
  testCases?: TestCase[];
  documentation?: string;
  performance: {
    estimatedComplexity: string;
    estimatedMemory: string;
    estimatedExecutionTime: string;
  };
}

export interface TestCase {
  name: string;
  description: string;
  inputs: Record<string, any>;
  expectedOutputs: Record<string, any>;
  testCode: string;
}

export interface Documentation {
  summary: string;
  description: string;
  parameters: ParameterDoc[];
  returnValue?: ReturnValueDoc;
  examples: ExampleDoc[];
  notes?: string[];
  seeAlso?: string[];
}

export interface ParameterDoc {
  name: string;
  type: string;
  description: string;
  required: boolean;
  defaultValue?: any;
}

export interface ReturnValueDoc {
  type: string;
  description: string;
}

export interface ExampleDoc {
  title: string;
  description: string;
  code: string;
  expectedOutput?: string;
}

export type IntentType = 
  | 'create_node' 
  | 'connect_nodes' 
  | 'optimize_logic' 
  | 'generate_code' 
  | 'explain_behavior' 
  | 'debug_issue' 
  | 'refactor_code'
  | 'add_feature'
  | 'performance_tune'
  | 'generate_tests'
  | 'generate_documentation';

export interface DataTransform {
  type: 'type_conversion' | 'format_change' | 'aggregation' | 'filtering';
  description: string;
  transformFunction: string;
}

export class AICodeAssistant {
  private apiEndpoint: string;
  private apiKey: string;
  private sessionId: string;
  private conversationHistory: (AIPrompt | AIResponse)[] = [];
  private userPreferences: Record<string, any> = {};
  private cachedResponses: Map<string, AIResponse> = new Map();
  private isEnabled: boolean = true;
  private rateLimitCount: number = 0;
  private rateLimitReset: number = 0;

  constructor(config?: {
    apiEndpoint?: string;
    apiKey?: string;
    enableCaching?: boolean;
    maxCacheSize?: number;
  }) {
    this.apiEndpoint = config?.apiEndpoint || 'https://api.marco2.ai/assistant';
    this.apiKey = config?.apiKey || '';
    this.sessionId = this.generateSessionId();
    
    // Load user preferences
    this.loadUserPreferences();
    
    // Setup event listeners
    this.setupEventListeners();
    
    console.log('🤖 AI Code Assistant initialized');
  }

  // Core AI Interface Methods
  public async generateNodeFromDescription(description: string, context?: {
    currentScope?: string;
    existingNodes?: LogicNode[];
  }): Promise<LogicNode | null> {
    try {
      const prompt = this.buildNodeGenerationPrompt(description, context);
      const response = await this.sendPrompt(prompt);
      
      if (response && response.actionable.nodes && response.actionable.nodes.length > 0) {
        const node = response.actionable.nodes[0];
        
        // Track successful node generation
        analytics.trackEvent('ai_node_generated', 'user_action', {
          description,
          nodeType: node.type,
          confidence: response.content.confidence,
          processingTime: response.metadata.processingTime
        });
        
        return node;
      }
      
      return null;
    } catch (error) {
      crashReporter.reportCustomError({
        type: 'javascript_error',
        message: 'AI node generation failed',
        stack: error instanceof Error ? error.stack : undefined
      });
      return null;
    }
  }

  public async suggestConnections(selectedNodes: LogicNode[]): Promise<ConnectionSuggestion[]> {
    if (selectedNodes.length < 2) {
      return [];
    }

    try {
      const prompt = this.buildConnectionSuggestionPrompt(selectedNodes);
      const response = await this.sendPrompt(prompt);
      
      const suggestions = response?.actionable.connections || [];
      
      // Track connection suggestions
      analytics.trackEvent('ai_connections_suggested', 'user_action', {
        nodeCount: selectedNodes.length,
        suggestionsCount: suggestions.length,
        avgConfidence: suggestions.reduce((sum, s) => sum + s.confidence, 0) / suggestions.length
      });
      
      return suggestions;
    } catch (error) {
      console.error('Connection suggestion failed:', error);
      return [];
    }
  }

  public async optimizeLogicGraph(nodes: LogicNode[]): Promise<OptimizationSuggestion[]> {
    if (nodes.length === 0) {
      return [];
    }

    try {
      const prompt = this.buildOptimizationPrompt(nodes);
      const response = await this.sendPrompt(prompt);
      
      // Extract optimization suggestions from response
      const suggestions = this.parseOptimizationSuggestions(response);
      
      // Track optimization analysis
      analytics.trackEvent('ai_optimization_analysis', 'performance', {
        nodeCount: nodes.length,
        suggestionsCount: suggestions.length,
        highPrioritySuggestions: suggestions.filter(s => s.severity === 'high' || s.severity === 'critical').length
      });
      
      return suggestions;
    } catch (error) {
      console.error('Logic optimization failed:', error);
      return [];
    }
  }

  public async explainNodeBehavior(node: LogicNode): Promise<string> {
    try {
      const prompt = this.buildExplanationPrompt(node);
      const response = await this.sendPrompt(prompt);
      
      const explanation = response?.content.primary || 'Unable to generate explanation';
      
      // Track explanation requests
      analytics.trackEvent('ai_explanation_requested', 'user_action', {
        nodeType: node.type,
        explanationLength: explanation.length,
        confidence: response?.content.confidence || 0
      });
      
      return explanation;
    } catch (error) {
      console.error('Node explanation failed:', error);
      return 'Error generating explanation';
    }
  }

  public async generateDocumentation(nodes: LogicNode[], scopeName?: string): Promise<Documentation> {
    try {
      const prompt = this.buildDocumentationPrompt(nodes, scopeName);
      const response = await this.sendPrompt(prompt);
      
      const documentation = this.parseDocumentation(response);
      
      // Track documentation generation
      analytics.trackEvent('ai_documentation_generated', 'user_action', {
        nodeCount: nodes.length,
        scopeName,
        docLength: documentation.description.length
      });
      
      return documentation;
    } catch (error) {
      console.error('Documentation generation failed:', error);
      return this.createEmptyDocumentation();
    }
  }

  // Code Generation Methods
  public async generateCode(nodes: LogicNode[], language: GeneratedCode['language']): Promise<GeneratedCode | null> {
    try {
      const prompt = this.buildCodeGenerationPrompt(nodes, language);
      const response = await this.sendPrompt(prompt);
      
      const code = response?.actionable.code;
      if (!code) {
        return null;
      }
      
      // Track code generation
      analytics.trackEvent('ai_code_generated', 'user_action', {
        language,
        nodeCount: nodes.length,
        codeLength: code.code.length,
        dependencyCount: code.dependencies.length
      });
      
      return code;
    } catch (error) {
      console.error('Code generation failed:', error);
      return null;
    }
  }

  public async generateTestCases(nodes: LogicNode[]): Promise<TestCase[]> {
    try {
      const prompt = this.buildTestGenerationPrompt(nodes);
      const response = await this.sendPrompt(prompt);
      
      const testCases = this.parseTestCases(response);
      
      // Track test generation
      analytics.trackEvent('ai_tests_generated', 'user_action', {
        nodeCount: nodes.length,
        testCaseCount: testCases.length
      });
      
      return testCases;
    } catch (error) {
      console.error('Test generation failed:', error);
      return [];
    }
  }

  // Prompt Building Methods
  private buildNodeGenerationPrompt(description: string, context?: any): AIPrompt {
    const contextInfo = context ? JSON.stringify(context, null, 2) : 'No additional context';
    
    return {
      id: this.generatePromptId(),
      text: `Generate a Marco 2.0 logic node based on this description: "${description}"

Context: ${contextInfo}

Requirements:
- Return a complete LogicNode object with all required properties
- Ensure the node type is appropriate for the described functionality
- Include proper input/output definitions
- Add meaningful property defaults
- Consider integration with existing nodes if context provided

Response format: JSON object representing the LogicNode`,
      context: {
        currentScope: context?.currentScope,
        selectedNodes: context?.existingNodes?.map((n: LogicNode) => n.id),
        userPreferences: this.userPreferences
      },
      metadata: {
        timestamp: Date.now(),
        sessionId: this.sessionId,
        intentType: 'create_node'
      }
    };
  }

  private buildConnectionSuggestionPrompt(nodes: LogicNode[]): AIPrompt {
    const nodeDescriptions = nodes.map(node => ({
      id: node.id,
      type: node.type,
      outputs: Object.keys(node.outputs || {}),
      inputs: Object.keys(node.inputs || {})
    }));

    return {
      id: this.generatePromptId(),
      text: `Analyze these Marco 2.0 logic nodes and suggest meaningful connections:

Nodes: ${JSON.stringify(nodeDescriptions, null, 2)}

Requirements:
- Suggest connections that make logical sense based on data flow
- Consider type compatibility between outputs and inputs
- Provide confidence scores (0-1) for each suggestion
- Include reasoning for each suggested connection
- Identify any required data transformations

Response format: Array of ConnectionSuggestion objects`,
      context: {
        selectedNodes: nodes.map(n => n.id),
        userPreferences: this.userPreferences
      },
      metadata: {
        timestamp: Date.now(),
        sessionId: this.sessionId,
        intentType: 'connect_nodes'
      }
    };
  }

  private buildOptimizationPrompt(nodes: LogicNode[]): AIPrompt {
    const graphStructure = this.analyzeGraphStructure(nodes);

    return {
      id: this.generatePromptId(),
      text: `Analyze this Marco 2.0 logic graph for optimization opportunities:

Graph Structure: ${JSON.stringify(graphStructure, null, 2)}

Focus areas:
- Performance bottlenecks and inefficient patterns
- Memory usage optimization opportunities
- Code readability and maintainability improvements
- Redundant or unnecessary computations
- Better algorithm or data structure choices

Response format: Array of OptimizationSuggestion objects with detailed analysis`,
      context: {
        selectedNodes: nodes.map(n => n.id),
        userPreferences: this.userPreferences
      },
      metadata: {
        timestamp: Date.now(),
        sessionId: this.sessionId,
        intentType: 'optimize_logic'
      }
    };
  }

  private buildExplanationPrompt(node: LogicNode): AIPrompt {
    return {
      id: this.generatePromptId(),
      text: `Explain the behavior and purpose of this Marco 2.0 logic node:

Node: ${JSON.stringify(node, null, 2)}

Requirements:
- Provide a clear, concise explanation of what the node does
- Explain how inputs are processed to produce outputs
- Describe any side effects or special behaviors
- Use technical but accessible language
- Include practical usage examples if helpful

Response format: Plain text explanation`,
      context: {
        userPreferences: this.userPreferences
      },
      metadata: {
        timestamp: Date.now(),
        sessionId: this.sessionId,
        intentType: 'explain_behavior'
      }
    };
  }

  private buildDocumentationPrompt(nodes: LogicNode[], scopeName?: string): AIPrompt {
    return {
      id: this.generatePromptId(),
      text: `Generate comprehensive documentation for this Marco 2.0 logic implementation:

Scope: ${scopeName || 'Unnamed scope'}
Nodes: ${JSON.stringify(nodes.map(n => ({ id: n.id, type: n.type })), null, 2)}

Requirements:
- Create a clear summary and detailed description
- Document all parameters and their purposes
- Include usage examples and best practices
- Note any limitations or special considerations
- Provide see-also references to related concepts

Response format: Documentation object with all required fields`,
      context: {
        selectedNodes: nodes.map(n => n.id),
        userPreferences: this.userPreferences
      },
      metadata: {
        timestamp: Date.now(),
        sessionId: this.sessionId,
        intentType: 'generate_documentation'
      }
    };
  }

  private buildCodeGenerationPrompt(nodes: LogicNode[], language: GeneratedCode['language']): AIPrompt {
    return {
      id: this.generatePromptId(),
      text: `Generate ${language} code from this Marco 2.0 logic graph:

Nodes: ${JSON.stringify(nodes, null, 2)}

Requirements:
- Generate clean, idiomatic ${language} code
- Include all necessary imports/dependencies
- Add appropriate error handling
- Include performance optimizations where applicable
- Generate corresponding test cases
- Add inline documentation and comments

Response format: GeneratedCode object with complete implementation`,
      context: {
        selectedNodes: nodes.map(n => n.id),
        userPreferences: this.userPreferences
      },
      metadata: {
        timestamp: Date.now(),
        sessionId: this.sessionId,
        intentType: 'generate_code'
      }
    };
  }

  private buildTestGenerationPrompt(nodes: LogicNode[]): AIPrompt {
    return {
      id: this.generatePromptId(),
      text: `Generate comprehensive test cases for this Marco 2.0 logic:

Nodes: ${JSON.stringify(nodes, null, 2)}

Requirements:
- Create unit tests for individual nodes
- Create integration tests for node combinations
- Include edge cases and error conditions
- Test with various input types and ranges
- Ensure good test coverage of all logic paths

Response format: Array of TestCase objects with complete test implementations`,
      context: {
        selectedNodes: nodes.map(n => n.id),
        userPreferences: this.userPreferences
      },
      metadata: {
        timestamp: Date.now(),
        sessionId: this.sessionId,
        intentType: 'generate_tests'
      }
    };
  }

  // AI Communication Methods
  private async sendPrompt(prompt: AIPrompt): Promise<AIResponse | null> {
    if (!this.isEnabled || !this.checkRateLimit()) {
      return null;
    }

    // Check cache first
    const cacheKey = this.generateCacheKey(prompt);
    const cached = this.cachedResponses.get(cacheKey);
    if (cached && this.isCacheValid(cached)) {
      return cached;
    }

    try {
      const startTime = performance.now();
      
      // In a real implementation, this would make an actual API call
      // For now, we'll simulate the response structure
      const response = await this.simulateAIResponse(prompt);
      
      const processingTime = performance.now() - startTime;
      response.metadata.processingTime = processingTime;
      
      // Update conversation history
      this.conversationHistory.push(prompt, response);
      
      // Cache the response
      this.cachedResponses.set(cacheKey, response);
      
      // Track API usage
      analytics.trackEvent('ai_api_call', 'system', {
        promptType: prompt.metadata.intentType,
        processingTime,
        tokensUsed: response.metadata.tokensUsed,
        cached: false
      });
      
      return response;
      
    } catch (error) {
      crashReporter.reportCustomError({
        type: 'javascript_error',
        message: 'AI API call failed',
        stack: error instanceof Error ? error.stack : undefined
      });
      return null;
    }
  }

  private async simulateAIResponse(prompt: AIPrompt): Promise<AIResponse> {
    // Simulate AI processing delay
    await new Promise(resolve => setTimeout(resolve, 500 + Math.random() * 1000));
    
    const response: AIResponse = {
      id: this.generateResponseId(),
      promptId: prompt.id,
      type: this.inferResponseType(prompt.metadata.intentType!),
      content: {
        primary: this.generateMockContent(prompt),
        confidence: 0.85 + Math.random() * 0.15,
        reasoning: 'Based on analysis of the provided context and Marco 2.0 patterns'
      },
      actionable: this.generateMockActionable(prompt),
      metadata: {
        processingTime: 0, // Will be set by caller
        tokensUsed: 150 + Math.floor(Math.random() * 300),
        modelVersion: 'marco2-assistant-v1.0',
        timestamp: Date.now()
      }
    };
    
    return response;
  }

  // Utility Methods
  private analyzeGraphStructure(nodes: LogicNode[]): any {
    return {
      nodeCount: nodes.length,
      nodeTypes: [...new Set(nodes.map(n => n.type))],
      avgComplexity: this.calculateAvgComplexity(nodes),
      hasLoops: this.detectLoops(nodes),
      depth: this.calculateGraphDepth(nodes)
    };
  }

  private calculateAvgComplexity(nodes: LogicNode[]): number {
    // Simplified complexity calculation
    return nodes.reduce((sum, node) => {
      const inputCount = Object.keys(node.inputs || {}).length;
      const outputCount = Object.keys(node.outputs || {}).length;
      return sum + (inputCount + outputCount);
    }, 0) / nodes.length;
  }

  private detectLoops(nodes: LogicNode[]): boolean {
    // Simplified loop detection
    return false; // TODO: Implement proper cycle detection
  }

  private calculateGraphDepth(nodes: LogicNode[]): number {
    // Simplified depth calculation
    return Math.ceil(Math.sqrt(nodes.length));
  }

  private parseOptimizationSuggestions(response: AIResponse | null): OptimizationSuggestion[] {
    if (!response) return [];
    
    // Parse from response content
    // This would parse actual AI response in real implementation
    return [
      {
        id: this.generateSuggestionId(),
        type: 'performance',
        severity: 'medium',
        description: 'Consider combining sequential scalar operations into a single node',
        currentImplementation: 'Multiple scalar nodes in sequence',
        suggestedImplementation: 'Single composite mathematical operation node',
        estimatedImpact: {
          performanceGain: 15,
          complexityReduction: 20
        },
        effort: 'low',
        applicableNodes: ['node-1', 'node-2']
      }
    ];
  }

  private parseDocumentation(response: AIResponse | null): Documentation {
    if (!response) return this.createEmptyDocumentation();
    
    // Parse from response content
    return {
      summary: 'AI-generated logic documentation',
      description: response.content.primary,
      parameters: [],
      examples: [],
      notes: ['Generated by Marco 2.0 AI Assistant']
    };
  }

  private parseTestCases(response: AIResponse | null): TestCase[] {
    if (!response) return [];
    
    // Parse from response content
    return [
      {
        name: 'Basic functionality test',
        description: 'Tests the core functionality of the logic',
        inputs: { value: 10 },
        expectedOutputs: { result: 20 },
        testCode: 'assert(logic.process({value: 10}).result === 20);'
      }
    ];
  }

  private createEmptyDocumentation(): Documentation {
    return {
      summary: 'No documentation available',
      description: 'Documentation could not be generated',
      parameters: [],
      examples: []
    };
  }

  private inferResponseType(intentType: IntentType): AIResponse['type'] {
    switch (intentType) {
      case 'create_node':
      case 'generate_code':
        return 'code_generation';
      case 'connect_nodes':
        return 'suggestion';
      case 'explain_behavior':
        return 'explanation';
      case 'optimize_logic':
        return 'optimization';
      case 'generate_tests':
        return 'documentation';
      default:
        return 'suggestion';
    }
  }

  private generateMockContent(prompt: AIPrompt): string {
    const intentType = prompt.metadata.intentType;
    
    switch (intentType) {
      case 'create_node':
        return 'Generated a new logic node based on your description';
      case 'connect_nodes':
        return 'Found several potential connections between the selected nodes';
      case 'explain_behavior':
        return 'This node processes input data and applies mathematical transformations';
      case 'optimize_logic':
        return 'Identified opportunities to improve performance and reduce complexity';
      default:
        return 'AI analysis complete';
    }
  }

  private generateMockActionable(prompt: AIPrompt): AIResponse['actionable'] {
    const intentType = prompt.metadata.intentType;
    
    if (intentType === 'create_node') {
      return {
        nodes: [{
          id: 'ai-generated-node',
          type: 'scalar',
          inputs: { value: { type: 'number', defaultValue: 0 } },
          outputs: { result: { type: 'number' } },
          properties: { operation: 'multiply', factor: 2 }
        } as LogicNode]
      };
    }
    
    if (intentType === 'connect_nodes') {
      return {
        connections: [{
          id: 'conn-1',
          sourceNodeId: 'node-1',
          targetNodeId: 'node-2',
          outputPort: 'result',
          inputPort: 'value',
          confidence: 0.9,
          reasoning: 'Data types are compatible and flow makes logical sense',
          dataType: 'number'
        }]
      };
    }
    
    return {};
  }

  private checkRateLimit(): boolean {
    const now = Date.now();
    if (now > this.rateLimitReset) {
      this.rateLimitCount = 0;
      this.rateLimitReset = now + 60000; // 1 minute
    }
    
    if (this.rateLimitCount >= 100) { // 100 requests per minute
      console.warn('AI Assistant rate limit exceeded');
      return false;
    }
    
    this.rateLimitCount++;
    return true;
  }

  private generateCacheKey(prompt: AIPrompt): string {
    // Create a hash of the prompt content for caching
    return `${prompt.metadata.intentType}-${prompt.text.slice(0, 100)}`;
  }

  private isCacheValid(response: AIResponse): boolean {
    const maxAge = 300000; // 5 minutes
    return Date.now() - response.metadata.timestamp < maxAge;
  }

  private setupEventListeners(): void {
    // Listen for user preference changes
    document.addEventListener('user-preferences-updated', (event: any) => {
      this.userPreferences = { ...this.userPreferences, ...event.detail };
    });
    
    // Listen for system events that might affect AI behavior
    document.addEventListener('marco2-system-error', () => {
      this.isEnabled = false;
      setTimeout(() => { this.isEnabled = true; }, 30000); // Re-enable after 30 seconds
    });
  }

  private loadUserPreferences(): void {
    // Load from localStorage or default settings
    const stored = localStorage.getItem('marco2-ai-preferences');
    if (stored) {
      try {
        this.userPreferences = JSON.parse(stored);
      } catch (error) {
        console.warn('Failed to parse AI preferences');
      }
    }
    
    // Set defaults
    this.userPreferences = {
      preferredLanguages: ['typescript', 'rust'],
      codingStyle: 'functional',
      explanationLevel: 'detailed',
      generateTests: true,
      includeComments: true,
      ...this.userPreferences
    };
  }

  private generateSessionId(): string {
    return `ai_session_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
  }

  private generatePromptId(): string {
    return `prompt_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
  }

  private generateResponseId(): string {
    return `response_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
  }

  private generateSuggestionId(): string {
    return `suggestion_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
  }

  // Public API
  public getConversationHistory(): (AIPrompt | AIResponse)[] {
    return [...this.conversationHistory];
  }

  public clearConversationHistory(): void {
    this.conversationHistory = [];
    console.log('AI conversation history cleared');
  }

  public updateUserPreferences(preferences: Partial<typeof this.userPreferences>): void {
    this.userPreferences = { ...this.userPreferences, ...preferences };
    localStorage.setItem('marco2-ai-preferences', JSON.stringify(this.userPreferences));
    
    // Emit event for other components
    document.dispatchEvent(new CustomEvent('user-preferences-updated', { detail: preferences }));
  }

  public isAIEnabled(): boolean {
    return this.isEnabled && this.apiKey.length > 0;
  }

  public getUsageStats(): {
    promptsSent: number;
    cacheHits: number;
    avgProcessingTime: number;
    totalTokensUsed: number;
  } {
    const responses = this.conversationHistory.filter(item => 'promptId' in item) as AIResponse[];
    
    return {
      promptsSent: responses.length,
      cacheHits: responses.filter(r => r.metadata.processingTime < 100).length,
      avgProcessingTime: responses.reduce((sum, r) => sum + r.metadata.processingTime, 0) / responses.length || 0,
      totalTokensUsed: responses.reduce((sum, r) => sum + r.metadata.tokensUsed, 0)
    };
  }

  public async testConnection(): Promise<boolean> {
    try {
      const testPrompt: AIPrompt = {
        id: 'test-connection',
        text: 'Test connection',
        context: {},
        metadata: {
          timestamp: Date.now(),
          sessionId: this.sessionId,
          intentType: 'explain_behavior'
        }
      };
      
      const response = await this.sendPrompt(testPrompt);
      return response !== null;
    } catch (error) {
      return false;
    }
  }

  public destroy(): void {
    this.clearConversationHistory();
    this.cachedResponses.clear();
    this.isEnabled = false;
    console.log('AI Code Assistant destroyed');
  }
}

// Global AI assistant instance
export const aiCodeAssistant = new AICodeAssistant();
