let __timerId = 0;
const __timers = new Map();

function __setTimer(callback, delay, args, interval = false) {
  const id = __timerId++;

  const time = Date.now() + delay;

  var timer = { id, callback, delay, args, time, interval };

  __timers.set(id, timer);

  // Get next timer
  __prepareNext();

  return id;
}

function __clearTimer(id) {
  // If timer does not exist, do nothing
  if (!__timers.delete(id)) {
    return;
  }

  if (!__timers.size) {
    postMessage({ kind: "timer", delay: null });
  } else {
    __prepareNext();
  }
}

function __prepareNext() {
  if (__timers.size) {
    const next = Array.from(__timers.values()).sort(
      (a, b) => a.time - b.time
    )[0];

    const delay = next.time - Date.now();

    postMessage({ kind: "timer", delay });
  }
}

function __wakeUp() {
  if (!__timers.size) {
    return;
  }

  const now = Date.now();

  // Call every pending timers
  Array.from(__timers.values())
    .filter((t) => t.time <= now)
    .map((timer) => {
      // If it's an interval, set it again
      if (timer.interval) {
        timer.time = now + timer.delay;
      } else {
        __timers.delete(timer.id);
      }

      timer.callback(...timer.args);
    });

  __prepareNext();
}

/*
 * Public API
 */

function setTimeout(callback, delay, ...args) {
  return __setTimer(callback, delay, args);
}

function clearTimeout(id) {
  __clearTimer(id);
}

function setInterval(callback, delay, ...args) {
  return __setTimer(callback, delay, args, true);
}

function clearInterval(id) {
  __clearTimer(id);
}
