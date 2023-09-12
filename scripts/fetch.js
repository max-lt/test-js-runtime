/// <reference lib="webworker" />

addEventListener("fetch", (event) => {
  console.log("Handle fetch evt", event);

  event.respondWith(
    handleRequest(event.request).catch((err) => {
      console.error("Error", err);

      return new Response(err.stack, { status: 500 });
    })
  );
});

async function handleRequest(request) {
  console.log("Handle request", request);

  if (request.url.endsWith("/favicon.ico")) {
    return new Response(null, { status: 404 });
  }

  // const myIp = await fetch("https://api.ipify.org?format=json")
  //   .then((res) => res.json())
  //   .then((res) => res.ip);

  const text = await fetch("https://registry.arq.pw/docker.svg").then((res) =>
    res.text()
  );

  return new Response(`Fetch:` + text, {
    headers: { "content-type": "text/plain" },
  });
}
