import { MediaPublicMetrics } from "./media_public_metrics";
import { Variants } from "./variants";

export interface Media {
  durationMs: number | undefined;
  height: number | undefined;
  mediaKey: string | undefined;
  altText: string | undefined;
  previewImageUrl: string | undefined;
  publicMetrics: MediaPublicMetrics | undefined;
  type: string | undefined;
  variants: Variants[] | undefined;
  url: string | undefined;
  width: number | undefined;
}