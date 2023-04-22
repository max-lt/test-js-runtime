// console.log("Hello world, I will register listener");

let a = 0;

addEventListener("fetch", (event) => {
  // console.log("Hello world, I received an event", event);
  event.respondWith("Hello world, I am a response" + a++);
});
