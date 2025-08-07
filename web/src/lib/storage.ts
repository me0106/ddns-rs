const TOKEN_KEY = "auth_token";

export const token = {
  get: (): string | null => localStorage.getItem(TOKEN_KEY),
  set: (value: string): void => localStorage.setItem(TOKEN_KEY, value),
  remove: (): void => localStorage.removeItem(TOKEN_KEY),
  exists: (): boolean => localStorage.getItem(TOKEN_KEY) !== null,
};
