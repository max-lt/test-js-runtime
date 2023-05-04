function delay(ms) {
  // return Promise.resolve(42);
  // return new Promise((resolve) => setTimeout(resolve, ms));
  return new Promise((resolve) => {
    queueMicrotask(() => resolve(42));
  });
}

async function start() {
  console.log("Async function start");

  const x = await delay(1000);

  console.log("Async function end", x);
}

console.log("Start async function");

start()
  .then(() => console.log("Async function then"))
  .catch((err) => console.error("Async function catch", err))
  .finally(() => console.log("Async function finally"));

console.log("End async function");

const timeout = setTimeout(() => {}, 2000);

addEventListener("test", async () => {
  console.log("Test event");
  clearTimeout(timeout);
});

addEventListener("01", () => {});
addEventListener("02", () => {});
addEventListener("03", () => {});
addEventListener("04", () => {});
addEventListener("05", () => {});
addEventListener("06", () => {});
addEventListener("07", () => {});
addEventListener("08", () => {});

async function handleRequest(request) {
  return new Response('ok');
}
