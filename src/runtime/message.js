onMessage((message) => {
  console.log("Got controller message", JSON.stringify(message));

  switch (message.kind) {
    // Runtime fetch message
    case "fetch":
      const request = new Request(
        message.request.url,
        message.request.options ?? {}
      );

      console.log("Got request", request);
      dispatchEvent(
        new FetchEvent(request, async (response) => {
          console.log("Got response", response);

          const res = await Promise.resolve(response).catch((err) => {
            // User did not handled error
            postMessage({
              type: "error",
              error: { message: err.message, stack: err.stack },
            });

            return new Response(err.stack, { status: 500 });
          });

          message.sendResponse({
            body: res.body, // await res.arrayBuffer(),
            headers: Object.fromEntries(res.headers?.entries()),
            status: res.status,
            statusText: res.statusText,
          });
        })
      );

      break;
      case "timer":
        __wakeUp();
        break;
    default:
      console.warn(`Unknown message kind: "${message.kind}"`);
  }
});
