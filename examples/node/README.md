# Example: Node

## Getting Started

*NOTE: work in progress, this is just to get a little demo going for now.*

1. Clone [getditto/cross-platform-rust-js-boilerplate](https://github.com/getditto/cross-platform-rust-js-boilerplate)

Build the core NPM module:

2. `cd` into repo
3. Run `npm install`
4. Run `npm run build-core-wasm-node`

Build the node example:

5. `cd` into `examples/node`
6. Run `npm install`
7. Run `node index.js`
9. Open [http://localhost:3000/](http://localhost:3000/)

Play with the JS version of the store in `index.js`, restart the server
(`node index.js`) and reload your browser. If you edit the core rust code,
you'll have to rerun `npm run build-core-wasm-node`, restart the server and
reload the browser.
