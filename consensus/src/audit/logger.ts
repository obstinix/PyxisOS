import * as fs from "fs";
import * as path from "path";

export class Logger {
  private logPath: string;

  constructor() {
    // Write audit logs to the consensus root folder
    this.logPath = path.join(__dirname, "../../audit.log");
  }

  /**
   * Log an event with structured metadata.
   */
  log(event: string, metadata: Record<string, any>): void {
    const logEntry = {
      timestamp: new Date().toISOString(),
      event,
      ...metadata,
    };

    const logLine = JSON.stringify(logEntry) + "\n";
    try {
      fs.appendFileSync(this.logPath, logLine, "utf8");
    } catch (err: any) {
      console.error(`Failed to write to audit log: ${err.message}`);
    }
  }

  /**
   * Helper to log agent analysis step.
   */
  logAgentCall(
    agentId: string,
    query: string,
    opinion: string,
    confidence: number,
  ): void {
    this.log("agent_call", {
      agentId,
      query,
      opinion,
      confidence,
    });
  }

  /**
   * Helper to log arbitration step.
   */
  logArbitration(
    query: string,
    decision: string,
    conflictFlagged: boolean,
    conflictDetails?: string,
  ): void {
    this.log("arbitration_decision", {
      query,
      decision,
      conflictFlagged,
      conflictDetails,
    });
  }
}
