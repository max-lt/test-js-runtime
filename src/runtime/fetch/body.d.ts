
// From lib.dom.d.ts
// file:///opt/visual-studio-code/resources/app/extensions/node_modules/typescript/lib/lib.dom.d.ts

// type BodyInit = ReadableStream | XMLHttpRequestBodyInit; // original definition
type BodyInit = Uint8Array | Blob | BufferSource | /* FormData | URLSearchParams | */  string

// From lib.webworker.d.ts
// file:///opt/visual-studio-code/resources/app/extensions/node_modules/typescript/lib/lib.webworker.d.ts

interface Body {
  /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request/body) */
  // readonly body: ReadableStream<Uint8Array> | null;
  readonly body: Uint8Array | null;

  /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request/bodyUsed) */
  readonly bodyUsed: boolean;

  /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request/arrayBuffer) */
  arrayBuffer(): Promise<ArrayBuffer>;

  /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request/blob) */
  blob(): Promise<Blob>;

  /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request/formData) */
  // formData(): Promise<FormData>;

  /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request/json) */
  json(): Promise<any>;

  /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request/text) */
  text(): Promise<string>;
}
