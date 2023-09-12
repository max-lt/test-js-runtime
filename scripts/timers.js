console.log("Hello from timers.js");

const start = Date.now();

function wait(ms, ...args) {
  return new Promise((resolve) => setTimeout(resolve, ms, ...args));
}

setTimeout(() => {
  console.log("Hello from setTimeout");

  setTimeout(console.log, 100, "Hello from setTimeout nested args", 1, 2, 3);
}, 200);

let i = 0;

const id = setInterval(() => {
  console.log("Hello from setInterval", i, Date.now() - start, "ms");

  if (++i >= 3) {
    clearInterval(id);
  }
}, 50);

(async () => {
  console.log("Hello from async setTimeout");
  await wait(800);
  console.log("Hello from async setTimeout after ", Date.now() - start, "ms");
})();

console.log("Bye from timers.js");
