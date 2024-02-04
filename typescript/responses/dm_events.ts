import { Attachments } from "./attachments";
import { ReferencedTweets } from "./referenced_tweets";

export interface DmEvents {
  id: string | undefined;
  text: string | undefined;
  eventType: string | undefined;
  createdAt: string | undefined;
  senderId: string | undefined;
  dmConversationId: string | undefined;
  attachments: Attachments | undefined;
  referencedTweets: ReferencedTweets[] | undefined;
}