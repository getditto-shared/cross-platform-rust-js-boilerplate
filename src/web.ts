import type { Store } from "./common";
import init, { JSStore } from "./web/core_web";
import wasm from "./web/core_web_bg.wasm";

export class WasmStore implements Store {
    private store: Store;
    constructor(private name: string) {
        this.store = new JSStore(name);
    }

    get(key: string): Promise<string | null | undefined> {
        return this.store.get(key)
    }
    put(key: string, value: string): Promise<void> {
        return this.store.put(key, value);
    }
}

export const loadWasm = async () => {
    // @ts-ignore
    await init(wasm());
  };
  
export const openWasm = async (): Promise<{ Store: WasmStore }> => {
    await loadWasm();
    return { Store: WasmStore };
  };
  