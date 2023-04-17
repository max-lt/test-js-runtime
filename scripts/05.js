console.log("Hello world, I will register listener");

addEventListener("fetch", event => {
    console.log("Hello world, I received an event", event);
    return "Hello world, I am a response";
});
