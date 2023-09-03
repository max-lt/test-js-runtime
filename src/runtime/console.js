globalThis.console = new Proxy(globalThis.console, {
  get(target, prop) {
    switch (prop) {
      case "log":
      case "info":
      case "warn":
      case "debug":
      case "error":
        return function (...args) {
          postMessage({ kind: "console", level: prop, args, date: Date.now() });
        };
      case "assert":
        return function (condition, ...args) {
          if (!condition) {
            throw new Error("Assertion failed: " + args.join(" "));
          }
        };
      default:
        return target[prop];
    }
  },
});
