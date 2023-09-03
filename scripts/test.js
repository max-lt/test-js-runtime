const str = "Hello world";

console.log(str);

const hello64 = btoa(str);

console.log("atob(str) =", hello64);

console.log("btoa(atob(str)) =", atob(hello64));

console.assert(str === atob(hello64));

console.log("addEventListener", typeof addEventListener);
console.log("removeEventListener", typeof removeEventListener);
console.log("dispatchEvent", typeof dispatchEvent);
console.log("Promise", Promise);
console.log("promise", new Promise(() => {}));

console.log("eval", eval("1 + 1"));

let count = 0;

addEventListener("fetch", (event) => {
  console.log("Handle fetch evt", event);

  event.respondWith(handleRequest(event.request));
});

async function handleRequest(request) {
  console.log("Handle request", request);

  if (request.url.endsWith("/favicon.ico")) {
    return new Response(null, { status: 404 });
  }

  return new Response(`Hello world, ive been called ${count++} times`, {
    headers: { "content-type": "text/plain" },
  });
}
