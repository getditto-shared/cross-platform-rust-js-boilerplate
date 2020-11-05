import type { Store } from "./common";

const {
  NativeStore: InternalStore,
} = require(`./index.node`);

export class NativeStore implements Store {
  private inner: typeof InternalStore;

  constructor(name: string) {
    this.inner = new InternalStore(name);
  }

  get = (key: string): Promise<string> => this.inner.get(key);
  put = (key: string, value: string): Promise<void> => this.inner.put(key, value);
  clear = (): Promise<void> => this.inner.clear();
}

export class NativeDitto {
  static async open(name: string): Promise<Store> {
    return new NativeStore(name);
  }
}
