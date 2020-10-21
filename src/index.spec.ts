import { Store } from "./common";

describe('basic storage tests', () => {
  let store: Store = new Store("test");

  beforeAll(() => {
    // todo initialize the store.
    // store = new Store()
  });

  it("should be able to write and get a value at a key", async () => {
    let key = generateUUID();
    let valueToInsert = generateUUID();
    {
      const value = await store.get(key);
      expect(value).toBeUndefined();
    }
    {
      await store.put(key, valueToInsert);
      const value = await store.get(key);
      expect(value).toEqual(valueToInsert);
    }
  });
});

/**
 * This is just a utility function to generate a UUID
 */
function generateUUID() { // Public Domain/MIT
  var d = new Date().getTime();//Timestamp
  var d2 = (performance && performance.now && (performance.now()*1000)) || 0;//Time in microseconds since page-load or 0 if unsupported
  return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function(c) {
      var r = Math.random() * 16;//random number between 0 and 16
      if(d > 0){//Use timestamp until depleted
          r = (d + r)%16 | 0;
          d = Math.floor(d/16);
      } else {//Use microseconds since page-load if supported
          r = (d2 + r)%16 | 0;
          d2 = Math.floor(d2/16);
      }
      return (c === 'x' ? r : (r & 0x3 | 0x8)).toString(16);
  });
}
