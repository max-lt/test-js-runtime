[whatwg_timers]: https://html.spec.whatwg.org/multipage/timers-and-user-prompts.html#timers
[whatwg_microtask_queuing]: https://html.spec.whatwg.org/multipage/timers-and-user-prompts.html#microtask-queuing

# Timers

Implements the [timers][whatwg_timers] and [microtask queuing][whatwg_microtask_queuing] functions.

https://html.spec.whatwg.org/multipage/webappapis.html#event-loops

## Properties added to the global scope

```js
setTimeout; // The setTimeout function
clearTimeout; // The clearTimeout function
setInterval; // The setInterval function
clearInterval; // The clearInterval function
queueMicrotask; // The queueMicrotask function
```

## Usage

```js
const timeout = setTimeout(() => {
  console.log("Hello, world!");
}, 1000);

clearTimeout(timeout);

const interval = setInterval(() => {
  console.log("Hello, world!");
}, 1000);

clearInterval(interval);

queueMicrotask(() => {
  console.log("Hello, world!");
});
```

## Limitations

- setTimeout and setInterval takes only one or two arguments, the callback and the delay.
  Not setting the delay will default to 0.

## To do

- Implement the `setImmediate` and `clearImmediate` functions
- Allow setTimeout and setInterval to take more than two arguments and pass them to the callback function

## References

https://html.spec.whatwg.org/multipage/timers-and-user-prompts.html#microtask-queuing
https://html.spec.whatwg.org/multipage/timers-and-user-prompts.html#timers
