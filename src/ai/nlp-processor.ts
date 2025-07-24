/**
 * Marco 2.0 - Natural Language Processor
 * Convert natural language descriptions into visual logic structures
 */

import { analytics } from '../analytics/analytics';
import { crashReporter } from '../analytics/crash-reporter';

// Define interfaces locally to avoid import dependencies
export interface LogicFlow {
  nodes: LogicFlowNode[];
  connections: LogicConnection[];
  metadata: LogicFlowMetadata;
}

export interface LogicFlowNode {
  id: string;
  type: string;
  position: { x: number; y: number };
  inputs: LogicInput[];
  outputs: LogicOutput[];
  properties: Record<string, any>;
}

export interface LogicConnection {
  id: string;
  from: { nodeId: string; outputId: string };
  to: { nodeId: string; inputId: string };
  type: 'data' | 'control';
}

export interface LogicInput {
  id: string;
  name: string;
  type: string;
  required: boolean;
}

export interface LogicOutput {
  id: string;
  name: string;
  type: string;
}

export interface LogicFlowMetadata {
  name: string;
  description: string;
  author: string;
  version: string;
  tags: string[];
  complexity: 'simple' | 'moderate' | 'complex';
}

export interface Intent {
  id: string;
  type: IntentType;
  confidence: number;
  entities: Entity[];
  action: ActionIntent;
  context: IntentContext;
  metadata: {
    timestamp: number;
    processingTime: number;
    languageDetected: string;
    complexityScore: number;
  };
}

export interface Entity {
  id: string;
  type: EntityType;
  value: string;
  normalizedValue: any;
  confidence: number;
  position: { start: number; end: number };
  dependencies: string[]; // Other entity IDs this depends on
  metadata: {
    isCore: boolean; // Critical to the intent
    alternatives: string[]; // Alternative interpretations
    disambiguation?: string; // Clarification if ambiguous
  };
}

export interface ActionIntent {
  verb: string; // 'create', 'connect', 'modify', 'analyze', etc.
  target: string; // What to act upon
  parameters: ActionParameter[];
  conditions: ActionCondition[];
  modifiers: ActionModifier[];
}

export interface ActionParameter {
  name: string;
  type: 'number' | 'string' | 'boolean' | 'object' | 'array';
  value: any;
  required: boolean;
  defaultValue?: any;
}

export interface ActionCondition {
  type: 'if' | 'when' | 'unless' | 'while';
  condition: string;
  parameters: Record<string, any>;
}

export interface ActionModifier {
  type: 'performance' | 'style' | 'behavior' | 'constraint';
  description: string;
  impact: 'low' | 'medium' | 'high';
}

export interface IntentContext {
  domain: 'math' | 'logic' | 'data' | 'ui' | 'system' | 'general';
  complexity: 'simple' | 'moderate' | 'complex' | 'advanced';
  scope: string; // Which part of the system this relates to
  dependencies: string[]; // What this intent depends on
  precedence: number; // Order of execution if multiple intents
}

export type IntentType = 
  | 'create_node'
  | 'connect_nodes' 
  | 'create_logic_flow'
  | 'modify_properties'
  | 'analyze_data'
  | 'optimize_performance'
  | 'generate_code'
  | 'create_visualization'
  | 'handle_input'
  | 'process_output'
  | 'control_flow'
  | 'data_transformation'
  | 'error_handling'
  | 'testing'
  | 'documentation';

export type EntityType =
  | 'node_type'
  | 'property_name'
  | 'data_type'
  | 'operation'
  | 'value'
  | 'condition'
  | 'variable'
  | 'function'
  | 'connection'
  | 'scope'
  | 'constraint'
  | 'modifier'
  | 'reference';

export interface ProcessingResult {
  success: boolean;
  intent: Intent | null;
  logicFlow: LogicFlow | null;
  alternatives: LogicFlow[];
  warnings: ProcessingWarning[];
  suggestions: ProcessingSuggestion[];
  confidence: number;
  processingTime: number;
}

export interface ProcessingWarning {
  type: 'ambiguity' | 'complexity' | 'performance' | 'compatibility';
  message: string;
  severity: 'low' | 'medium' | 'high';
  suggestion: string;
}

export interface ProcessingSuggestion {
  type: 'optimization' | 'clarification' | 'alternative' | 'enhancement';
  description: string;
  impact: string;
  implementation: string;
}

export interface ValidationResult {
  isValid: boolean;
  errors: ValidationError[];
  warnings: ValidationWarning[];
  suggestions: ValidationSuggestion[];
  confidence: number;
  semanticCorrectness: number;
  syntacticCorrectness: number;
}

export interface ValidationError {
  type: 'syntax' | 'semantic' | 'type' | 'logic' | 'reference';
  message: string;
  location?: string;
  severity: 'error' | 'warning' | 'info';
  fix?: string;
}

export interface ValidationWarning {
  type: 'performance' | 'readability' | 'maintenance' | 'security' | 'best_practice';
  message: string;
  suggestion: string;
  impact: 'low' | 'medium' | 'high';
}

export interface ValidationSuggestion {
  type: 'optimization' | 'refactoring' | 'alternative' | 'enhancement';
  description: string;
  impact: 'low' | 'medium' | 'high';
  effort: 'minimal' | 'moderate' | 'significant';
  code?: string; // Example implementation
}

export class NLPProcessor {
  private entityPatterns: Map<EntityType, RegExp[]> = new Map();
  private intentPatterns: Map<IntentType, PatternMatcher[]> = new Map();
  private domainKnowledge: Map<string, DomainKnowledge> = new Map();
  private processingCache: Map<string, ProcessingResult> = new Map();
  private vocabularyExpansions: Map<string, string[]> = new Map();
  private contextMemory: ContextMemory[] = [];
  private isInitialized: boolean = false;

  constructor() {
    this.initialize();
  }

  private async initialize(): Promise<void> {
    console.log('🧠 Initializing NLP Processor...');
    
    try {
      this.initializeEntityPatterns();
      this.initializeIntentPatterns();
      this.initializeDomainKnowledge();
      this.initializeVocabularyExpansions();
      
      this.isInitialized = true;
      console.log('✅ NLP Processor initialized successfully');
      
      // Track initialization
      analytics.trackEvent('nlp_processor_initialized', 'system', {
        entityPatterns: this.entityPatterns.size,
        intentPatterns: this.intentPatterns.size,
        domainKnowledge: this.domainKnowledge.size
      });
      
    } catch (error) {
      crashReporter.reportCustomError({
        type: 'javascript_error',
        message: 'NLP Processor initialization failed',
        stack: error instanceof Error ? error.stack : undefined
      });
      throw error;
    }
  }

  // Main Processing Methods
  public async parseUserIntent(input: string): Promise<Intent> {
    if (!this.isInitialized) {
      await this.initialize();
    }

    const startTime = performance.now();
    
    try {
      // Pre-process the input
      const processedInput = this.preprocessInput(input);
      
      // Extract entities
      const entities = this.extractEntities(processedInput);
      
      // Classify intent
      const intentType = this.classifyIntentType(processedInput, entities);
      
      // Extract action components
      const action = this.extractActionIntent(processedInput, entities);
      
      // Determine context
      const context = this.determineContext(processedInput, entities);
      
      // Calculate confidence
      const confidence = this.calculateIntentConfidence(intentType, entities, action);
      
      const processingTime = performance.now() - startTime;
      
      const intent: Intent = {
        id: this.generateIntentId(),
        type: intentType,
        confidence,
        entities,
        action,
        context,
        metadata: {
          timestamp: Date.now(),
          processingTime,
          languageDetected: this.detectLanguage(input),
          complexityScore: this.calculateComplexityScore(processedInput, entities)
        }
      };

      // Store in context memory
      this.addToContextMemory(intent);
      
      // Track intent parsing
      analytics.trackEvent('intent_parsed', 'user_action', {
        intentType,
        confidence,
        entityCount: entities.length,
        processingTime,
        complexity: context.complexity
      });
      
      return intent;
      
    } catch (error) {
      crashReporter.reportCustomError({
        type: 'javascript_error',
        message: 'Intent parsing failed',
        stack: error instanceof Error ? error.stack : undefined
      });
      
      // Return fallback intent
      return this.createFallbackIntent(input, performance.now() - startTime);
    }
  }

  public extractEntities(text: string): Entity[] {
    const entities: Entity[] = [];
    
    // Extract entities using pattern matching
    for (const [entityType, patterns] of this.entityPatterns.entries()) {
      const extractedEntities = this.extractEntitiesOfType(text, entityType, patterns);
      entities.push(...extractedEntities);
    }
    
    // Resolve entity dependencies
    this.resolveEntityDependencies(entities);
    
    // Normalize entity values
    entities.forEach(entity => {
      entity.normalizedValue = this.normalizeEntityValue(entity);
    });
    
    // Remove duplicate and conflicting entities
    return this.deduplicateEntities(entities);
  }

  public async generateLogicFlow(intent: Intent): Promise<LogicFlow> {
    try {
      // Generate nodes based on intent and entities
      const nodes = await this.generateNodesFromIntent(intent);
      
      // Generate connections between nodes
      const connections = await this.generateConnectionsFromIntent(intent, nodes);
      
      // Create flow metadata
      const metadata = this.generateFlowMetadata(intent, nodes);
      
      const logicFlow: LogicFlow = {
        nodes,
        connections,
        metadata
      };
      
      // Validate the generated flow
      const validation = this.validateLogicFlow(logicFlow);
      if (!validation.isValid) {
        console.warn('Generated logic flow has validation issues:', validation.errors);
      }
      
      // Track flow generation
      analytics.trackEvent('logic_flow_generated', 'system', {
        intentType: intent.type,
        nodeCount: nodes.length,
        connectionCount: connections.length,
        complexity: metadata.complexity,
        confidence: intent.confidence
      });
      
      return logicFlow;
      
    } catch (error) {
      console.error('Logic flow generation failed:', error);
      return this.createEmptyLogicFlow(intent);
    }
  }

  public validateSyntacticCorrectness(flow: LogicFlow): ValidationResult {
    const errors: ValidationError[] = [];
    const warnings: ValidationWarning[] = [];
    const suggestions: ValidationSuggestion[] = [];
    
    // Validate node structure
    flow.nodes.forEach(node => {
      const nodeValidation = this.validateNodeSyntax(node);
      errors.push(...nodeValidation.errors);
      warnings.push(...nodeValidation.warnings);
    });
    
    // Validate connections
    flow.connections.forEach(connection => {
      const connectionValidation = this.validateConnectionSyntax(connection, flow.nodes);
      errors.push(...connectionValidation.errors);
      warnings.push(...connectionValidation.warnings);
    });
    
    // Check for orphaned nodes
    const orphanedNodes = this.findOrphanedNodes(flow);
    if (orphanedNodes.length > 0) {
      warnings.push({
        type: 'readability',
        message: `Found ${orphanedNodes.length} orphaned nodes`,
        suggestion: 'Consider connecting or removing orphaned nodes',
        impact: 'medium'
      });
    }
    
    // Check for circular dependencies
    const cycles = this.detectCycles(flow);
    if (cycles.length > 0) {
      errors.push({
        type: 'logic',
        message: 'Circular dependencies detected',
        severity: 'error',
        fix: 'Break circular dependencies by restructuring connections'
      });
    }
    
    // Generate optimization suggestions
    const optimizationSuggestions = this.generateOptimizationSuggestions(flow);
    suggestions.push(...optimizationSuggestions);
    
    const syntacticCorrectness = errors.length === 0 ? 1.0 : Math.max(0, 1.0 - (errors.length * 0.2));
    const semanticCorrectness = this.calculateSemanticCorrectness(flow);
    const confidence = (syntacticCorrectness + semanticCorrectness) / 2;
    
    return {
      isValid: errors.length === 0,
      errors,
      warnings,
      suggestions,
      confidence,
      syntacticCorrectness,
      semanticCorrectness
    };
  }

  // Advanced Processing Methods
  private preprocessInput(input: string): string {
    let processed = input.trim().toLowerCase();
    
    // Expand contractions
    processed = this.expandContractions(processed);
    
    // Normalize whitespace
    processed = processed.replace(/\s+/g, ' ');
    
    // Expand vocabulary using domain knowledge
    processed = this.expandVocabulary(processed);
    
    return processed;
  }

  private extractEntitiesOfType(text: string, entityType: EntityType, patterns: RegExp[]): Entity[] {
    const entities: Entity[] = [];
    
    patterns.forEach(pattern => {
      const matches = Array.from(text.matchAll(pattern));
      
      matches.forEach(match => {
        if (match.index !== undefined) {
          const entity: Entity = {
            id: this.generateEntityId(),
            type: entityType,
            value: match[0],
            normalizedValue: match[0],
            confidence: this.calculateEntityConfidence(match, pattern),
            position: {
              start: match.index,
              end: match.index + match[0].length
            },
            dependencies: [],
            metadata: {
              isCore: this.isCoreConcept(entityType, match[0]),
              alternatives: this.findAlternatives(entityType, match[0])
            }
          };
          
          entities.push(entity);
        }
      });
    });
    
    return entities;
  }

  private classifyIntentType(text: string, entities: Entity[]): IntentType {
    // Score each intent type based on patterns and entities
    const intentScores = new Map<IntentType, number>();
    
    for (const [intentType, patterns] of this.intentPatterns.entries()) {
      let score = 0;
      
      patterns.forEach(pattern => {
        if (pattern.matches(text, entities)) {
          score += pattern.weight;
        }
      });
      
      intentScores.set(intentType, score);
    }
    
    // Find the highest scoring intent
    let bestIntent: IntentType = 'create_node';
    let bestScore = 0;
    
    for (const [intentType, score] of intentScores.entries()) {
      if (score > bestScore) {
        bestIntent = intentType;
        bestScore = score;
      }
    }
    
    return bestIntent;
  }

  private extractActionIntent(text: string, entities: Entity[]): ActionIntent {
    // Extract action verbs
    const verbs = this.extractActionVerbs(text);
    const primaryVerb = verbs.length > 0 ? verbs[0] : 'create';
    
    // Extract action target
    const target = this.extractActionTarget(text, entities);
    
    // Extract parameters
    const parameters = this.extractActionParameters(text, entities);
    
    // Extract conditions
    const conditions = this.extractActionConditions(text);
    
    // Extract modifiers
    const modifiers = this.extractActionModifiers(text, entities);
    
    return {
      verb: primaryVerb,
      target,
      parameters,
      conditions,
      modifiers
    };
  }

  private determineContext(text: string, entities: Entity[]): IntentContext {
    // Determine domain based on entities and keywords
    const domain = this.determineDomain(text, entities);
    
    // Calculate complexity based on entity count and relationships
    const complexity = this.determineComplexity(entities);
    
    // Determine scope from context memory and current entities
    const scope = this.determineScope(entities);
    
    // Find dependencies
    const dependencies = this.findDependencies(entities);
    
    return {
      domain,
      complexity,
      scope,
      dependencies,
      precedence: this.calculatePrecedence(entities)
    };
  }

  private async generateNodesFromIntent(intent: Intent): Promise<LogicFlowNode[]> {
    const nodes: LogicFlowNode[] = [];
    
    // Generate primary nodes based on intent type
    switch (intent.type) {
      case 'create_node':
        nodes.push(...this.generateNodeCreationNodes(intent));
        break;
      case 'create_logic_flow':
        nodes.push(...this.generateLogicFlowNodes(intent));
        break;
      case 'data_transformation':
        nodes.push(...this.generateDataTransformNodes(intent));
        break;
      case 'control_flow':
        nodes.push(...this.generateControlFlowNodes(intent));
        break;
      default:
        nodes.push(...this.generateGenericNodes(intent));
    }
    
    // Add supporting nodes based on entities
    const supportingNodes = this.generateSupportingNodes(intent.entities);
    nodes.push(...supportingNodes);
    
    // Optimize node placement
    this.optimizeNodePositions(nodes);
    
    return nodes;
  }

  private async generateConnectionsFromIntent(intent: Intent, nodes: LogicFlowNode[]): Promise<LogicConnection[]> {
    const connections: LogicConnection[] = [];
    
    // Generate connections based on data flow analysis
    const dataFlow = this.analyzeDataFlow(intent, nodes);
    connections.push(...this.generateDataFlowConnections(dataFlow));
    
    // Generate control flow connections
    const controlFlow = this.analyzeControlFlow(intent, nodes);
    connections.push(...this.generateControlFlowConnections(controlFlow));
    
    // Validate and optimize connections
    return this.optimizeConnections(connections, nodes);
  }

  // Entity Processing Methods
  private resolveEntityDependencies(entities: Entity[]): void {
    entities.forEach(entity => {
      // Find entities that this entity depends on
      const dependencies = entities.filter(other => 
        other.id !== entity.id && 
        this.checkEntityDependency(entity, other)
      );
      
      entity.dependencies = dependencies.map(dep => dep.id);
    });
  }

  private normalizeEntityValue(entity: Entity): any {
    switch (entity.type) {
      case 'value':
        return this.parseValue(entity.value);
      case 'data_type':
        return this.normalizeDataType(entity.value);
      case 'operation':
        return this.normalizeOperation(entity.value);
      case 'node_type':
        return this.normalizeNodeType(entity.value);
      default:
        return entity.value;
    }
  }

  private deduplicateEntities(entities: Entity[]): Entity[] {
    const deduped: Entity[] = [];
    const seen = new Set<string>();
    
    // Sort by confidence (highest first)
    const sorted = entities.sort((a, b) => b.confidence - a.confidence);
    
    sorted.forEach(entity => {
      const key = `${entity.type}:${entity.value}:${entity.position.start}`;
      if (!seen.has(key)) {
        seen.add(key);
        deduped.push(entity);
      }
    });
    
    return deduped;
  }

  // Pattern Initialization
  private initializeEntityPatterns(): void {
    // Node type patterns
    this.entityPatterns.set('node_type', [
      /\b(scalar|vector|matrix|function|condition|loop|input|output|transform)\b/g,
      /\b(math|calculation|compute|process|convert|filter|sort|merge)\s+(node|block|component)\b/g,
      /\b(add|subtract|multiply|divide|sum|average|min|max|count)\b/g
    ]);
    
    // Data type patterns
    this.entityPatterns.set('data_type', [
      /\b(number|string|boolean|array|object|matrix|vector|scalar)\b/g,
      /\b(int|integer|float|double|char|text|bool)\b/g,
      /\b(list|collection|set|map|dictionary)\b/g
    ]);
    
    // Operation patterns
    this.entityPatterns.set('operation', [
      /\b(add|subtract|multiply|divide|modulo|power|sqrt|abs|round|floor|ceil)\b/g,
      /\b(and|or|not|xor|if|then|else|while|for|repeat)\b/g,
      /\b(filter|map|reduce|sort|group|join|split|concat)\b/g
    ]);
    
    // Value patterns
    this.entityPatterns.set('value', [
      /\b\d+\.?\d*\b/g, // Numbers
      /"[^"]*"/g, // Quoted strings
      /'[^']*'/g, // Single quoted strings
      /\b(true|false|null|undefined)\b/g // Literals
    ]);
    
    // Property patterns
    this.entityPatterns.set('property_name', [
      /\bproperty\s+(\w+)/g,
      /\bset\s+(\w+)\s+to\b/g,
      /\b(\w+)\s+property\b/g
    ]);
  }

  private initializeIntentPatterns(): void {
    // Create node patterns
    this.intentPatterns.set('create_node', [
      new PatternMatcher(/\b(create|add|make|build)\s+(a\s+)?(node|block|component)\b/, 0.9),
      new PatternMatcher(/\b(new|another)\s+(\w+)\s+(node|block)\b/, 0.8),
      new PatternMatcher(/\bi\s+(want|need)\s+(a|to\s+create)\b/, 0.7)
    ]);
    
    // Connect nodes patterns
    this.intentPatterns.set('connect_nodes', [
      new PatternMatcher(/\b(connect|link|join|attach)\b/, 0.9),
      new PatternMatcher(/\bfrom\s+\w+\s+to\s+\w+\b/, 0.8),
      new PatternMatcher(/\boutput\s+to\s+input\b/, 0.8)
    ]);
    
    // Logic flow patterns
    this.intentPatterns.set('create_logic_flow', [
      new PatternMatcher(/\b(flow|sequence|pipeline|process)\b/, 0.8),
      new PatternMatcher(/\bstep\s+by\s+step\b/, 0.7),
      new PatternMatcher(/\bthen\b.*\bthen\b/, 0.6) // Multiple "then"s suggest sequence
    ]);
    
    // Data transformation patterns
    this.intentPatterns.set('data_transformation', [
      new PatternMatcher(/\b(transform|convert|change|modify)\s+(data|value|input)\b/, 0.9),
      new PatternMatcher(/\bfrom\s+\w+\s+to\s+\w+\b/, 0.7),
      new PatternMatcher(/\b(map|filter|reduce|process)\b/, 0.8)
    ]);
  }

  private initializeDomainKnowledge(): void {
    // Math domain
    this.domainKnowledge.set('math', {
      concepts: ['addition', 'subtraction', 'multiplication', 'division', 'algebra', 'calculus'],
      operations: ['add', 'subtract', 'multiply', 'divide', 'power', 'sqrt', 'sin', 'cos'],
      patterns: ['linear', 'quadratic', 'exponential', 'logarithmic'],
      complexity: 'moderate'
    });
    
    // Logic domain
    this.domainKnowledge.set('logic', {
      concepts: ['boolean', 'condition', 'branch', 'loop', 'recursion'],
      operations: ['and', 'or', 'not', 'if', 'while', 'for'],
      patterns: ['conditional', 'iterative', 'recursive'],
      complexity: 'moderate'
    });
    
    // Data domain
    this.domainKnowledge.set('data', {
      concepts: ['array', 'object', 'stream', 'transformation', 'aggregation'],
      operations: ['filter', 'map', 'reduce', 'sort', 'group', 'join'],
      patterns: ['pipeline', 'batch', 'stream'],
      complexity: 'complex'
    });
  }

  private initializeVocabularyExpansions(): void {
    // Mathematical synonyms
    this.vocabularyExpansions.set('calculate', ['compute', 'process', 'evaluate']);
    this.vocabularyExpansions.set('number', ['value', 'amount', 'quantity']);
    this.vocabularyExpansions.set('result', ['output', 'answer', 'outcome']);
    
    // Action synonyms
    this.vocabularyExpansions.set('create', ['make', 'build', 'generate', 'produce']);
    this.vocabularyExpansions.set('connect', ['link', 'join', 'attach', 'bind']);
    this.vocabularyExpansions.set('modify', ['change', 'update', 'alter', 'adjust']);
  }

  // Utility Methods
  private expandContractions(text: string): string {
    const contractions: Record<string, string> = {
      "can't": "cannot",
      "won't": "will not",
      "shouldn't": "should not",
      "couldn't": "could not",
      "wouldn't": "would not",
      "i'll": "i will",
      "you'll": "you will",
      "it's": "it is",
      "that's": "that is"
    };
    
    let expanded = text;
    for (const [contraction, expansion] of Object.entries(contractions)) {
      expanded = expanded.replace(new RegExp(contraction, 'gi'), expansion);
    }
    
    return expanded;
  }

  private expandVocabulary(text: string): string {
    let expanded = text;
    
    for (const [word, synonyms] of this.vocabularyExpansions.entries()) {
      const regex = new RegExp(`\\b${word}\\b`, 'gi');
      if (regex.test(expanded)) {
        // Add synonyms to help pattern matching
        expanded += ` (${synonyms.join('|')})`;
      }
    }
    
    return expanded;
  }

  private calculateIntentConfidence(intentType: IntentType, entities: Entity[], action: ActionIntent): number {
    let confidence = 0.5; // Base confidence
    
    // Boost confidence based on entity quality
    const avgEntityConfidence = entities.reduce((sum, e) => sum + e.confidence, 0) / entities.length || 0;
    confidence += avgEntityConfidence * 0.3;
    
    // Boost confidence if we have clear action verbs
    if (action.verb && action.verb !== 'create') {
      confidence += 0.2;
    }
    
    // Boost confidence based on entity count and types
    const coreEntities = entities.filter(e => e.metadata.isCore);
    confidence += Math.min(coreEntities.length * 0.1, 0.3);
    
    return Math.min(confidence, 1.0);
  }

  private calculateComplexityScore(text: string, entities: Entity[]): number {
    let score = 0;
    
    // Text length factor
    score += Math.min(text.length / 100, 5);
    
    // Entity count factor
    score += entities.length * 0.5;
    
    // Nested structure factor
    const nestedPatterns = text.match(/\b(if|while|for|when)\b.*\b(then|do)\b/gi);
    score += (nestedPatterns?.length || 0) * 2;
    
    // Multiple verbs suggest complexity
    const verbs = text.match(/\b(create|make|connect|transform|process|calculate)\b/gi);
    score += Math.max(0, (verbs?.length || 1) - 1);
    
    return Math.min(score, 10);
  }

  private detectLanguage(input: string): string {
    // Simple English detection (could be expanded for multi-language support)
    const englishWords = ['the', 'and', 'or', 'but', 'in', 'on', 'at', 'to', 'for', 'of', 'with', 'by'];
    const words = input.toLowerCase().split(/\s+/);
    const englishWordCount = words.filter(word => englishWords.includes(word)).length;
    
    return englishWordCount > words.length * 0.1 ? 'en' : 'unknown';
  }

  private addToContextMemory(intent: Intent): void {
    this.contextMemory.push({
      intent,
      timestamp: Date.now(),
      used: false
    });
    
    // Keep only recent context (last 10 intents)
    if (this.contextMemory.length > 10) {
      this.contextMemory.shift();
    }
  }

  private createFallbackIntent(input: string, processingTime: number): Intent {
    return {
      id: this.generateIntentId(),
      type: 'create_node',
      confidence: 0.1,
      entities: [],
      action: {
        verb: 'create',
        target: 'node',
        parameters: [],
        conditions: [],
        modifiers: []
      },
      context: {
        domain: 'general',
        complexity: 'simple',
        scope: 'global',
        dependencies: [],
        precedence: 0
      },
      metadata: {
        timestamp: Date.now(),
        processingTime,
        languageDetected: 'unknown',
        complexityScore: 1
      }
    };
  }

  private createEmptyLogicFlow(intent: Intent): LogicFlow {
    return {
      nodes: [],
      connections: [],
      metadata: {
        name: 'Empty Flow',
        description: 'Failed to generate logic flow',
        author: 'AI Assistant',
        version: '1.0.0',
        tags: ['failed'],
        complexity: 'simple'
      }
    };
  }

  // ID Generators
  private generateIntentId(): string {
    return `intent_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
  }

  private generateEntityId(): string {
    return `entity_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
  }

  // Placeholder implementations for complex methods
  private calculateEntityConfidence(match: RegExpMatchArray, pattern: RegExp): number {
    return 0.8; // Simplified confidence calculation
  }

  private isCoreConcept(entityType: EntityType, value: string): boolean {
    const coreTypes: EntityType[] = ['node_type', 'operation', 'data_type'];
    return coreTypes.includes(entityType);
  }

  private findAlternatives(entityType: EntityType, value: string): string[] {
    return this.vocabularyExpansions.get(value) || [];
  }

  private checkEntityDependency(entity: Entity, other: Entity): boolean {
    // Simplified dependency checking
    return entity.position.start > other.position.end && 
           entity.type !== other.type;
  }

  private parseValue(value: string): any {
    // Try to parse as number
    const num = parseFloat(value);
    if (!isNaN(num)) return num;
    
    // Try to parse as boolean
    if (value === 'true') return true;
    if (value === 'false') return false;
    if (value === 'null') return null;
    
    // Return as string
    return value.replace(/^["']|["']$/g, '');
  }

  private normalizeDataType(value: string): string {
    const typeMap: Record<string, string> = {
      'int': 'number',
      'integer': 'number',
      'float': 'number',
      'double': 'number',
      'char': 'string',
      'text': 'string',
      'bool': 'boolean'
    };
    
    return typeMap[value] || value;
  }

  private normalizeOperation(value: string): string {
    const opMap: Record<string, string> = {
      'plus': 'add',
      'minus': 'subtract',
      'times': 'multiply',
      'divided by': 'divide'
    };
    
    return opMap[value] || value;
  }

  private normalizeNodeType(value: string): string {
    const nodeMap: Record<string, string> = {
      'calculation': 'scalar',
      'math': 'scalar',
      'compute': 'function',
      'process': 'function'
    };
    
    return nodeMap[value] || value;
  }

  // Additional utility methods would be implemented here...
  private extractActionVerbs(text: string): string[] { return ['create']; }
  private extractActionTarget(text: string, entities: Entity[]): string { return 'node'; }
  private extractActionParameters(text: string, entities: Entity[]): ActionParameter[] { return []; }
  private extractActionConditions(text: string): ActionCondition[] { return []; }
  private extractActionModifiers(text: string, entities: Entity[]): ActionModifier[] { return []; }
  private determineDomain(text: string, entities: Entity[]): IntentContext['domain'] { return 'general'; }
  private determineComplexity(entities: Entity[]): IntentContext['complexity'] { return 'simple'; }
  private determineScope(entities: Entity[]): string { return 'global'; }
  private findDependencies(entities: Entity[]): string[] { return []; }
  private calculatePrecedence(entities: Entity[]): number { return 0; }
  private generateNodeCreationNodes(intent: Intent): LogicFlowNode[] { return []; }
  private generateLogicFlowNodes(intent: Intent): LogicFlowNode[] { return []; }
  private generateDataTransformNodes(intent: Intent): LogicFlowNode[] { return []; }
  private generateControlFlowNodes(intent: Intent): LogicFlowNode[] { return []; }
  private generateGenericNodes(intent: Intent): LogicFlowNode[] { return []; }
  private generateSupportingNodes(entities: Entity[]): LogicFlowNode[] { return []; }
  private optimizeNodePositions(nodes: LogicFlowNode[]): void {}
  private analyzeDataFlow(intent: Intent, nodes: LogicFlowNode[]): any { return {}; }
  private generateDataFlowConnections(dataFlow: any): LogicConnection[] { return []; }
  private analyzeControlFlow(intent: Intent, nodes: LogicFlowNode[]): any { return {}; }
  private generateControlFlowConnections(controlFlow: any): LogicConnection[] { return []; }
  private optimizeConnections(connections: LogicConnection[], nodes: LogicFlowNode[]): LogicConnection[] { return connections; }
  private validateNodeSyntax(node: LogicFlowNode): { errors: ValidationError[]; warnings: ValidationWarning[] } { return { errors: [], warnings: [] }; }
  private validateConnectionSyntax(connection: LogicConnection, nodes: LogicFlowNode[]): { errors: ValidationError[]; warnings: ValidationWarning[] } { return { errors: [], warnings: [] }; }
  private findOrphanedNodes(flow: LogicFlow): LogicFlowNode[] { return []; }
  private detectCycles(flow: LogicFlow): LogicConnection[][] { return []; }
  private generateOptimizationSuggestions(flow: LogicFlow): ValidationSuggestion[] { return []; }
  private calculateSemanticCorrectness(flow: LogicFlow): number { return 0.8; }
  private generateFlowMetadata(intent: Intent, nodes: LogicFlowNode[]): any { 
    return {
      name: `${intent.type} Flow`,
      description: `Generated from intent: ${intent.type}`,
      author: 'AI Assistant',
      version: '1.0.0',
      tags: [intent.type],
      complexity: intent.context.complexity
    }; 
  }
  private validateLogicFlow(flow: LogicFlow): ValidationResult {
    return {
      isValid: true,
      errors: [],
      warnings: [],
      suggestions: [],
      confidence: 0.8,
      syntacticCorrectness: 0.9,
      semanticCorrectness: 0.8
    };
  }

  // Public API
  public getProcessingStats(): any {
    return {
      cacheSize: this.processingCache.size,
      contextMemorySize: this.contextMemory.length,
      entityPatterns: this.entityPatterns.size,
      intentPatterns: this.intentPatterns.size,
      domainKnowledge: this.domainKnowledge.size
    };
  }

  public clearCache(): void {
    this.processingCache.clear();
    this.contextMemory = [];
    console.log('NLP Processor cache cleared');
  }

  public addDomainKnowledge(domain: string, knowledge: DomainKnowledge): void {
    this.domainKnowledge.set(domain, knowledge);
    console.log(`Domain knowledge added for: ${domain}`);
  }
}

// Supporting interfaces and classes
interface DomainKnowledge {
  concepts: string[];
  operations: string[];
  patterns: string[];
  complexity: 'simple' | 'moderate' | 'complex';
}

interface ContextMemory {
  intent: Intent;
  timestamp: number;
  used: boolean;
}

class PatternMatcher {
  constructor(
    public pattern: RegExp,
    public weight: number
  ) {}

  matches(text: string, entities: Entity[]): boolean {
    return this.pattern.test(text);
  }
}

// Global NLP processor instance
export const nlpProcessor = new NLPProcessor();
