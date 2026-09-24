import { Router } from "./routing/router";
import { DecisionArbitrationLayer } from "./arbitration/decisionArbitrationLayer";
import { ResearchAgent } from "./agents/research";
import { SecurityAgent } from "./agents/security";
import { LogicAgent } from "./agents/logic";
import { LLMProvider, LLMRequest } from "./llm/provider";
import { Logger } from "./audit/logger";

// Mock LLM Provider for unit testing
class MockLLMProvider implements LLMProvider {
  public responseText = "";
  async send(_request: LLMRequest): Promise<string> {
    return this.responseText;
  }
}

// Mock Logger to skip file-append operations during testing
class MockLogger extends Logger {
  log(_event: string, _metadata: Record<string, any>): void {
    // No-op
  }
}

describe("Astral Consensus Engine MVP Unit Tests", () => {
  let mockProvider: MockLLMProvider;
  let mockLogger: MockLogger;

  beforeEach(() => {
    mockProvider = new MockLLMProvider();
    mockLogger = new MockLogger();
  });

  test("Pluggable Agents properties mapping", () => {
    const research = new ResearchAgent(mockProvider);
    const security = new SecurityAgent(mockProvider);
    const logic = new LogicAgent(mockProvider);

    expect(research.id).toBe("research");
    expect(security.id).toBe("security");
    expect(logic.id).toBe("logic");

    expect(typeof research.name).toBe("string");
    expect(typeof security.description).toBe("string");
  });

  test("Router dispatches to all registered experts", async () => {
    const research = new ResearchAgent(mockProvider);
    const security = new SecurityAgent(mockProvider);
    const logic = new LogicAgent(mockProvider);

    const router = new Router([research, security, logic]);
    const context = { query: "test routing query", timestamp: Date.now() };
    const selected = await router.route(context);

    expect(selected).toHaveLength(3);
    expect(selected[0]).toBe(research);
    expect(selected[1]).toBe(security);
    expect(selected[2]).toBe(logic);
  });

  test("DecisionArbitrationLayer processes judge pass correctly", async () => {
    const arbitrator = new DecisionArbitrationLayer(mockProvider, mockLogger);
    mockProvider.responseText = JSON.stringify({
      synthesizedDecision: "Synthesized resolution.",
      conflictFlagged: false,
    });

    const context = { query: "safe query", timestamp: Date.now() };
    const opinions = [
      {
        agentId: "research",
        opinion: {
          opinion: "Safe facts",
          confidence: 0.9,
          rationale: "Factual findings",
        },
      },
    ];

    const result = await arbitrator.arbitrate(context, opinions);

    expect(result.synthesizedDecision).toBe("Synthesized resolution.");
    expect(result.conflictFlagged).toBe(false);
    expect(result.conflictDetails).toBeUndefined();
  });

  test("DecisionArbitrationLayer flags conflict on contradiction", async () => {
    const arbitrator = new DecisionArbitrationLayer(mockProvider, mockLogger);
    mockProvider.responseText = JSON.stringify({
      synthesizedDecision: "Conflicting opinions received.",
      conflictFlagged: true,
      conflictDetails: "Research and Security have opposing stances.",
    });

    const context = { query: "risky operations query", timestamp: Date.now() };
    const opinions = [
      {
        agentId: "research",
        opinion: {
          opinion: "Proceed with command.",
          confidence: 0.8,
          rationale: "Requested action.",
        },
      },
      {
        agentId: "security",
        opinion: {
          opinion: "Forbidden command.",
          confidence: 0.85,
          rationale: "Root execution check.",
        },
      },
    ];

    const result = await arbitrator.arbitrate(context, opinions);

    expect(result.synthesizedDecision).toBe("Conflicting opinions received.");
    expect(result.conflictFlagged).toBe(true);
    expect(result.conflictDetails).toBe(
      "Research and Security have opposing stances.",
    );
  });
});
