// From lib.webworker.d.ts
// file:///opt/visual-studio-code/resources/app/extensions/node_modules/typescript/lib/lib.webworker.d.ts

/**
 * The URL interface represents an object providing static methods used for creating object URLs.
 *
 * [MDN Reference](https://developer.mozilla.org/docs/Web/API/URL)
 */
interface URL {
  /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/URL/hash) */
  hash: string;
  /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/URL/host) */
  host: string;
  /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/URL/hostname) */
  hostname: string;
  /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/URL/href) */
  href: string;
  toString(): string;
  /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/URL/origin) */
  readonly origin: string;
  /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/URL/password) */
  password: string;
  /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/URL/pathname) */
  pathname: string;
  /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/URL/port) */
  port: string;
  /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/URL/protocol) */
  protocol: string;
  /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/URL/search) */
  search: string;
  /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/URL/searchParams) */
  readonly searchParams: URLSearchParams;
  /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/URL/username) */
  username: string;
  /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/URL/toJSON) */
  toJSON(): string;
}

declare var URL: {
  prototype: URL;
  new(url: string | URL, base?: string | URL): URL;
  /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/URL/createObjectURL) */
  createObjectURL(obj: Blob): string;
  /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/URL/revokeObjectURL) */
  revokeObjectURL(url: string): void;
};

/** [MDN Reference](https://developer.mozilla.org/docs/Web/API/URLSearchParams) */
interface URLSearchParams {
  /**
   * Appends a specified key/value pair as a new search parameter.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/URLSearchParams/append)
   */
  append(name: string, value: string): void;
  /**
   * Deletes the given search parameter, and its associated value, from the list of all search parameters.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/URLSearchParams/delete)
   */
  delete(name: string): void;
  /**
   * Returns the first value associated to the given search parameter.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/URLSearchParams/get)
   */
  get(name: string): string | null;
  /**
   * Returns all the values association with a given search parameter.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/URLSearchParams/getAll)
   */
  getAll(name: string): string[];
  /**
   * Returns a Boolean indicating if such a search parameter exists.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/URLSearchParams/has)
   */
  has(name: string): boolean;
  /**
   * Sets the value associated to a given search parameter to the given value. If there were several values, delete the others.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/URLSearchParams/set)
   */
  set(name: string, value: string): void;
  /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/URLSearchParams/sort) */
  sort(): void;
  /** Returns a string containing a query string suitable for use in a URL. Does not include the question mark. */
  toString(): string;
  forEach(callbackfn: (value: string, key: string, parent: URLSearchParams) => void, thisArg?: any): void;
}

declare var URLSearchParams: {
  prototype: URLSearchParams;
  new(init?: string[][] | Record<string, string> | string | URLSearchParams): URLSearchParams;
};
