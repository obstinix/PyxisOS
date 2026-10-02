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

2. Run the baseline evaluator:
   ```bash
   python research/eval/run_baseline.py
   ```

3. Run the consensus evaluator:
   ```bash
   python research/eval/run_consensus.py
   ```

4. Run the scorer script to grade both outputs and generate the evaluation report:
   ```bash
   python research/eval/score.py
   ```
