import type { Store } from "./common";
import init, { JSStore } from "./web/core_web";
import wasm from "./web/core_web_bg.wasm";

export class WasmStore implements Store {
  constructor(private store: JSStore) {
  }

  async get(key: string): Promise<string | null | undefined> {
    return this.store.get(key);
  }

  async put(key: string, value: string): Promise<void> {
    return this.store.put(key, value);
  }

  async clear(): Promise<void> {
    return this.store.clear();
  }
}

export class WasmDitto {
  static async open(name: string): Promise<Store> {
    await loadWasm();
    const store = await new JSStore(name);
    return new WasmStore(store);
  }
}

let wasm_initialized = false;
const loadWasm = async () => {
  if (!wasm_initialized) {
    // @ts-ignore
    await init(wasm());
    wasm_initialized = true;
  }
};
