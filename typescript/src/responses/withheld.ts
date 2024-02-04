
export interface Withheld {
  copyright: boolean | undefined;
  countryCodes: string[] | undefined;
  scope: "tweet" | "user" | undefined;
}