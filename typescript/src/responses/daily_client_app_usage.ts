import { Usage } from "./usage";

export interface DailyClientAppUsage {
  clientAppId: string | undefined;
  usageResultCount: number | undefined;
  usage: Usage[] | undefined;
}