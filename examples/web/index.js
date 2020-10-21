import * as core from "core";

async function test() {
  let store = new core.JSStore("test");

  let engineerBefore = await store.get("engineer");
  console.log("Engineer before: " + engineerBefore);

  let result = await store.put("engineer", "Konstantin");
  console.log("Result after inserting 'engineer' => 'Konstantin': " + result);
  
  let engineerAfter = await store.get("engineer");
  console.log("Engineer after: '" + engineerAfter + "'");  
};

console.log(core.JSStore);
test().then(() => {
  console.log("Test went smooth!");
}).catch(error => {
  console.log("Testing failed: " + error);
});