export interface AgentContext {
  query: string;
  timestamp: number;
  [key: string]: any;
}

export interface AgentOpinion {
  opinion: string;
  confidence: number;
  rationale: string;
}

export interface ArbitrationResult {
  synthesizedDecision: string;
  conflictFlagged: boolean;
  conflictDetails?: string;
}
