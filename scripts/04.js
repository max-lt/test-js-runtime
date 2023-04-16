console.log(typeof globalThis);
console.log(
  Object.keys(globalThis),
  Object.keys(globalThis).includes("console")
);
console.log("console", typeof globalThis.console);
console.log("atob", typeof globalThis.atob);
console.log("setInterval", typeof globalThis.setInterval);
console.log(
  "typeof atob",
  typeof atob,
  typeof atob !== "undefined" && atob("aGVsbG8=", 5)
);
console.log(atob("eyJhbGciOiJIUzI1NiJ9"));
console.log(atob("e30="));
console.log(atob("e30"));
console.log(btoa("bonjour"), atob(btoa("bonjour")));
console.log("queueMicrotask", typeof queueMicrotask);
