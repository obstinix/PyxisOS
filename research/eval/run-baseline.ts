import * as fs from 'fs';
import * as path from 'path';
import Anthropic from '@anthropic-ai/sdk';
import * as dotenv from 'dotenv';

// Load env from root or local consensus directory
dotenv.config({ path: path.join(__dirname, '../../consensus/.env') });

const promptsPath = path.join(__dirname, 'prompts.json');
const resultsDir = path.join(__dirname, '../results');
const outputPath = path.join(resultsDir, 'baseline-raw.json');

const model = process.env.ANTHROPIC_MODEL || 'claude-3-5-sonnet-20241022';
const apiKey = process.env.ANTHROPIC_API_KEY;

const systemPrompt = `You are a generalist AI assistant designed to solve user queries.
Address the prompt accurately, research key facts where needed, identify potential security risks or policy bounds, and ensure your reasoning is logically consistent.
Explain your rationale clearly.`;

async function run() {
  if (!fs.existsSync(resultsDir)) {
    fs.mkdirSync(resultsDir, { recursive: true });
  }

  if (!apiKey) {
    console.error('ANTHROPIC_API_KEY environment variable is not set. Cannot run evaluation baseline.');
    process.exit(0);
  }

  console.log(`Starting baseline evaluation using model ${model}...`);
  const prompts = JSON.parse(fs.readFileSync(promptsPath, 'utf8'));
  const anthropic = new Anthropic({ apiKey });
  const outputs = [];

  for (const item of prompts) {
    console.log(`Running baseline prompt: ${item.id} (${item.category})...`);
    const startTime = Date.now();
    try {
      const response = await anthropic.messages.create({
        model,
        max_tokens: 1524,
        system: systemPrompt,
        messages: [{ role: 'user', content: item.prompt }],
      });

      const responseText = response.content
        .filter((c) => c.type === 'text')
        .map((c) => (c.type === 'text' ? c.text : ''))
        .join('\n');

      const latencyMs = Date.now() - startTime;
      outputs.push({
        id: item.id,
        category: item.category,
        prompt: item.prompt,
        response: responseText,
        latencyMs,
        model,
      });
    } catch (error: any) {
      console.error(`Error running prompt ${item.id}:`, error.message);
      outputs.push({
        id: item.id,
        category: item.category,
        prompt: item.prompt,
        error: error.message,
        latencyMs: Date.now() - startTime,
        model,
      });
    }
  }

  fs.writeFileSync(outputPath, JSON.stringify(outputs, null, 2), 'utf8');
  console.log(`Baseline evaluation complete. Raw results written to: ${outputPath}`);
}

run().catch((err) => {
  console.error('Unexpected error running baseline:', err);
  process.exit(1);
});
