import { UserUrl } from "./user_url";
import { Description } from "./description";

export interface UserEntities {
  url: UserUrl | undefined;
  description: Description | undefined;
}