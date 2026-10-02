#!/usr/bin/env python3
"""
PyxisOS Research Evaluation: Multi-Agent Consensus Pipeline Runner
Runs prompts through the Astral Consensus Engine pipeline (Research, Security, Logic, Arbitration)
and writes raw consensus results.
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
OUTPUT_PATH = os.path.join(RESULTS_DIR, "consensus-raw.json")

RESEARCH_SYSTEM_PROMPT = (
    "You are the Research Agent for PyxisOS. Your role is context-gathering, "
    "compiling technical specifications, and factual research.\n"
    "Analyze the user query, focus on factual correctness, relevant background details, "
    "and reference materials.\n\n"
    "You MUST respond ONLY with a raw JSON object matching the following structure:\n"
    "{\n"
    '  "opinion": "your detailed findings and context summary",\n'
    '  "confidence": 0.95,\n'
    '  "rationale": "why this confidence level was chosen based on current knowledge"\n'
    "}\n\n"
    "Do NOT wrap the JSON in markdown code blocks. Do NOT include any explanations outside the JSON object."
)

SECURITY_SYSTEM_PROMPT = (
    "You are the Security Agent for PyxisOS. Your role is safety-framing, "
    "identifying security risks, privilege boundaries, and containment policies.\n"
    "Analyze the user query, focus on potential execution risks (e.g., shell command execution, "
    "container escapes, unauthorized host filesystem access, untrusted network operations) "
    "and outline mitigation requirements.\n\n"
    "You MUST respond ONLY with a raw JSON object matching the following structure:\n"
    "{\n"
    '  "opinion": "your detailed security assessment, vulnerability notes, and risk constraints",\n'
    '  "confidence": 0.95,\n'
    '  "rationale": "why this confidence level was chosen based on the risk profile of the request"\n'
    "}\n\n"
    "Do NOT wrap the JSON in markdown code blocks. Do NOT include any explanations outside the JSON object."
)

LOGIC_SYSTEM_PROMPT = (
    "You are the Logic Agent for PyxisOS. Your role is logical verification, "
    "consistency validation, and identifying contradictions in user queries or plan drafts.\n"
    "Analyze the user query, focus on structural logic, consistency, paradoxes, "
    "causal links, and overall feasibility.\n\n"
    "You MUST respond ONLY with a raw JSON object matching the following structure:\n"
    "{\n"
    '  "opinion": "your detailed logical analysis, contradiction filtering, and consistency checks",\n'
    '  "confidence": 0.95,\n'
    '  "rationale": "why this confidence level was chosen based on the logical coherence of the request"\n'
    "}\n\n"
    "Do NOT wrap the JSON in markdown code blocks. Do NOT include any explanations outside the JSON object."
)

ARBITRATION_SYSTEM_PROMPT = (
    "You are the Decision Arbitration Layer for the PyxisOS Astral Consensus Engine.\n"
    'Your role is to perform a single "judge" pass that synthesizes the opinions of '
    "specialized agents into a unified, transparent decision.\n"
    "You must also analyze the agents' opinions for any material contradictions or disagreements.\n\n"
    "Conflict Threshold Rule:\n"
    "A conflict exists if two or more high-confidence agents (confidence >= 0.7) present materially "
    "contradictory stances or disagree on the primary resolution.\n\n"
    "You MUST respond ONLY with a raw JSON object matching the following structure:\n"
    "{\n"
    '  "synthesizedDecision": "your transparent final decision resolving the query, highlighting the rationale of the agents",\n'
    '  "conflictFlagged": true/false,\n'
    '  "conflictDetails": "explanation of the contradiction if flagged, detailing which agents disagreed and on what points"\n'
    "}\n\n"
    "Do NOT wrap the JSON in markdown code blocks. Do NOT include any explanations outside the JSON object."
)

def call_anthropic(system_prompt: str, user_prompt: str, api_key: str, model: str) -> str:
    url = "https://api.anthropic.com/v1/messages"
    payload = {
        "model": model,
        "max_tokens": 1524,
        "temperature": 0.0,
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

def parse_agent_response(raw: str, default_agent: str) -> dict:
    clean = raw.strip()
    if clean.startswith("```json"):
        clean = clean[7:]
    if clean.startswith("```"):
        clean = clean[3:]
    if clean.endswith("```"):
        clean = clean[:-3]
    clean = clean.strip()
    try:
        parsed = json.loads(clean)
        return {
            "opinion": parsed.get("opinion", raw),
            "confidence": float(parsed.get("confidence", 0.5)),
            "rationale": parsed.get("rationale", "Successfully parsed."),
        }
    except Exception:
        return {
            "opinion": raw,
            "confidence": 0.5,
            "rationale": f"Unstructured response from {default_agent}.",
        }

def arbitrate(query: str, opinions: list, api_key: str, model: str) -> dict:
    formatted = "\n\n".join(
        f"[Agent: {o['agentId']}] (Confidence: {o['confidence']:.2f})\n"
        f"Opinion: {o['opinion']}\nRationale: {o['rationale']}"
        for o in opinions
    )
    user_prompt = f'Query: "{query}"\n\nAgent Opinions:\n{formatted}'
    raw = call_anthropic(ARBITRATION_SYSTEM_PROMPT, user_prompt, api_key, model)
    clean = raw.strip()
    if clean.startswith("```json"):
        clean = clean[7:]
    if clean.startswith("```"):
        clean = clean[3:]
    if clean.endswith("```"):
        clean = clean[:-3]
    clean = clean.strip()
    try:
        parsed = json.loads(clean)
        return {
            "synthesizedDecision": parsed.get("synthesizedDecision", raw),
            "conflictFlagged": bool(parsed.get("conflictFlagged", False)),
            "conflictDetails": parsed.get("conflictDetails"),
        }
    except Exception:
        return {
            "synthesizedDecision": raw,
            "conflictFlagged": False,
            "conflictDetails": None,
        }

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
        print("ANTHROPIC_API_KEY environment variable is not set. Cannot run evaluation consensus pipeline.")
        sys.exit(0)

    print("Starting consensus engine evaluation pipeline...")
    outputs = []

    for item in prompts:
        pid = item["id"]
        cat = item["category"]
        prompt_text = item["prompt"]
        print(f"Running consensus prompt: {pid} ({cat})...")
        start_time = time.time()
        try:
            # 1. Dispatch agents
            res_raw = call_anthropic(RESEARCH_SYSTEM_PROMPT, f'Query: "{prompt_text}"', api_key, model)
            sec_raw = call_anthropic(SECURITY_SYSTEM_PROMPT, f'Query: "{prompt_text}"', api_key, model)
            log_raw = call_anthropic(LOGIC_SYSTEM_PROMPT, f'Query: "{prompt_text}"', api_key, model)

            opinions = [
                {"agentId": "research", **parse_agent_response(res_raw, "research")},
                {"agentId": "security", **parse_agent_response(sec_raw, "security")},
                {"agentId": "logic", **parse_agent_response(log_raw, "logic")},
            ]

            # 2. Arbitrate consensus
            arb = arbitrate(prompt_text, opinions, api_key, model)
            latency_ms = int((time.time() - start_time) * 1000)

            outputs.append({
                "id": pid,
                "category": cat,
                "prompt": prompt_text,
                "response": arb["synthesizedDecision"],
                "opinions": opinions,
                "conflictFlagged": arb["conflictFlagged"],
                "conflictDetails": arb["conflictDetails"],
                "latencyMs": latency_ms,
            })
        except Exception as e:
            print(f"Error running prompt {pid} in consensus pipeline: {e}", file=sys.stderr)
            latency_ms = int((time.time() - start_time) * 1000)
            outputs.append({
                "id": pid,
                "category": cat,
                "prompt": prompt_text,
                "error": str(e),
                "latencyMs": latency_ms,
            })

    with open(OUTPUT_PATH, "w", encoding="utf-8") as f:
        json.dump(outputs, f, indent=2)

    print(f"Consensus evaluation complete. Raw results written to: {OUTPUT_PATH}")

if __name__ == "__main__":
    main()
