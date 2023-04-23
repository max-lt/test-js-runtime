# Fetch extension

This extension provides a fetch API for the application.

## Properties added to the global scope

```js
fetch // The fetch function (not implemented)
addEventListener // The addEventListener function

Headers // The Headers class (not implemented)
Request // The Request class (partially implemented)
Response // The Response class (partially implemented)
FetchEvent // The FetchEvent class (not implemented)
```

## Usage

```js
addEventListener('fetch', (event) => {
  event.respondWith(new Response('Hello, world!'));
});
```

## Limitations

- The `fetch` function is not implemented.
- The `Headers` class is not implemented.
- The `Request` class is only partially implemented.
- The `Response` class is only partially implemented.
- The `FetchEvent` class is not implemented.
- event.respondWith can only handle a Response instance (not a Promise\<Response\>)
