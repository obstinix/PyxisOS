import { Agent } from "../agents/base";
import { AgentContext } from "../types";

export class Router {
  constructor(private agents: Agent[]) {}

  /**
   * Routes the given query context to the appropriate subset of agents.
   * MVP: Always returns all registered agents.
   */
  async route(_context: AgentContext): Promise<Agent[]> {
    return this.agents;
  }
}
