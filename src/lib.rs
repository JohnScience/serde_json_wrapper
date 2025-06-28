#[doc = include_str!("../README.md")]

use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// A wrapper for serializing a value as a pretty JSON string.
/// 
/// One use case for this is to render a value as a pretty JSON string
/// using [`handlebars::Handlebars::render`].
/// 
/// [`handlebars::Handlebars::render`]: https://docs.rs/handlebars/latest/handlebars/struct.Handlebars.html#method.render
pub struct JsonPretty<T>(pub T);

impl<T: Serialize> Serialize for JsonPretty<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let json_string = serde_json::to_string_pretty(&self.0)
            .map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(&json_string)
    }
}

struct JsonPrettyVisitor<T> {
    _marker: std::marker::PhantomData<T>,
}

impl<'de, T> serde::de::Visitor<'de> for JsonPrettyVisitor<T>
where
    T: serde::de::DeserializeOwned,
{
    type Value = JsonPretty<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a pretty JSON string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        let value: T = serde_json::from_str(v).map_err(serde::de::Error::custom)?;
        Ok(JsonPretty(value))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        let value: T = serde_json::from_str(&v).map_err(serde::de::Error::custom)?;
        Ok(JsonPretty(value))
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let value: T = serde_json::from_str(v).map_err(serde::de::Error::custom)?;
        Ok(JsonPretty(value))
    }
}

impl<'de, T> Deserialize<'de> for JsonPretty<T>
where
    T: DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(JsonPrettyVisitor {
            _marker: std::marker::PhantomData,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct TestStruct {
        field1: String,
        field2: i32,
    }

    #[test]
    #[ignore]
    fn print_json_pretty() {
        let test_data = TestStruct {
            field1: "value1".to_string(),
            field2: 42,
        };
        let json_pretty = JsonPretty(test_data);

        let json_value = serde_json::to_value(&json_pretty).unwrap();
        let serde_json::Value::String(serialized) = json_value else {
            panic!("Expected a JSON string");
        };
        println!("{serialized}");
    }

    #[test]
    fn test_json_pretty_serialize() {
        let test_data = TestStruct {
            field1: "value1".to_string(),
            field2: 42,
        };
        let json_pretty = JsonPretty(test_data);

        let serialized = serde_json::to_string(&json_pretty).unwrap();
        let expected = "\"{\\n  \\\"field1\\\": \\\"value1\\\",\\n  \\\"field2\\\": 42\\n}\"";
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_json_pretty_serialization() {
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

    #[test]
    fn deserialize_after_serialize_is_valid() {
        let test_data = TestStruct {
            field1: "value1".to_string(),
            field2: 42,
        };
        let json_pretty = JsonPretty(test_data.clone());

        let serialized = serde_json::to_string(&json_pretty).unwrap();
        let deserialized: JsonPretty<TestStruct> =
            serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.0, test_data);
    }
}