// From lib.webworker.d.ts
// file:///opt/visual-studio-code/resources/app/extensions/node_modules/typescript/lib/lib.webworker.d.ts

// type ResponseType = "basic" | "cors" | "default" | "error" | "opaque" | "opaqueredirect";

/**
 * This Fetch API interface represents the response to a request.
 *
 * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response)
 */
interface Response extends Body {
  /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/headers) */
  readonly headers: Headers;

  /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/ok) */
  readonly ok: boolean;

  /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/redirected) */
  readonly redirected: boolean;

  /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/status) */
  readonly status: number;

  /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/statusText) */
  readonly statusText: string;

  /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/type) */
  // readonly type: ResponseType;

  /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/url) */
  readonly url: string;

  /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/clone) */
  clone(): Response;
}

interface ResponseInit {
    headers?: HeadersInit;
    status?: number;
    statusText?: string;
}

declare var Response: {
  prototype: Response;
  new(body?: BodyInit | null, init?: ResponseInit): Response;
  /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/error) */
  error(): Response;
  /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/redirect) */
  redirect(url: string | URL, status?: number): Response;
};
