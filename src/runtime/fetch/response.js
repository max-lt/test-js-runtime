class Response {
  constructor(body = null, options = {}) {
    this.body = body;
    this.headers = new Headers(options.headers ?? {});
    this.status = options.status ?? 200;
    this.statusText = options.statusText;
  }

  json() {
    return JSON.parse(this.body);
  }

  text() {
    return this.body;
  }
}
