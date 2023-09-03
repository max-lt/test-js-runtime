class FetchEvent extends Event {
  #request;
  #requestId;
  #startTime;
  #respondWith;
  #responded;

  get request() {
    return this.#request;
  }

  get requestId() {
    return this.#requestId;
  }

  get startTime() {
    return this.#startTime;
  }

  constructor(request, respondWith) {
    super("fetch");
    this.#request = request;
    this.#requestId = request.headers?.get("x-request-id");
    this.#startTime = Date.now();
    this.#respondWith = respondWith;
    this.#responded = false;
  }

  respondWith(response) {
    if (this.#responded === true) {
      throw new TypeError("Already responded to this FetchEvent.");
    }

    this.#responded = true;
    this.#respondWith(response).catch((err) => console.warn(err));
  }

  [Symbol.toStringTag]() {
    return "FetchEvent";
  }
}
