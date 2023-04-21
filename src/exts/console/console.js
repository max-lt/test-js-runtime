globalThis.buildConsole = function buildConsole(logger) {
  return {
    log(...args) {
      logger?.call(this, "log", ...args);
    },
    info(...args) {
      logger?.call(this, "info", ...args);
    },
    warn(...args) {
      logger?.call(this, "warn", ...args);
    },
    debug(...args) {
      logger?.call(this, "debug", ...args);
    },
    error(...args) {
      logger?.call(this, "error", ...args);
    },
    assert(condition, ...data) {
      throw new Error("Method not implemented.");
    },
    clear() {
      throw new Error("Method not implemented.");
    },
    count(label) {
      throw new Error("Method not implemented.");
    },
    countReset(label) {
      throw new Error("Method not implemented.");
    },
    dir(item, options) {
      throw new Error("Method not implemented.");
    },
    dirxml(...data) {
      throw new Error("Method not implemented.");
    },
    group(...data) {
      throw new Error("Method not implemented.");
    },
    groupCollapsed(...data) {
      throw new Error("Method not implemented.");
    },
    groupEnd() {
      throw new Error("Method not implemented.");
    },
    table(tabularData, properties) {
      throw new Error("Method not implemented.");
    },
    time(label) {
      throw new Error("Method not implemented.");
    },
    timeEnd(label) {
      throw new Error("Method not implemented.");
    },
    timeLog(label, ...data) {
      throw new Error("Method not implemented.");
    },
    timeStamp(label) {
      throw new Error("Method not implemented.");
    },
    trace(...data) {
      throw new Error("Method not implemented.");
    },
  };
};
