import { Topics } from "./topics";

export interface Spaces {
  id: string;
  hostIds: string[] | undefined;
  createdAt: string | undefined;
  creatorId: string | undefined;
  endedAt: string | undefined;
  lang: string | undefined;
  isTicketed: boolean | undefined;
  invitedUserIds: string[] | undefined;
  participantCount: number | undefined;
  scheduledStart: string | undefined;
  speakerIds: string[] | undefined;
  startedAt: string | undefined;
  state: State;
  subscriberCount: number | undefined;
  topicIds: string[] | undefined;
  topics: Topics[] | undefined;
  title: string | undefined;
  updatedAt: string | undefined;
}