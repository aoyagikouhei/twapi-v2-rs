
export interface Lists {
  id: string;
  name: string;
  createdAt: string | undefined;
  private: boolean | undefined;
  followerCount: number | undefined;
  memberCount: number | undefined;
  ownerId: string | undefined;
  description: string | undefined;
}