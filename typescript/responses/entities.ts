import { Annotations } from "./annotations";
import { Cashtags } from "./cashtags";
import { Hashtags } from "./hashtags";
import { Mentions } from "./mentions";
import { Urls } from "./urls";

export interface Entities {
  annotations: Annotations[] | undefined;
  cashtags: Cashtags[] | undefined;
  hashtags: Hashtags[] | undefined;
  mentions: Mentions[] | undefined;
  urls: Urls[] | undefined;
}