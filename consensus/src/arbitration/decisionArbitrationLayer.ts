import { AgentContext, AgentOpinion, ArbitrationResult } from '../types';
import { LLMProvider } from '../llm/provider';
import { Logger } from '../audit/logger';

export class DecisionArbitrationLayer {
  constructor(
    private provider: LLMProvider,
    private logger: Logger
  ) {}

  /**
   * Arbitrates the opinions of active agents.
   * Performs a single "judge" pass to synthesize consensus and verify conflicts.
   * Deliberate v2 direction: multi-agent voting/debate protocols will be implemented here.
   */
  async arbitrate(
    context: AgentContext,
    opinions: Array<{ agentId: string; opinion: AgentOpinion }>
  ): Promise<ArbitrationResult> {
    // 1. Log all input opinions for observability
    for (const op of opinions) {
      this.logger.logAgentCall(op.agentId, context.query, op.opinion.opinion, op.opinion.confidence);
    }

    // 2. Format agent outputs for the judge pass
    const formattedOpinions = opinions
      .map(
        (o) =>
          `[Agent: ${o.agentId}] (Confidence: ${o.opinion.confidence})\nOpinion: ${o.opinion.opinion}\nRationale: ${o.opinion.rationale}`
      )
      .join('\n\n');

    // 3. Build arbitration prompt
    const systemPrompt = `You are the Decision Arbitration Layer for the PyxisOS Astral Consensus Engine.
Your role is to perform a single "judge" pass that synthesizes the opinions of specialized agents into a unified, transparent decision.
You must also analyze the agents' opinions for any material contradictions or disagreements.

Conflict Threshold Rule:
A conflict exists if two or more high-confidence agents (confidence >= 0.7) present materially contradictory stances or disagree on the primary resolution.

You MUST respond ONLY with a raw JSON object matching the following structure:
{
  "synthesizedDecision": "your transparent final decision resolving the query, highlighting the rationale of the agents",
  "conflictFlagged": true/false (set to true if a material contradiction exists between high-confidence agents),
  "conflictDetails": "explanation of the contradiction if flagged, detailing which agents disagreed and on what points"
}

Do NOT wrap the JSON in markdown code blocks. Do NOT include any explanations outside the JSON object.

NOTE: Future versions (v2) of this layer will implement voting/debate mechanics between agents. For this v1 MVP, we utilize this single LLM-judge arbitration pass.`;

    const userPrompt = `Query: "${context.query}"\n\nAgent Opinions:\n${formattedOpinions}`;

    try {
      const responseText = await this.provider.send({
        systemPrompt,
        userPrompt,
      });

      const parsed = JSON.parse(responseText.trim());
      const result: ArbitrationResult = {
        synthesizedDecision: parsed.synthesizedDecision || responseText,
        conflictFlagged: !!parsed.conflictFlagged,
        conflictDetails: parsed.conflictDetails,
      };

      // 4. Log the final arbitration result
      this.logger.logArbitration(
        context.query,
        result.synthesizedDecision,
        result.conflictFlagged,
        result.conflictDetails
      );

      return result;
    } catch (error: any) {
      const errorResult: ArbitrationResult = {
        synthesizedDecision: `Failed to arbitrate consensus: ${error.message}`,
        conflictFlagged: true,
        conflictDetails: `Arbitration system error: ${error.message}`,
      };

      this.logger.logArbitration(
        context.query,
        errorResult.synthesizedDecision,
        errorResult.conflictFlagged,
        errorResult.conflictDetails
      );

      return errorResult;
    }
  }
}
