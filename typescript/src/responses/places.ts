import { Geo } from "./geo";

export interface Places {
  fullName: string;
  id: string;
  containedWithin: string[] | undefined;
  country: string | undefined;
  countryCode: string | undefined;
  geo: Geo | undefined;
  name: string | undefined;
  placeType: string | undefined;
}