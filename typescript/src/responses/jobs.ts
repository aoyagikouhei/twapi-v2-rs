
export interface Jobs {
  id: string | undefined;
  createdAt: string | undefined;
  type: "tweets" | "users" | undefined;
  name: string | undefined;
  uploadUrl: string | undefined;
  uploadExpiresAt: string | undefined;
  downloadUrl: string | undefined;
  downloadExpiresAt: string | undefined;
  url: string | undefined;
  status: "created" | "in_progress" | "failed" | "complete" | undefined;
  error: string | undefined;
  resumable: boolean | undefined;
}