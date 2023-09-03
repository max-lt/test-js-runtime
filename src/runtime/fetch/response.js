class Response {
  constructor(body = null, init = {}) {
    this.body = body;
    this.headers = new Headers(init.headers ?? {});
    this.status = init.status ?? 200;
    this.statusText = init.statusText;
  }

}
