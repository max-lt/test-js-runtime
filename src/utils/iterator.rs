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

// pub struct ArrayIter<'a> {
//   scope: &'a mut v8::HandleScope<'a>,
//   arr: &'a v8::Array,
//   key:  v8::Local<'a, v8::Value>
// }

// impl<'a> Iterator for ArrayIter<'a> {
//   type Item = v8::Local<'a, v8::Value>;

//   fn next(&mut self) -> Option<Self::Item> {
//       if self.key < self.arr.length() {
//           let value = self.arr.get(&mut self.scope, self.key);
//           self.key += 1;
//           value
//       } else {
//           None
//       }
//   }
// }
