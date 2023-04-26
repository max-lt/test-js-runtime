queueMicrotask(() => console.log("Microtask 0"));

// Task 1
console.log("Start script");

// Task 2
queueMicrotask(() => console.log("Microtask 1"), 78, 564, 12);
queueMicrotask(() => console.log("Microtask 2"));
queueMicrotask(() => console.log("Microtask 3"));
queueMicrotask(854);

// Task 3
console.log("End script");

queueMicrotask(() => console.log("Microtask 4"));
