# research/ — Evaluation & Benchmarks

Evaluation harnesses and benchmark data. Primarily designed to verify if multi-agent consensus actually beats a single-agent baseline (which is the most important open question in the project) and, later, `native/aegis/` (isolation test suite) and `native/lunar-core/` (boot/regression benchmarks).

Doubles as the data source for the companion research paper — see `docs/PRD.md` Appendix B for how these map to a paper's Experimental Evaluation section.

## Evaluation Harness (`eval/`)
The `eval/` directory contains:
- `prompts.json`: 15–20 fixed prompts of varying complexity and domains (e.g., coding, logic, safety, search) to exercise Research, Security, and Logic agents.
- `rubric.md`: The multi-dimensional evaluation rubric scoring Correctness, Completeness, Safety-Awareness, and Clarity on a 1-5 scale.
- `run-baseline.ts`: Runs a single-agent Claude generalist baseline.
- `run-consensus.ts`: Runs the multi-agent Astral Consensus Engine pipeline.
- `score.ts`: Uses an LLM-judge to heuristically grade outputs against `rubric.md`.

Results are logged in `research/results/v1-baseline-vs-consensus.md`.

