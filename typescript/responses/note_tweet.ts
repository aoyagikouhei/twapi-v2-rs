import { Entities } from "./entities";

export interface NoteTweet {
  text: string | undefined;
  entities: Entities | undefined;
}