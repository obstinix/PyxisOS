#!/usr/bin/env python3
"""
PyxisOS Research Evaluation: Single-Agent Baseline Runner
Runs prompts through a single-agent baseline model and writes raw results.
"""

import json
import os
import sys
import time
import urllib.request
import urllib.error

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
PROMPTS_PATH = os.path.join(SCRIPT_DIR, "prompts.json")
RESULTS_DIR = os.path.join(SCRIPT_DIR, "..", "results")
OUTPUT_PATH = os.path.join(RESULTS_DIR, "baseline-raw.json")

SYSTEM_PROMPT = (
    "You are a generalist AI assistant designed to solve user queries.\n"
    "Address the prompt accurately, research key facts where needed, "
    "identify potential security risks or policy bounds, and ensure your "
    "reasoning is logically consistent.\n"
    "Explain your rationale clearly."
)

def call_anthropic(prompt: str, api_key: str, model: str) -> str:
    url = "https://api.anthropic.com/v1/messages"
    payload = {
        "model": model,
        "max_tokens": 1524,
        "system": SYSTEM_PROMPT,
        "messages": [{"role": "user", "content": prompt}],
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
    os.makedirs(RESULTS_DIR, exist_ok=True)
    api_key = os.environ.get("ANTHROPIC_API_KEY")
    model = os.environ.get("ANTHROPIC_MODEL", "claude-3-5-sonnet-20241022")

    if not os.path.exists(PROMPTS_PATH):
        print(f"Error: Prompts file not found at {PROMPTS_PATH}", file=sys.stderr)
        sys.exit(1)

    with open(PROMPTS_PATH, "r", encoding="utf-8") as f:
        prompts = json.load(f)

    if not api_key:
        print("ANTHROPIC_API_KEY environment variable is not set. Cannot run evaluation baseline.")
        sys.exit(0)

    print(f"Starting baseline evaluation using model {model}...")
    outputs = []

    for item in prompts:
        pid = item["id"]
        cat = item["category"]
        prompt_text = item["prompt"]
        print(f"Running baseline prompt: {pid} ({cat})...")
        start_time = time.time()
        try:
            resp = call_anthropic(prompt_text, api_key, model)
            latency_ms = int((time.time() - start_time) * 1000)
            outputs.append({
                "id": pid,
                "category": cat,
                "prompt": prompt_text,
                "response": resp,
                "latencyMs": latency_ms,
                "model": model,
            })
        except Exception as e:
            print(f"Error running prompt {pid}: {e}", file=sys.stderr)
            latency_ms = int((time.time() - start_time) * 1000)
            outputs.append({
                "id": pid,
                "category": cat,
                "prompt": prompt_text,
                "error": str(e),
                "latencyMs": latency_ms,
                "model": model,
            })

    with open(OUTPUT_PATH, "w", encoding="utf-8") as f:
        json.dump(outputs, f, indent=2)

    print(f"Baseline evaluation complete. Raw results written to: {OUTPUT_PATH}")

if __name__ == "__main__":
    main()
