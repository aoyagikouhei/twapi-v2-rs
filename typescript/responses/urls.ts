import { Images } from "./images";

export interface Urls {
  displayUrl: string | undefined;
  description: string | undefined;
  end: number | undefined;
  expandedUrl: string | undefined;
  images: Images[] | undefined;
  mediaKey: string | undefined;
  start: number | undefined;
  status: number | undefined;
  title: string | undefined;
  url: string | undefined;
  unwoundUrl: string | undefined;
}