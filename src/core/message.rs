use v8::HandleScope;
use v8::Local;
use v8::Value;

use crate::utils;

// Message sent from the runtime to the worker
pub trait RuntimeMessage {
    fn kind(&self) -> String;

    fn prepare<'s>(&mut self, scope: &mut HandleScope<'s, ()>);

    // value
    fn to_value<'s>(&self, scope: &mut HandleScope<'s>) -> Local<'s, Value>;
}

#[derive(Debug)]
pub struct RuntimeBasicMessage {
    kind: String,
    data: Option<v8::Global<v8::Value>>,
}

impl RuntimeBasicMessage {
    pub fn new(kind: String) -> RuntimeBasicMessage {
        RuntimeBasicMessage { kind, data: None }
    }

    pub fn new_with_data(kind: String, data: v8::Global<v8::Value>) -> RuntimeBasicMessage {
        RuntimeBasicMessage {
            kind,
            data: Some(data),
        }
    }

    pub fn set_data(&mut self, data: v8::Global<v8::Value>) {
        self.data = Some(data);
    }

    pub fn clear_data(&mut self) {
        self.data = None;
    }
}

impl RuntimeMessage for RuntimeBasicMessage {
    fn kind(&self) -> String {
        self.kind.clone()
    }

    fn prepare<'s>(&mut self, _scope: &mut HandleScope<'s, ()>) {}

    fn to_value<'s>(&self, scope: &mut HandleScope<'s>) -> Local<'s, Value> {
        let event = v8::Object::new(scope);

        utils::assign_string(scope, event, "kind", self.kind());

        if let Some(data) = &self.data {
            let data = v8::Local::new(scope, data);
            utils::assign(scope, event, "data", data);
        }

        event.into()
    }
}
