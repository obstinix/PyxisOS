export interface LLMRequest {
  systemPrompt: string;
  userPrompt: string;
  temperature?: number;
  maxTokens?: number;
}

export interface LLMProvider {
  send(request: LLMRequest): Promise<string>;
}
