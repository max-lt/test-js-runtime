// From lib.webworker.d.ts
// file:///opt/visual-studio-code/resources/app/extensions/node_modules/typescript/lib/lib.webworker.d.ts

type HeadersInit = [string, string][] | Record<string, string> | Headers;

// From dom.iterable.d.ts
// /opt/visual-studio-code/resources/app/extensions/node_modules/typescript/lib/lib.dom.iterable.d.ts
interface Headers {
  [Symbol.iterator](): IterableIterator<[string, string]>;
  /** Returns an iterator allowing to go through all key/value pairs contained in this object. */
  entries(): IterableIterator<[string, string]>;
  /** Returns an iterator allowing to go through all keys of the key/value pairs contained in this object. */
  keys(): IterableIterator<string>;
  /** Returns an iterator allowing to go through all values of the key/value pairs contained in this object. */
  values(): IterableIterator<string>;
}

/**
 * This Fetch API interface allows you to perform various actions on HTTP request and response headers. These actions include retrieving, setting, adding to, and removing. A Headers object has an associated header list, which is initially empty and consists of zero or more name and value pairs.  You can add to this using methods like append() (see Examples.) In all methods of this interface, header names are matched by case-insensitive byte sequence.
 *
 * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Headers)
 */
interface Headers {
  /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/Headers/append) */
  append(name: string, value: string): void;
  /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/Headers/delete) */
  delete(name: string): void;
  /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/Headers/get) */
  get(name: string): string | null;
  /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/Headers/has) */
  has(name: string): boolean;
  /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/Headers/set) */
  set(name: string, value: string): void;
  forEach(callbackfn: (value: string, key: string, parent: Headers) => void, thisArg?: any): void;
}

declare var Headers: {
  prototype: Headers;
  new(init?: HeadersInit): Headers;
};
