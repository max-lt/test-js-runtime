// console.log("Hello world, I will register listener");

let a = 0;

addEventListener("fetch", (event) => {
  event.respondWith(
    handleRequest(event.request)
  );
});

function handleRequest(request) {
  return new Response(`Hello world, I have been called ${++a} times`, {
    status: 200,
    statusText: "OK",
    headers: {
      "Content-Type": "text/plain; charset=utf-8",
    },
  });
}
