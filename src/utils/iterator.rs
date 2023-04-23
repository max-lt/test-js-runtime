/**
 * FunctionCallbackArguments iterator
 */

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

/**
 * Array iterator
 */
/**
 * Array iterator
 */

pub trait ArrayIterExt {
    fn iter<'a>(&'a self, scope: &'a mut v8::HandleScope<'a>) -> ArrayIter<'a>;
}

impl ArrayIterExt for v8::Array {
    fn iter<'a>(&'a self, scope: &'a mut v8::HandleScope<'a>) -> ArrayIter<'a> {
        ArrayIter {
            scope,
            arr: self,
            index: 0,
        }
    }
}

pub struct ArrayIter<'a> {
    scope: &'a mut v8::HandleScope<'a>,
    arr: &'a v8::Array,
    index: u32,
}

impl<'a> Iterator for ArrayIter<'a> {
    type Item = v8::Local<'a, v8::Value>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.arr.length() {
            let value = self.arr.get_index(&mut self.scope, self.index);
            self.index += 1;
            value
        } else {
            None
        }
    }
}

/**
 * Object iterator
 */

 pub trait ObjectIterExt {
  fn iter<'a>(&'a self, scope: &'a mut v8::HandleScope<'a>) -> ObjectIter<'a>;
}

impl ObjectIterExt for v8::Object {
  fn iter<'a>(&'a self, scope: &'a mut v8::HandleScope<'a>) -> ObjectIter<'a> {
      let names = self.get_own_property_names(scope, v8::GetPropertyNamesArgs::default()).unwrap();
      ObjectIter {
          scope,
          obj: self,
          names,
          index: 0,
      }
  }
}

pub struct ObjectIter<'a> {
  scope: &'a mut v8::HandleScope<'a>,
  obj: &'a v8::Object,
  names: v8::Local<'a, v8::Array>,
  index: u32,
}

impl<'a> Iterator for ObjectIter<'a> {
  type Item = (v8::Local<'a, v8::Value>, v8::Local<'a, v8::Value>);

  fn next(&mut self) -> Option<Self::Item> {
      if self.index < self.names.length() {
          let key = self.names.get_index(&mut self.scope, self.index).unwrap();
          let value = self.obj.get(&mut self.scope, key).unwrap();
          self.index += 1;
          Some((key, value))
      } else {
          None
      }
  }
}
