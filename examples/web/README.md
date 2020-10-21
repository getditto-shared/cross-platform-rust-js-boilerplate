# Example: Web

## Getting Started

*NOTE: work in progress, this is just to get the playground going for now.*

1. Clone [getditto/cross-platform-rust-js-boilerplate](https://github.com/getditto/cross-platform-rust-js-boilerplate)

Build the core NPM module:

2. `cd` into repo
3. Run `npm install`
4. Run `npm run build-core-wasm-bundler`

Build the web example:

5. `cd` into `examples/web`
6. Run `npm install`
7. Run `npm run build`
8. Run `npm run start`
9. Open [http://localhost:8080/](http://localhost:8080/)

Play with the JS version of the store in `index.js`, the browser should
reload automatically. If you edit the core rust code, you'll have to rerun
`npm run build-core-wasm` (in root of repo) and `npm run build`
(in `examples/web`) and reload your browser.
