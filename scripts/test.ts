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


addEventListener("fetch", (event: FetchEvent) => {
  event.respondWith(
    handleRequest(event.request).catch(
      (err: Error) => new Response(err.stack ?? err.message, { status: 500 })
    )
  );
});

async function handleRequest(request: Request): Promise<Response> {
  const url = request.url;

  if (url.endsWith("favicon.ico")) {
    return new Response(null, { status: 404 });
  }

  return new Response("Hello World!", { status: 200 });
}
