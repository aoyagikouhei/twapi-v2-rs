import { Options } from "./options";

export interface Polls {
  id: string;
  options: Options[];
  durationMinutes: number | undefined;
  endDatetime: string | undefined;
  votingStatus: string | undefined;
}