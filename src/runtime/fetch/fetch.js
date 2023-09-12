/**
 *
 * @param Sting url
 * @param { method, body, headers } options
 * @returns
 */
function fetch(url, options = {}) {
  return new Promise((resolve, reject) => {
    postMessage({
      kind: "fetch",
      request: {
        url,
        options: {
          ...options, // method body headers
          headers: Array.from(options.headers ?? []),
        },
      },
      sendResponse: (response) => {
        resolve(new Response(response.body, response.options));
      },
    });
  });
}
