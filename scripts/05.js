// console.log("Hello world, I will register listener");

let a = 0;

addEventListener("fetch", (event) => {
  event.respondWith(
    new Response(`Hello world, I have been called ${++a} times`)
  );
});
