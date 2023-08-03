console.log("Hello, waiting for test event");

addEventListener("test", (event) => {
  console.log("Received test event", event);
});

addEventListener("hello", (event) => {
  console.log("Received hello event", event);
});
