import { Usage } from "./usage";

export interface DailyProjectUsage {
  projectId: string | undefined;
  usage: Usage[] | undefined;
}