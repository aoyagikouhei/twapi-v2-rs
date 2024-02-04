
export interface Jobs {
  id: string | undefined;
  createdAt: string | undefined;
  type: Type | undefined;
  name: string | undefined;
  uploadUrl: string | undefined;
  uploadExpiresAt: string | undefined;
  downloadUrl: string | undefined;
  downloadExpiresAt: string | undefined;
  url: string | undefined;
  status: Status | undefined;
  error: string | undefined;
  resumable: boolean | undefined;
}