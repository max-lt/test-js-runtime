// Extension trait for v8::FunctionCallbackArguments
pub trait FunctionCallbackArgumentsExt {
  fn iter(&self) -> FunctionCallbackArgumentsIter;
}

impl FunctionCallbackArgumentsExt for v8::FunctionCallbackArguments<'_> {
  fn iter(&self) -> FunctionCallbackArgumentsIter {
      FunctionCallbackArgumentsIter {
          args: self,
          index: 0,
      }
  }
}

// Iterator for FunctionCallbackArguments
pub struct FunctionCallbackArgumentsIter<'a> {
  args: &'a v8::FunctionCallbackArguments<'a>,
  index: i32,
}

impl<'a> Iterator for FunctionCallbackArgumentsIter<'a> {
  type Item = v8::Local<'a, v8::Value>;

  fn next(&mut self) -> Option<Self::Item> {
      if self.index < self.args.length() {
          let value = self.args.get(self.index);
          self.index += 1;
          Some(value)
      } else {
          None
      }
  }
}
