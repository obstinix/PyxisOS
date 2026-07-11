# Evaluation Results: Single-Agent Baseline vs. Multi-Agent Consensus MVP

This report records the comparative evaluation results between the single-agent Claude baseline and the multi-agent Astral Consensus Engine MVP (Research + Security + Logic).

## Status: Evaluation Harness Ready (No Run Executed)

> [!WARNING]
> No live evaluation run has been executed yet because the `ANTHROPIC_API_KEY` environment variable is not configured in the execution environment. To prevent data contamination, **no scores or results have been fabricated**.

### How to Run the Evaluation

To execute the comparison run and populate this report, follow these steps:

1. Configure your Anthropic API Key in the environment or in `consensus/.env`:
   ```bash
   export ANTHROPIC_API_KEY="your-api-key"
   ```

2. Compile the `consensus` package:
   ```bash
   cd consensus
   npm run build
   ```

3. Run the baseline evaluator:
   ```bash
   npx ts-node research/eval/run-baseline.ts
   ```

4. Run the consensus evaluator:
   ```bash
   npx ts-node research/eval/run-consensus.ts
   ```

5. Run the scorer script to grade both outputs and overwrite this document:
   ```bash
   npx ts-node research/eval/score.ts
   ```
