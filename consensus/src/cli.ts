import { Router } from './routing/router';
import { DecisionArbitrationLayer } from './arbitration/decisionArbitrationLayer';
import { AnthropicProvider } from './llm/anthropicProvider';
import { ResearchAgent } from './agents/research';
import { SecurityAgent } from './agents/security';
import { LogicAgent } from './agents/logic';
import { Logger } from './audit/logger';
import { AgentContext } from './types';

async function main() {
  const query = process.argv.slice(2).join(' ');
  if (!query) {
    console.log('Usage: npm run cli -- "Your query string here"');
    process.exit(1);
  }

  // Check API key availability
  if (!process.env.ANTHROPIC_API_KEY) {
    console.warn(
      'Warning: ANTHROPIC_API_KEY environment variable is not configured. Live execution will fail.'
    );
  }

  const provider = new AnthropicProvider();
  const logger = new Logger();

  const researchAgent = new ResearchAgent(provider);
  const securityAgent = new SecurityAgent(provider);
  const logicAgent = new LogicAgent(provider);

  const arbitrationLayer = new DecisionArbitrationLayer(provider, logger);
  const router = new Router([researchAgent, securityAgent, logicAgent]);

  console.log(`\n==================================================`);
  console.log(`PyxisOS Astral Consensus Engine MVP (Phase III)`);
  console.log(`Query: "${query}"`);
  console.log(`==================================================\n`);

  const context: AgentContext = { query, timestamp: Date.now() };

  try {
    console.log(`[Router] Selecting active experts...`);
    const activeAgents = await router.route(context);
    console.log(`[Router] Dispatched query to: ${activeAgents.map((a) => a.name).join(', ')}\n`);

    console.log(`[Agents] Analyzing request in parallel...`);
    const opinionPromises = activeAgents.map(async (agent) => {
      const start = Date.now();
      const opinion = await agent.analyze(context);
      return {
        agentId: agent.id,
        name: agent.name,
        opinion,
        latency: Date.now() - start,
      };
    });

    const opinions = await Promise.all(opinionPromises);

    // Print individual opinions for transparent reasoning
    for (const op of opinions) {
      console.log(`--------------------------------------------------`);
      console.log(
        `👤 ${op.name} Opinion (Confidence: ${op.opinion.confidence.toFixed(2)}) [${op.latency}ms]`
      );
      console.log(`Rationale: ${op.opinion.rationale}`);
      console.log(`Stance/Opinion:\n${op.opinion.opinion}\n`);
    }

    console.log(`--------------------------------------------------`);
    console.log(`[Arbitrator] Performing single judge pass to synthesize consensus...`);
    const arbitrationInput = opinions.map((o) => ({ agentId: o.agentId, opinion: o.opinion }));
    const result = await arbitrationLayer.arbitrate(context, arbitrationInput);

    console.log(`\n==================================================`);
    console.log(`FINAL SYNTHESIZED DECISION`);
    console.log(`==================================================`);
    console.log(result.synthesizedDecision);
    console.log(`==================================================\n`);

    if (result.conflictFlagged) {
      console.log(`⚠️  CONFLICT DETECTED BETWEEN HIGH-CONFIDENCE AGENTS:`);
      console.log(result.conflictDetails);
      console.log(`==================================================\n`);
    }
  } catch (error: any) {
    console.error(`System Execution Error:`, error.message);
    process.exit(1);
  }
}

if (require.main === module) {
  main();
}
