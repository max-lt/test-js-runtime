use crate::base::JsRuntime;
use crate::base::JsStateRef;

use v8::ContextScope;
use v8::HandleScope;
use v8::Local;

use super::fetch_event::JsFetchEvent;
use crate::exts::event::trigger_event;

pub trait Fetch {
    fn fetch(&mut self, req: actix_web::HttpRequest) -> Option<JsFetchEvent>;
    fn has_fetch_handler(&mut self) -> bool;
}

impl Fetch for JsRuntime {
    /// Call fetch event handler
    fn fetch(&mut self, req: actix_web::HttpRequest) -> Option<JsFetchEvent> {
        let scope = &mut HandleScope::new(&mut self.isolate);
        let context = Local::new(scope, &self.context);
        let scope = &mut ContextScope::new(scope, context);

        let event = super::fetch_event::create_fetch_event(scope, req);

        let result = match trigger_event("fetch", scope, Some(event.event.into())) {
            Some(result) => result,
            None => return None,
        };

        println!(
            "fetch call result: {:?}",
            crate::utils::inspect::inspect_v8_value(result, scope)
        );

        Some(event)
    }

    fn has_fetch_handler(&mut self) -> bool {
        let scope = &mut HandleScope::new(&mut self.isolate);

        let context = Local::new(scope, &self.context);
        let scope = &mut ContextScope::new(scope, context);

        // Check if script registered event listeners
        let state = scope.get_slot::<JsStateRef>().expect("No state found");
        let state = state.borrow();

        state.handlers.get("fetch").is_some()
    }
}
