import type { Store } from "./common";
import { platform } from 'os';

const MODULE_NAME = "cross-platform-rust-js-boilerplate";
const { createNativeStoreClass } = require(`./${MODULE_NAME}.${platform()}.node`);
const InternalStore = createNativeStoreClass();

export class NativeStore implements Store {
        private inner: typeof InternalStore;
        constructor(name: string) {
            this.inner = new InternalStore(name);
        }
        
    get = (key: string): Promise<string> => this.inner.get(key);
    put = (key: string, value: string): Promise<void> => this.inner.put(key, value);
};
