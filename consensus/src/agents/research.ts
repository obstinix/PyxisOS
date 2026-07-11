import { Agent } from "./base";
import { AgentContext, AgentOpinion } from "../types";
import { LLMProvider } from "../llm/provider";

export class ResearchAgent implements Agent {
  readonly id = "research";
  readonly name = "Research Agent";
  readonly description =
    "Gathers factual context, compiles technical specifications, and verifies details.";

  constructor(private provider: LLMProvider) {}

  async analyze(context: AgentContext): Promise<AgentOpinion> {
    const systemPrompt = `You are the Research Agent for PyxisOS. Your role is context-gathering, compiling technical specifications, and factual research.
Analyze the user query, focus on factual correctness, relevant background details, and reference materials.

You MUST respond ONLY with a raw JSON object matching the following structure:
{
  "opinion": "your detailed findings and context summary",
  "confidence": 0.95, 
  "rationale": "why this confidence level was chosen based on current knowledge"
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
        confidence:
          typeof parsed.confidence === "number" ? parsed.confidence : 0.5,
        rationale: parsed.rationale || "Parsed response successfully.",
      };
    } catch (error: any) {
      // Fallback in case of parsing or API issues
      return {
        opinion: `Factual research failed: ${error.message}`,
        confidence: 0.0,
        rationale: `Failed to compile research due to error: ${error.message}`,
      };
    }
  }
}
