import { Agent } from './base';
import { AgentContext, AgentOpinion } from '../types';
import { LLMProvider } from '../llm/provider';

export class SecurityAgent implements Agent {
  readonly id = 'security';
  readonly name = 'Security Agent';
  readonly description =
    'Identifies safety risks, maps privilege boundaries, and validates containment policies.';

  constructor(private provider: LLMProvider) {}

  async analyze(context: AgentContext): Promise<AgentOpinion> {
    const systemPrompt = `You are the Security Agent for PyxisOS. Your role is safety-framing, identifying security risks, privilege boundaries, and containment policies.
Analyze the user query, focus on potential execution risks (e.g., shell command execution, container escapes, unauthorized host filesystem access, untrusted network operations) and outline mitigation requirements.

You MUST respond ONLY with a raw JSON object matching the following structure:
{
  "opinion": "your detailed security assessment, vulnerability notes, and risk constraints",
  "confidence": 0.95,
  "rationale": "why this confidence level was chosen based on the risk profile of the request"
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
        opinion: `Security screening failed: ${error.message}`,
        confidence: 0.0,
        rationale: `Failed to execute security screen due to error: ${error.message}`,
      };
    }
  }
}
