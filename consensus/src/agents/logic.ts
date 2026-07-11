import { Agent } from './base';
import { AgentContext, AgentOpinion } from '../types';
import { LLMProvider } from '../llm/provider';

export class LogicAgent implements Agent {
  readonly id = 'logic';
  readonly name = 'Logic Agent';
  readonly description =
    'Verifies logical consistency, identifies contradictions, and checks reasoning flow.';

  constructor(private provider: LLMProvider) {}

  async analyze(context: AgentContext): Promise<AgentOpinion> {
    const systemPrompt = `You are the Logic Agent for PyxisOS. Your role is logical verification, consistency validation, and identifying contradictions in user queries or plan drafts.
Analyze the user query, focus on structural logic, consistency, paradoxes, causal links, and overall feasibility.

You MUST respond ONLY with a raw JSON object matching the following structure:
{
  "opinion": "your detailed logical analysis, contradiction filtering, and consistency checks",
  "confidence": 0.95,
  "rationale": "why this confidence level was chosen based on the logical coherence of the request"
}

Do NOT wrap the JSON in markdown code blocks. Do NOT include any explanations outside the JSON object.`;

    try {
      const responseText = await this.provider.send({
        systemPrompt,
        userPrompt: `Query: "${context.query}"\nTimestamp: ${context.timestamp}`,
      });

      const parsed = JSON.parse(responseText.trim());
      return {
        opinion: parsed.opinion || responseText,
        confidence: typeof parsed.confidence === 'number' ? parsed.confidence : 0.5,
        rationale: parsed.rationale || 'Parsed response successfully.',
      };
    } catch (error: any) {
      return {
        opinion: `Logical screening failed: ${error.message}`,
        confidence: 0.0,
        rationale: `Failed to execute logical check due to error: ${error.message}`,
      };
    }
  }
}
