console.log("Hello world, I will register listener");

let a = 0;

addEventListener("fetch", (event) => {
    console.log("Hello world, I received an event", event);
    return `Hello world, I received an event ${event}, i've been called ${++a} times`
});
