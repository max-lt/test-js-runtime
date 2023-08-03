let a = 0;

function wait() {
  return new Promise((resolve) => queueMicrotask(resolve))
}

addEventListener("fetch", (event) => {
  return event.respondWith(
    handleRequest(event.request)
      .then((response) => {
        console.log("Response:", response);

        return response;
      })
      .catch((err) => {
        console.error("Error:", err);
        return new Response(err.stack, {
          status: 500,
        });
      })
  );
});

async function handleRequest(request) {
  console.log("Preparing response...");

  // await wait(30);

  console.log("Done waiting, returning response");

  return new Response(`Hello world, I have been called ${++a} times`, {
    status: 200,
    statusText: "OK",
    headers: {
      "Content-Type": "text/plain; charset=utf-8",
    },
  });
}
