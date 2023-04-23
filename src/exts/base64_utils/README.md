[whatwg]: https://html.spec.whatwg.org/multipage/webappapis.html#atob
[whatwg_atob]: https://html.spec.whatwg.org/multipage/webappapis.html#dom-atob-dev
[whatwg_btoa]: https://html.spec.whatwg.org/multipage/webappapis.html#dom-btoa-dev
[mdn_atob]: https://developer.mozilla.org/en-US/docs/Web/API/atob
[mdn_btoa]: https://developer.mozilla.org/en-US/docs/Web/API/btoa
[mdn]: https://developer.mozilla.org/en-US/docs/Glossary/Base64

# Base64 utility methods (atob, btoa) extension

| Method | Status | Description                      | Specification                           |
| ------ | ------ | -------------------------------- | --------------------------------------- |
| atob   | done   | Decodes a base-64 encoded string | [WHATWG][whatwg_atob] / [MDN][mdn_atob] |
| btoa   | done   | Encodes a string in base-64      | [WHATWG][whatwg_btoa] / [MDN][mdn_btoa] |

Perfectly spec-compliant atob and btoa implementations as defined in the [WHATWG HTML Standard][whatwg].

Source code from https://github.com/jsdom/abab

## Properties added to the global scope

```js
atob; // The atob function
btoa; // The btoa function
```

## Usage

```js
const encoded = btoa("Hello, world!");

console.log(encoded); // SGVsbG8sIHdvcmxkIQ==

const decoded = atob(encoded);

console.log(decoded); // Hello, world!
```
