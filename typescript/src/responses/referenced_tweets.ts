
export interface ReferencedTweets {
  id: string | undefined;
  type: "retweeted" | "quoted" | "replied_to" | undefined;
}