import Anthropic from '@anthropic-ai/sdk';
import { LLMProvider, LLMRequest } from './provider';

export class AnthropicProvider implements LLMProvider {
  private client: Anthropic | null = null;
  private model: string;

  constructor() {
    const apiKey = process.env.ANTHROPIC_API_KEY;
    this.model = process.env.ANTHROPIC_MODEL || 'claude-3-5-sonnet-20241022';

    if (apiKey) {
      this.client = new Anthropic({ apiKey });
    }
  }

  async send(request: LLMRequest): Promise<string> {
    if (!this.client) {
      throw new Error(
        'Anthropic client is not initialized. Please ensure ANTHROPIC_API_KEY is configured in the environment.'
      );
    }

    const response = await this.client.messages.create({
      model: this.model,
      max_tokens: request.maxTokens || 1524,
      temperature: request.temperature ?? 0.0,
      system: request.systemPrompt,
      messages: [{ role: 'user', content: request.userPrompt }],
    });

    return response.content
      .filter((c) => c.type === 'text')
      .map((c) => (c.type === 'text' ? c.text : ''))
      .join('\n');
  }
}
