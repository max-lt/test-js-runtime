Promise.reject(new Error("Hello from async function error"))
  .then(() => {
    console.log("Hello from async function then");
  })
  .catch((err) => {
    console.log("Hello from async function catch", err);
  })
  .finally(() => {
    console.log("Hello from async function finally");
  });
