const { DittoStore, Store } = require("..");

describe("basic storage tests", () => {
  let store: typeof Store = undefined;

  beforeAll(() => {
    // todo initialize the store.
    // store = new Store()
  });

  it("should be a different store impl depending on the backend", async () => {
    if (isNode()) {
      expect(DittoStore.name).toEqual("NativeDitto");
    } else {
      expect(DittoStore.name).toEqual("WasmDitto");
    }
  });

  it("should be able to write and get a value at a key", async () => {
    // a poor man's it.skip as karmatic doesn't support the entire jest global API
    return;

    let key = generateUUID();
    let valToInsert = generateUUID();
    {
      const val = await store.get(key);
      expect(val).toBeUndefined();
    }
    {
      await store.put(key, valToInsert);
      const val = await store.get(key);
      expect(val).toEqual(valToInsert);
    }
  });
});

/**
 * Detects if the process is a browser or nodejs
 * ref: https://github.com/iliakan/detect-node
 * MIT Licensed
 */
function isNode() {
  return (
    Object.prototype.toString.call(
      typeof process !== "undefined" ? process : 0
    ) === "[object process]"
  );
}

/**
 * This is just a utility function to generate a UUID
 */
function generateUUID() {
  // Public Domain/MIT
  var d = new Date().getTime(); //Timestamp
  var d2 = (performance && performance.now && performance.now() * 1000) || 0; //Time in microseconds since page-load or 0 if unsupported
  return "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx".replace(/[xy]/g, function (c) {
    var r = Math.random() * 16; //random number between 0 and 16
    if (d > 0) {
      //Use timestamp until depleted
      r = (d + r) % 16 | 0;
      d = Math.floor(d / 16);
    } else {
      //Use microseconds since page-load if supported
      r = (d2 + r) % 16 | 0;
      d2 = Math.floor(d2 / 16);
    }
    return (c === "x" ? r : (r & 0x3) | 0x8).toString(16);
  });
}
