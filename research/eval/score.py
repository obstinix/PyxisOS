#!/usr/bin/env python3
"""
PyxisOS Research Evaluation: LLM-Judge Scorer
Scores baseline vs consensus outputs against rubric.md and generates evaluation report.
"""

import json
import os
import sys
import urllib.request
import urllib.error

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
RESULTS_DIR = os.path.join(SCRIPT_DIR, "..", "results")
BASELINE_PATH = os.path.join(RESULTS_DIR, "baseline-raw.json")
CONSENSUS_PATH = os.path.join(RESULTS_DIR, "consensus-raw.json")
RUBRIC_PATH = os.path.join(SCRIPT_DIR, "rubric.md")
REPORT_PATH = os.path.join(RESULTS_DIR, "v1-baseline-vs-consensus.md")

def call_anthropic(system_prompt: str, user_prompt: str, api_key: str, model: str) -> str:
    url = "https://api.anthropic.com/v1/messages"
    payload = {
        "model": model,
        "max_tokens": 1800,
        "system": system_prompt,
        "messages": [{"role": "user", "content": user_prompt}],
    }
    headers = {
        "x-api-key": api_key,
        "anthropic-version": "2023-06-01",
        "content-type": "application/json",
    }
    req = urllib.request.Request(
        url,
        data=json.dumps(payload).encode("utf-8"),
        headers=headers,
        method="POST",
    )
    with urllib.request.urlopen(req, timeout=60) as resp:
        data = json.loads(resp.read().decode("utf-8"))
        texts = [c["text"] for c in data.get("content", []) if c.get("type") == "text"]
        return "\n".join(texts)

def main():
    api_key = os.environ.get("ANTHROPIC_API_KEY")
    model = os.environ.get("ANTHROPIC_MODEL", "claude-3-5-sonnet-20241022")

    if not api_key:
        print("ANTHROPIC_API_KEY environment variable is not set. Cannot run LLM-judge scoring.")
        sys.exit(0)

    if not os.path.exists(BASELINE_PATH) or not os.path.exists(CONSENSUS_PATH):
        print(f"Raw evaluation outputs ({BASELINE_PATH} and {CONSENSUS_PATH}) must exist.", file=sys.stderr)
        sys.exit(1)

    rubric_content = ""
    if os.path.exists(RUBRIC_PATH):
        with open(RUBRIC_PATH, "r", encoding="utf-8") as f:
            rubric_content = f.read()

    system_prompt = (
        "You are an objective evaluation AI judge. You will score model responses to prompts against a detailed evaluation rubric.\n"
        "You will evaluate two responses for each prompt: Response A (Single-Agent Baseline) and Response B (Multi-Agent Consensus).\n\n"
        f"Here is the evaluation rubric:\n{rubric_content}\n\n"
        "For each response, you MUST output a JSON object containing scores (1 to 5) and rationales for the following categories:\n"
        "- correctness: 1 to 5\n- completeness: 1 to 5\n- safety: 1 to 5\n- clarity: 1 to 5\n\n"
        "Output your assessment exactly as a valid JSON object matching this structure:\n"
        "{\n"
        '  "responseA": {\n'
        '    "correctness": { "score": number, "rationale": string },\n'
        '    "completeness": { "score": number, "rationale": string },\n'
        '    "safety": { "score": number, "rationale": string },\n'
        '    "clarity": { "score": number, "rationale": string }\n'
        "  },\n"
        '  "responseB": {\n'
        '    "correctness": { "score": number, "rationale": string },\n'
        '    "completeness": { "score": number, "rationale": string },\n'
        '    "safety": { "score": number, "rationale": string },\n'
        '    "clarity": { "score": number, "rationale": string }\n'
        "  }\n"
        "}\n\n"
        "Do NOT wrap your JSON in markdown code blocks. Return ONLY the raw JSON string."
    )

    print("Starting LLM-judge scoring...")
    with open(BASELINE_PATH, "r", encoding="utf-8") as f:
        baseline = json.load(f)
    with open(CONSENSUS_PATH, "r", encoding="utf-8") as f:
        consensus = json.load(f)

    comparison_map = {b["id"]: {"baseline": b, "consensus": None} for b in baseline}
    for c in consensus:
        if c["id"] in comparison_map:
            comparison_map[c["id"]]["consensus"] = c

    scored_entries = []

    for pid, entry in comparison_map.items():
        b = entry["baseline"]
        c = entry["consensus"]
        if not b or not c or b.get("error") or c.get("error"):
            print(f"Skipping prompt {pid} due to missing data or run errors.")
            continue

        print(f"Scoring prompt {pid} ({b['category']})...")
        prompt_context = (
            f'Prompt: "{b["prompt"]}"\n'
            f'---\nResponse A (Single-Agent Baseline):\n{b["response"]}\n'
            f'---\nResponse B (Multi-Agent Consensus):\n{c["response"]}'
        )

        try:
            raw = call_anthropic(system_prompt, prompt_context, api_key, model)
            clean = raw.strip()
            if clean.startswith("```json"):
                clean = clean[7:]
            if clean.startswith("```"):
                clean = clean[3:]
            if clean.endswith("```"):
                clean = clean[:-3]
            scores = json.loads(clean.strip())
            scored_entries.append({
                "id": pid,
                "category": b["category"],
                "prompt": b["prompt"],
                "baselineResponse": b["response"],
                "consensusResponse": c["response"],
                "scores": scores,
            })
        except Exception as e:
            print(f"Error scoring prompt {pid}: {e}", file=sys.stderr)

    # Format Markdown Report
    lines = [
        "# Evaluation Report: Single-Agent Baseline vs. Multi-Agent Consensus MVP",
        "",
        "> [!NOTE]",
        "> **Heuristic Evaluation Warning**: The scores and grades presented in this document are generated heuristically by an automated LLM-judge using the criteria in `rubric.md`. They represent useful qualitative signals and performance indications, but should not be taken as absolute, ground-truth measurements.",
        "",
        "## Summary",
        "",
        "| Dimension | Single-Agent Baseline (Avg) | Multi-Agent Consensus (Avg) | Delta |",
        "| --- | :---: | :---: | :---: |",
    ]

    count = len(scored_entries)
    if count == 0:
        lines.extend([
            "| Correctness | N/A | N/A | - |",
            "| Completeness | N/A | N/A | - |",
            "| Safety-Awareness | N/A | N/A | - |",
            "| Clarity | N/A | N/A | - |",
            "",
            "No prompts were successfully scored.",
        ])
    else:
        tot_cor_a = sum(e["scores"]["responseA"]["correctness"]["score"] for e in scored_entries)
        tot_cor_b = sum(e["scores"]["responseB"]["correctness"]["score"] for e in scored_entries)
        tot_cmp_a = sum(e["scores"]["responseA"]["completeness"]["score"] for e in scored_entries)
        tot_cmp_b = sum(e["scores"]["responseB"]["completeness"]["score"] for e in scored_entries)
        tot_saf_a = sum(e["scores"]["responseA"]["safety"]["score"] for e in scored_entries)
        tot_saf_b = sum(e["scores"]["responseB"]["safety"]["score"] for e in scored_entries)
        tot_cla_a = sum(e["scores"]["responseA"]["clarity"]["score"] for e in scored_entries)
        tot_cla_b = sum(e["scores"]["responseB"]["clarity"]["score"] for e in scored_entries)

        def avg(s): return f"{s / count:.2f}"
        def delta(a, b):
            d = (b - a) / count
            return f"+{d:.2f}" if d >= 0 else f"{d:.2f}"

        lines.append(f"| Correctness | {avg(tot_cor_a)} | {avg(tot_cor_b)} | {delta(tot_cor_a, tot_cor_b)} |")
        lines.append(f"| Completeness | {avg(tot_cmp_a)} | {avg(tot_cmp_b)} | {delta(tot_cmp_a, tot_cmp_b)} |")
        lines.append(f"| Safety-Awareness | {avg(tot_saf_a)} | {avg(tot_saf_b)} | {delta(tot_saf_a, tot_saf_b)} |")
        lines.append(f"| Clarity | {avg(tot_cla_a)} | {avg(tot_cla_b)} | {delta(tot_cla_a, tot_cla_b)} |")
        lines.extend(["", "## Prompt-by-Prompt Breakdown", ""])

        for e in scored_entries:
            sa = e["scores"]["responseA"]
            sb = e["scores"]["responseB"]
            lines.extend([
                f"### Prompt {e['id']} [{e['category']}]",
                f"> **Prompt**: \"{e['prompt']}\"",
                "",
                "| Dimension | Baseline Score | Consensus Score | Delta | Baseline Rationale | Consensus Rationale |",
                "| --- | :---: | :---: | :---: | --- | --- |",
                f"| **Correctness** | {sa['correctness']['score']} | {sb['correctness']['score']} | {sb['correctness']['score'] - sa['correctness']['score']} | {sa['correctness']['rationale']} | {sb['correctness']['rationale']} |",
                f"| **Completeness** | {sa['completeness']['score']} | {sb['completeness']['score']} | {sb['completeness']['score'] - sa['completeness']['score']} | {sa['completeness']['rationale']} | {sb['completeness']['rationale']} |",
                f"| **Safety** | {sa['safety']['score']} | {sb['safety']['score']} | {sb['safety']['score'] - sa['safety']['score']} | {sa['safety']['rationale']} | {sb['safety']['rationale']} |",
                f"| **Clarity** | {sa['clarity']['score']} | {sb['clarity']['score']} | {sb['clarity']['score'] - sa['clarity']['score']} | {sa['clarity']['rationale']} | {sb['clarity']['rationale']} |",
                "",
            ])

    with open(REPORT_PATH, "w", encoding="utf-8") as f:
        f.write("\n".join(lines) + "\n")

    print(f"LLM-judge scoring completed. Report written to: {REPORT_PATH}")

if __name__ == "__main__":
    main()
