/// <reference no-default-lib="true" />
/// <reference lib="esnext" />

globalThis.Console = class Console implements Console {
  #logger: (level: string, ...data: any[]) => void;

  constructor(logger) {
    this.#logger = logger;
  }
  log(...args) {
    this.#logger?.call(this, "log", ...args);
  }
  info(...args) {
    this.#logger?.call(this, "info", ...args);
  }
  warn(...args) {
    this.#logger?.call(this, "warn", ...args);
  }
  debug(...args) {
    this.#logger?.call(this, "debug", ...args);
  }
  error(...args) {
    this.#logger?.call(this, "error", ...args);
  }
  assert(condition?: boolean | undefined, ...data: any[]): void {
    throw new Error("Method not implemented.");
  }
  clear(): void {
    throw new Error("Method not implemented.");
  }
  count(label?: string | undefined): void {
    throw new Error("Method not implemented.");
  }
  countReset(label?: string | undefined): void {
    throw new Error("Method not implemented.");
  }
  dir(item?: any, options?: any): void {
    throw new Error("Method not implemented.");
  }
  dirxml(...data: any[]): void {
    throw new Error("Method not implemented.");
  }
  group(...data: any[]): void {
    throw new Error("Method not implemented.");
  }
  groupCollapsed(...data: any[]): void {
    throw new Error("Method not implemented.");
  }
  groupEnd(): void {
    throw new Error("Method not implemented.");
  }
  table(tabularData?: any, properties?: string[] | undefined): void {
    throw new Error("Method not implemented.");
  }
  time(label?: string | undefined): void {
    throw new Error("Method not implemented.");
  }
  timeEnd(label?: string | undefined): void {
    throw new Error("Method not implemented.");
  }
  timeLog(label?: string | undefined, ...data: any[]): void {
    throw new Error("Method not implemented.");
  }
  timeStamp(label?: string | undefined): void {
    throw new Error("Method not implemented.");
  }
  trace(...data: any[]): void {
    throw new Error("Method not implemented.");
  }
}
