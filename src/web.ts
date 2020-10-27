import type { Store } from "./common";
import init, { JSStore } from "./web/core_web";
import wasm from "./web/core_web_bg.wasm";

export class WasmStore implements Store {
  private store: Store;
  constructor(name: string) {
    this.store = new JSStore(name);
  }

  get(key: string): Promise<string | null | undefined> {
    return this.store.get(key);
  }
  put(key: string, value: string): Promise<void> {
    return this.store.put(key, value);
  }
}

export class WasmDitto {
  static async load(name: string): Promise<Store> {
    await loadWasm();
    return new WasmStore(name);
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
