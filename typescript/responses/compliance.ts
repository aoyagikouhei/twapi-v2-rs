import { ComplianceTweet } from "./compliance_tweet";
import { ComplianceUser } from "./compliance_user";

export interface Compliance {
  tweet: ComplianceTweet | undefined;
  user: ComplianceUser | undefined;
  eventAt: string | undefined;
}