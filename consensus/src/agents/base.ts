import { AgentContext, AgentOpinion } from '../types';

export interface Agent {
  id: string;
  name: string;
  description: string;
  analyze(context: AgentContext): Promise<AgentOpinion>;
}
