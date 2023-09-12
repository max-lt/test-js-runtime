// From lib.webworker.d.ts
// file:///opt/visual-studio-code/resources/app/extensions/node_modules/typescript/lib/lib.webworker.d.ts

type RequestInfo = Request | string;
// type RequestMode = "cors" | "navigate" | "no-cors" | "same-origin"; ??
type RequestRedirect = "error" | "follow" | "manual";

interface RequestInit {

  /** A BodyInit object or null to set request's body. */
  body?: BodyInit | null;

  /** A string indicating how the request will interact with the browser's cache to set request's cache. */
  // cache?: RequestCache;

  /** A string indicating whether credentials will be sent with the request always, never, or only when sent to a same-origin URL. Sets request's credentials. */
  // credentials?: RequestCredentials;

  /** A Headers object, an object literal, or an array of two-item arrays to set request's headers. */
  headers?: HeadersInit;

  /** A cryptographic hash of the resource to be fetched by request. Sets request's integrity. */
  // integrity?: string;

  /** A boolean to set request's keepalive. */
  // keepalive?: boolean;

  /** A string to set request's method. */
  method?: string;

  /** A string to indicate whether the request will use CORS, or will be restricted to same-origin URLs. Sets request's mode. */
  // mode?: RequestMode;

  /** A string indicating whether request follows redirects, results in an error upon encountering a redirect, or returns the redirect (in an opaque fashion). Sets request's redirect. */
  // redirect?: RequestRedirect;

  /** A string whose value is a same-origin URL, "about:client", or the empty string, to set request's referrer. */
  // referrer?: string;

  /** A referrer policy to set request's referrerPolicy. */
  // referrerPolicy?: ReferrerPolicy;

  /** An AbortSignal to set request's signal. */
  // signal?: AbortSignal | null;

  /** Can only be null. Used to disassociate request from any Window. */
  // window?: null;
}


/**
 * This Fetch API interface represents a resource request.
 *
 * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request)
 */
interface Request extends Body {
  /**
   * Returns the cache mode associated with request, which is a string indicating how the request will interact with the browser's cache when fetching.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request/cache)
   */
  // readonly cache: RequestCache;

  /**
   * Returns the credentials mode associated with request, which is a string indicating whether credentials will be sent with the request always, never, or only when sent to a same-origin URL.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request/credentials)
   */
  // readonly credentials: RequestCredentials;

  /**
   * Returns the kind of resource requested by request, e.g., "document" or "script".
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request/destination)
   */
  // readonly destination: RequestDestination;

  /**
   * Returns a Headers object consisting of the headers associated with request. Note that headers added in the network layer by the user agent will not be accounted for in this object, e.g., the "Host" header.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request/headers)
   */
  readonly headers: Headers;

  /**
   * Returns request's subresource integrity metadata, which is a cryptographic hash of the resource being fetched. Its value consists of multiple hashes separated by whitespace. [SRI]
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request/integrity)
   */
  // readonly integrity: string;

  /**
   * Returns a boolean indicating whether or not request can outlive the global in which it was created.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request/keepalive)
   */
  // readonly keepalive: boolean;

  /**
   * Returns request's HTTP method, which is "GET" by default.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request/method)
   */
  readonly method: string;

  /**
   * Returns the mode associated with request, which is a string indicating whether the request will use CORS, or will be restricted to same-origin URLs.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request/mode)
   */
  // readonly mode: RequestMode;

  /**
   * Returns the redirect mode associated with request, which is a string indicating how redirects for the request will be handled during fetching. A request will follow redirects by default.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request/redirect)
   */
  // readonly redirect: RequestRedirect;

  /**
   * Returns the referrer of request. Its value can be a same-origin URL if explicitly set in init, the empty string to indicate no referrer, and "about:client" when defaulting to the global's default. This is used during fetching to determine the value of the `Referer` header of the request being made.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request/referrer)
   */
  // readonly referrer: string;

  /**
   * Returns the referrer policy associated with request. This is used during fetching to compute the value of the request's referrer.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request/referrerPolicy)
   */
  // readonly referrerPolicy: ReferrerPolicy;

  /**
   * Returns the signal associated with request, which is an AbortSignal object indicating whether or not request has been aborted, and its abort event handler.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request/signal)
   */
  // readonly signal: AbortSignal;

  /**
   * Returns the URL of request as a string.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request/url)
   */
  readonly url: string;

  /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request/clone) */
  clone(): Request;
}

declare var Request: {
  prototype: Request;
  new(input: RequestInfo | URL, init?: RequestInit): Request;
};
