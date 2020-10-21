# Cross Platform JavaScript (TypeScript) Repo for Rust

This library shows you how to take a Rust library, compile it for WASM and as a Native Binary and use it as an NPM package with the same interface. In addition, it'll run the same Jest integration tests on all runtimes.

This simple library is a key value store that will use a different embedded key value store depending on the platform:

* If compiling for WASM + Web Browsers, it'll use [IndexedDB](https://developer.mozilla.org/en-US/docs/Web/API/IndexedDB_API)
* If compiling for Native it'll use [sled](https://crates.io/crates/sled)

## Preparing

1. Install dependencies with `yarn` or `npm install`
2. To build rust run `npm run build-core`. This builds the core library in [./core](./core)
3. To build the typescript common code run `npm run build-typescript`
4. To build everything run `npm run build`
4. Run tests with `npm test`. This runs the tests in `index.spec.ts`
