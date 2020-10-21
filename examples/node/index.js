const http = require('http');
const core = require('core');

const hostname = 'localhost';
const port = 3000;

async function test() {
  let store = new core.JSStore("test");

  let engineerBefore = await store.get("engineer");
  console.log("Engineer before: " + engineerBefore);

  let result = await store.put("engineer", "Konstantin");
  console.log("Result after inserting 'engineer' => 'Konstantin': " + result);
  
  let engineerAfter = await store.get("engineer");
  console.log("Engineer after: '" + engineerAfter + "'");  
};

const server = http.createServer((request, response) => {
  response.statusCode = 200;
  response.setHeader('Content-Type', 'text/plain');
  
  console.log(core.JSStore);
  test().then(() => {
    console.log("Test went smooth!");
    response.end('Test went smooth!');
  }).catch(error => {
    console.log("Testing failed: " + error);
    response.end('Testing failed: ' + error);
  });
});

server.listen(port, hostname, () => {
  console.log(`Server running at http://${hostname}:${port}/`);
});
