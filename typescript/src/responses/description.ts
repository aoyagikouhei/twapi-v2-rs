import { Urls } from "./urls";
import { Hashtags } from "./hashtags";
import { Mentions } from "./mentions";

export interface Description {
  urls: Urls[] | undefined;
  hashtags: Hashtags[] | undefined;
  mentions: Mentions[] | undefined;
}