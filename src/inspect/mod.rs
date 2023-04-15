use std::fmt::Write;
use v8::{Local, HandleScope};

pub fn inspect_v8_value(value: Local<v8::Value>, scope: &mut HandleScope) -> String {
  let mut output = String::new();

  if value.is_string() {
      write!(&mut output, "\"{}\"", value.to_rust_string_lossy(scope))
          .expect("Error writing to output string");
      return output;
  }

  if value.is_function() {
      write!(&mut output, "[Function]").expect("Error writing to output string");
      return output;
  }

  // If value is not an object, return it as a string
  if !value.is_object() && !value.is_array() {
      return value.to_rust_string_lossy(scope);
  }

  let object = value.to_object(scope).unwrap();
  let keys = object
      .get_own_property_names(scope, v8::GetPropertyNamesArgs::default())
      .unwrap();

  let mut output = String::new();

  write!(&mut output, "{}", if value.is_array() { "[" } else { "{" })
      .expect("Error writing to output string");

  for i in 0..keys.length() {
      let key = keys.get_index(scope, i).unwrap();
      let key_str = keys
          .get_index(scope, i)
          .unwrap()
          .to_rust_string_lossy(scope);
      let val = object.get(scope, key).unwrap();
      let val_str = inspect_v8_value(val, scope);

      if value.is_array() {
          write!(&mut output, "{}{}", if i > 0 { ", " } else { "" }, val_str)
              .expect("Error writing to output string");
      } else {
          write!(
              &mut output,
              "{}{}: {}",
              if i > 0 { ", " } else { "" },
              key_str,
              val_str
          )
          .expect("Error writing to output string");
      }
  }

  write!(&mut output, "{}", if value.is_array() { "]" } else { "}" })
      .expect("Error writing to output string");

  output
}
