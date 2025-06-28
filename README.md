# serde_json_wrapper

[![Crates.io](https://img.shields.io/crates/v/serde_json_wrapper)](https://crates.io/crates/serde_json_wrapper)
[![Downloads](https://img.shields.io/crates/d/serde_json_wrapper.svg)](https://crates.io/crates/serde_json_wrapper)
[![Documentation](https://docs.rs/serde_json_wrapper/badge.svg)](https://docs.rs/serde_json_wrapper)
[![License](https://img.shields.io/crates/l/serde_json_wrapper)](https://crates.io/crates/serde_json_wrapper)
[![Dependency Status](https://deps.rs/repo/github/JohnScience/serde_json_wrapper/status.svg)](https://deps.rs/repo/github/JohnScience/serde_json_wrapper)

This is a simple library that provides a `JsonPretty<T>` wrapper around any type `T` that implements [`serde::Serialize`] and/or [`serde::Deserialize`]. It allows you to serialize and deserialize `T` as a pretty JSON string (see [`serde_json::to_string_pretty`]).

One notable use case is to use it for rendering the JSON representation of `T` with [`handlebars::Handlebars::render`].

# Example

```rust
use serde::{Deserialize, Serialize};
use serde_json_wrapper::JsonPretty;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct TestStruct {
    field1: String,
    field2: i32,
}

fn main() {
    let test_data = TestStruct {
        field1: "value1".to_string(),
        field2: 42,
    };
    let json_pretty = JsonPretty(test_data);

    let json_value = serde_json::to_value(&json_pretty).unwrap();
    let serde_json::Value::String(serialized) = json_value else {
        panic!("Expected a JSON string");
    };
    let expected = "{\n  \"field1\": \"value1\",\n  \"field2\": 42\n}";
    assert_eq!(serialized, expected);
}
```

[`JsonPretty<T>`]: https://docs.rs/serde_json_wrapper/latest/serde_json_wrapper/struct.JsonPretty.html
[`serde::Serialize`]: https://docs.rs/serde/latest/serde/trait.Serialize.html
[`serde::Deserialize`]: https://docs.rs/serde/latest/serde/trait.Deserialize.html
[`serde_json::to_string_pretty`]: https://docs.rs/serde_json/latest/serde_json/fn.to_string_pretty.html
[`handlebars::Handlebars::render`]: https://docs.rs/handlebars/latest/handlebars/struct.Handlebars.html#method.render
