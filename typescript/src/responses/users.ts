import { UserEntities } from "./user_entities";
import { Withheld } from "./withheld";

export interface Users {
  connectionStatus: string[] | undefined;
  createdAt: string | undefined;
  description: string | undefined;
  entities: UserEntities | undefined;
  id: string;
  location: string | undefined;
  mostRecentTweetId: string | undefined;
  name: string;
  pinnedTweetId: string | undefined;
  profileImageUrl: string | undefined;
  protected: boolean | undefined;
  publicMetrics: PublicMetrics | undefined;
  url: string | undefined;
  username: string;
  verified: boolean | undefined;
  verifiedType: string | undefined;
  withheld: Withheld | undefined;
}

export interface PublicMetrics {
  followersCount: number | undefined;
  followingCount: number | undefined;
  tweetCount: number | undefined;
  listedCount: number | undefined;
  likeCount: number | undefined;
}