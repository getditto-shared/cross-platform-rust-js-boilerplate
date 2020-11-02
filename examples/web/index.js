import * as core from "core-web";

async function test() {
  window.core = core;

  let store = await new core.JSStore("test", core.JSStoreVariant.IndexedDB);
  console.log("Store after creating: " + store);

  let clearResult1 = await store.clear();
  console.log("Result of CLEAR operation right after opening: " + clearResult1);

  let engineerBeforePut = await store.get("engineer");
  console.log("Engineer before PUT: '" + engineerBeforePut + "'");

  let putResult = await store.put("engineer", "Konstantin");
  console.log("Result of PUT 'engineer' => 'Konstantin': " + putResult);

  let engineerAfterPut = await store.get("engineer");
  console.log("Engineer after PUT: '" + engineerAfterPut + "'");

  let clearResult2 = await store.clear();
  console.log("Result of CLEAR operation after PUT: " + clearResult2);

  let engineerAfterClear = await store.get("engineer");
  console.log("Engineer after CLEAR: '" + engineerAfterClear + "'");
};

console.log(core.JSStore);
test().then(() => {
  console.log("Test went smooth!");
}).catch(error => {
  console.log("Testing failed: " + error);
});
