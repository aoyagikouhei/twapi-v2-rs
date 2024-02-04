import { Media } from "./media";
import { Places } from "./places";
import { Polls } from "./polls";
import { Tweets } from "./tweets";
import { Users } from "./users";

export interface Includes {
  media: Media[] | undefined;
  places: Places[] | undefined;
  polls: Polls[] | undefined;
  tweets: Tweets[] | undefined;
  users: Users[] | undefined;
}