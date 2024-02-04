import { Attachments } from "./attachments";
import { ContextAnnotations } from "./context_annotations";
import { EditControls } from "./edit_controls";
import { Entities } from "./entities";
import { Geo } from "./geo";
import { PublicMetrics } from "./public_metrics";
import { NonPublicMetrics } from "./non_public_metrics";
import { NoteTweet } from "./note_tweet";
import { OrganicMetrics } from "./organic_metrics";
import { PromotedMetrics } from "./promoted_metrics";
import { ReferencedTweets } from "./referenced_tweets";
import { Withheld } from "./withheld";

export interface Tweets {
  attachments: Attachments | undefined;
  authorId: string | undefined;
  contextAnnotations: ContextAnnotations[] | undefined;
  conversationId: string | undefined;
  createdAt: string | undefined;
  editControls: EditControls | undefined;
  editHistoryTweetIds: string[];
  entities: Entities | undefined;
  geo: Geo | undefined;
  id: string;
  inReplyToUserId: string | undefined;
  lang: string | undefined;
  possiblySensitive: boolean | undefined;
  publicMetrics: PublicMetrics | undefined;
  nonPublicMetrics: NonPublicMetrics | undefined;
  noteTweet: NoteTweet | undefined;
  organicMetrics: OrganicMetrics | undefined;
  promotedMetrics: PromotedMetrics | undefined;
  replySettings: string | undefined;
  referencedTweets: ReferencedTweets[] | undefined;
  text: string;
  withheld: Withheld | undefined;
  source: string | undefined;
}