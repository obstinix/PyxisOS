//! Astral Consensus Engine CLI Executable
//!
//! Provides command-line access to the multi-agent consensus pipeline.

use std::env;
use std::time::Instant;

use consensus::{
    AnthropicProvider, AuditLogger, ConsensusEngine, LlmProvider, MockProvider,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: consensus <query> [--mock]");
        eprintln!("Example: consensus \"Verify page table isolation for microkernel\"");
        std::process::exit(1);
    }

    let use_mock = args.iter().any(|a| a == "--mock");
    let query_parts: Vec<&str> = args[1..]
        .iter()
        .filter(|a| !a.starts_with("--"))
        .map(|s| s.as_str())
        .collect();

    let query = query_parts.join(" ");
    if query.is_empty() {
        eprintln!("Error: Query cannot be empty.");
        std::process::exit(1);
    }

    let provider: Box<dyn LlmProvider> = if use_mock {
        Box::new(MockProvider::new())
    } else {
        Box::new(AnthropicProvider::new())
    };

    let logger = AuditLogger::default_path();
    let engine = ConsensusEngine::new(logger);

    println!("\n==================================================");
    println!("PyxisOS Astral Consensus Engine (Native Rust)");
    println!("Query: \"{}\"", query);
    println!("==================================================\n");

    println!("[Router] Selecting active experts...");
    println!("[Router] Dispatched query to: Research Agent, Security Agent, Logic Agent\n");

    println!("[Agents] Analyzing request in parallel...");
    let start_time = Instant::now();
    let (opinions, result) = engine.evaluate(&query, provider.as_ref());
    let elapsed = start_time.elapsed();

    for (agent_id, op) in &opinions {
        println!("--------------------------------------------------");
        println!(
            "👤 {} Opinion (Confidence: {:.2})",
            agent_id.to_uppercase(),
            op.confidence
        );
        println!("Rationale: {}", op.rationale);
        println!("Stance/Opinion:\n{}\n", op.opinion);
    }

    println!("--------------------------------------------------");
    println!("[Arbitrator] Synthesizing consensus from specialist opinions...");

    println!("\n==================================================");
    println!("FINAL SYNTHESIZED DECISION (Completed in {:?})", elapsed);
    println!("==================================================");
    println!("{}", result.synthesized_decision);
    println!("==================================================\n");

    if result.conflict_flagged {
        println!("⚠️  CONFLICT DETECTED BETWEEN HIGH-CONFIDENCE AGENTS:");
        if let Some(ref details) = result.conflict_details {
            println!("{}", details);
        } else {
            println!("Material divergence identified in agent stances.");
        }
        println!("==================================================\n");
    }
}
