import { Coordinates } from "./coordinates";

export interface Geo {
  type: string | undefined;
  bbox: number[] | undefined;
  containedWithin: string[] | undefined;
  coordinates: Coordinates | undefined;
  placeId: string | undefined;
  properties: any | undefined;
}