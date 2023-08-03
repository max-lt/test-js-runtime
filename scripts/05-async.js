// console.log("Hello world, I will register listener");

let a = 0;

addEventListener("fetch", (event) => {
  event.respondWith(
    handleRequest(event.request)
      .then((response) => {
        console.log("Response:", response);

        return response;
      })
      .catch((err) => {
        console.warn("Error:", err);
        return new Response(err.stack, {
          status: 500,
        });
      })
  );
});

async function handleRequest(request) {
  // await new Promise((resolve) => setTimeout(resolve, 100)); // This is not working

  return new Response(`Hello world, I have been called ${++a} times`, {
    status: 200,
    statusText: "OK",
    headers: {
      "Content-Type": "text/plain; charset=utf-8",
    },
  });
}
