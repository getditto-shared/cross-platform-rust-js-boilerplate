# Example: Web

## Getting Started

*NOTE: work in progress, this is just to get the playground going for now.*

1. Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
2. Clone [getditto/cross-platform-rust-js-boilerplate](https://github.com/getditto/cross-platform-rust-js-boilerplate)
3. `cd` into repo -> `examples/web`
4. Run `npm run build` (`npm install` is run as part of this due to dependency cycle with local pkg)
5. Run `npm run start`
6. Open [http://localhost:8080/](http://localhost:8080/)

Play with the JS version of the store in `index.js`, the browser should
reload automatically. If you edit the core rust code, you'll have to run
`npm run build-core`, the browser should reload automatically.
