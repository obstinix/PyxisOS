import * as fs from 'fs';
import * as path from 'path';
import * as dotenv from 'dotenv';

// Import from the consensus workspace
import { Router } from '../../consensus/src/routing/router';
import { DecisionArbitrationLayer } from '../../consensus/src/arbitration/decisionArbitrationLayer';
import { AnthropicProvider } from '../../consensus/src/llm/anthropicProvider';
import { ResearchAgent } from '../../consensus/src/agents/research';
import { SecurityAgent } from '../../consensus/src/agents/security';
import { LogicAgent } from '../../consensus/src/agents/logic';
import { Logger } from '../../consensus/src/audit/logger';

// Load env from local consensus directory
dotenv.config({ path: path.join(__dirname, '../../consensus/.env') });

const promptsPath = path.join(__dirname, 'prompts.json');
const resultsDir = path.join(__dirname, '../results');
const outputPath = path.join(resultsDir, 'consensus-raw.json');

const apiKey = process.env.ANTHROPIC_API_KEY;

async function run() {
  if (!fs.existsSync(resultsDir)) {
    fs.mkdirSync(resultsDir, { recursive: true });
  }

  if (!apiKey) {
    console.error('ANTHROPIC_API_KEY environment variable is not set. Cannot run evaluation consensus pipeline.');
    process.exit(0);
  }

  console.log('Starting consensus engine evaluation pipeline...');

  // Setup Consensus Engine dependencies
  const provider = new AnthropicProvider();
  const logger = new Logger();

  const researchAgent = new ResearchAgent(provider);
  const securityAgent = new SecurityAgent(provider);
  const logicAgent = new LogicAgent(provider);

  const arbitrationLayer = new DecisionArbitrationLayer(provider, logger);
  const router = new Router([researchAgent, securityAgent, logicAgent]);

  const prompts = JSON.parse(fs.readFileSync(promptsPath, 'utf8'));
  const outputs = [];

  for (const item of prompts) {
    console.log(`Running consensus prompt: ${item.id} (${item.category})...`);
    const startTime = Date.now();
    try {
      // 1. Route query to select relevant agents (MVP: routes to all 3)
      const context = { query: item.prompt, timestamp: Date.now() };
      const selectedAgents = await router.route(context);

      // 2. Call selected agents in parallel to obtain opinions
      const opinionPromises = selectedAgents.map(async (agent) => {
        const opinion = await agent.analyze(context);
        return {
          agentId: agent.id,
          opinion,
        };
      });
      const opinions = await opinionPromises;

      // 3. Arbitrate consensus / resolve conflicts
      const result = await arbitrationLayer.arbitrate(context, opinions);
      const latencyMs = Date.now() - startTime;

      outputs.push({
        id: item.id,
        category: item.category,
        prompt: item.prompt,
        response: result.synthesizedDecision,
        opinions: opinions.map((o) => ({
          agentId: o.agentId,
          opinion: o.opinion.opinion,
          confidence: o.opinion.confidence,
          rationale: o.opinion.rationale,
        })),
        conflictFlagged: result.conflictFlagged,
        conflictDetails: result.conflictDetails,
        latencyMs,
      });
    } catch (error: any) {
      console.error(`Error running prompt ${item.id} in consensus pipeline:`, error.message);
      outputs.push({
        id: item.id,
        category: item.category,
        prompt: item.prompt,
        error: error.message,
        latencyMs: Date.now() - startTime,
      });
    }
  }

  fs.writeFileSync(outputPath, JSON.stringify(outputs, null, 2), 'utf8');
  console.log(`Consensus evaluation complete. Raw results written to: ${outputPath}`);
}

run().catch((err) => {
  console.error('Unexpected error running consensus pipeline:', err);
  process.exit(1);
});
