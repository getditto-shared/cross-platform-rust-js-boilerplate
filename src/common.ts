export interface Store {
  get(key: string): Promise<string | null | undefined>;
  put(key: string, value: string): Promise<void>;
  clear(): Promise<void>;
}
