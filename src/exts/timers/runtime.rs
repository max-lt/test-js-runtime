use crate::base::JsRuntime;
use crate::base::JsStateRef;

use tokio::time::Instant;
use std::task::Context;
use std::task::Poll;
use v8::ContextScope;
use v8::HandleScope;

fn poll_timers(
  cx: &mut Context,
  scope: &mut v8::ContextScope<'_, v8::HandleScope<'_>>,
) -> Poll<()> {
  let state = scope.get_slot::<JsStateRef>().expect("No state found");
  let state = state.clone();
  let mut state_ref = state.borrow_mut();

  let timers = &mut state_ref.timers.timers;

  if timers.is_empty() {
      // No timers to process, so the future is considered complete.
      return Poll::Ready(());
  }

  // Find the timer with the earliest timestamp
  let (timer_id, timestamp) = match timers
      .iter_mut()
      .min_by(|(_, a), (_, b)| a.timestamp.cmp(&b.timestamp))
      .map(|(id, timer)| (*id, timer.timestamp))
  {
      Some(id) => id,
      None => return Poll::Ready(()),
  };

  println!("Timer {} is ready to be executed", timer_id);

  let now = Instant::now();
  if timestamp <= now {
      // Execute the timer's callback.
      {
          println!("Executing timer {}", timer_id);
          let timer = timers.get(&timer_id).unwrap();
          let undefined = v8::undefined(scope);
          let callback = v8::Local::new(scope, &timer.callback);
          drop(state_ref); // Explicitly drop the mutable borrow before calling the callback
          callback.call(scope, undefined.into(), &[]);
      }

      // Update the timer's timestamp.
      {
          let mut state_ref = state.borrow_mut(); // Re-acquire the mutable borrow after the callback
          let timers = &mut state_ref.timers.timers;
          match timers.get_mut(&timer_id) {
              Some(timer) => match timer.interval {
                  Some(duration) => {
                      // The timer is a repeating timer, so we update its timestamp.
                      timer.timestamp += duration;
                  }
                  None => {
                      // The timer is not a repeating timer, so we remove it from the state.
                      timers.remove(&timer_id);
                  }
              },
              None => {
                  // The timer was removed from the state by the callback.
                  println!("Timer {} was removed by the callback", timer_id)
              }
          }
      }

      // Notify the executor to poll again, as more timers may be ready.
      cx.waker().wake_by_ref();

      // Since we executed a timer, we return `Poll::Pending` to indicate that the future is not complete yet.
      Poll::Pending
  } else {
      // No timers are ready to be executed.

      // Calculate the time until the next timer is ready and register the waker to be woken at that time.
      let sleep_until = timestamp;
      let sleep_duration = sleep_until.saturating_duration_since(now);
      let waker = cx.waker().clone();
      tokio::spawn(async move {
          tokio::time::sleep(sleep_duration).await;
          waker.wake();
      });

      // Since no timer is ready to be executed, we return `Poll::Pending` to indicate that the future is not complete yet.
      Poll::Pending
  }
}

pub trait PollTimers {
    fn poll_timers(cx: &mut std::task::Context, scope: &mut ContextScope<HandleScope>) -> Poll<()>;
}

impl PollTimers for JsRuntime {
    fn poll_timers(cx: &mut std::task::Context, scope: &mut ContextScope<HandleScope>) -> Poll<()> {
        poll_timers(cx, scope)
    }
}
