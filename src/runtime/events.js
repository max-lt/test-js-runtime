class Event {
  constructor(type) {
    this.type = type;
  }
}

const eventMap = new Map();

function addEventListener(eventType, eventTarget) {
  const eventHandlers = eventMap.get(eventType);
  if (eventHandlers) {
    eventHandlers.add(eventTarget);
  } else {
    eventMap.set(eventType, new Set([eventTarget]));
  }
}

function removeEventListener(eventType, eventTarget) {
  eventMap.delete(eventType);
}

function dispatchEvent(event) {
  const eventType = event.type;
  const eventHandlers = eventMap.get(eventType);
  if (eventHandlers) {
    for (const eventHandler of eventHandlers) {
      eventHandler.call(this, event);
    }
  }
}
