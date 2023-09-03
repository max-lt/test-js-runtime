// Will be overwritten on instantiation
function postMessage() {};
function onMessage() {};

Object.defineProperty(globalThis, "self", { value: this });
