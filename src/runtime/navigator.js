Object.defineProperty(globalThis, "navigator", {
  get() {
    return {
      userAgent: "Node.js",
    };
  },
});
