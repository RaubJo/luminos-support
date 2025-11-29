use crate::contracts::{Arrayable, JsonSerializable, Jsonable, Vectorable};
use serde_json::Value;
use std::collections::HashMap;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Fluent {
    attributes: HashMap<String, Value>,
}

impl Fluent {
    ///
    /// Create a new fluent instance.
    ///
    pub fn new() -> Self {
        Self {
            attributes: HashMap::new(),
        }
    }

    ///
    /// Create a new fluent instance.
    ///
    /// Proxies to Fluent::new()
    ///
    pub fn make() -> Self {
        Self::new()
    }

    ///
    /// Create a fluent instance from the given attributes.
    ///
    pub fn from(attributes: HashMap<String, Value>) -> Self {
        Self { attributes }
    }

    ///
    /// Set an attribute on the fluent instance using "dot" notation.
    ///
    pub fn set(&mut self, key: &str, value: impl Into<Value>) {
        self.attributes.insert(key.to_string(), value.into());
    }

    ///
    /// Get an attribute from the fluent instance using "dot" notation.
    ///
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.attributes.get(key)
    }

    ///
    /// Get a value off the instance and cast to str.
    ///
    pub fn get_as_str(&self, key: &str) -> Option<&str> {
        self.attributes.get(key).and_then(|value| value.as_str())
    }

    ///
    /// Fill the fluent instance with the attributes.
    ///
    pub fn fill(&mut self, attributes: HashMap<String, Value>) -> &mut Self {
        self.attributes.extend(attributes);
        self
    }

    ///
    /// Does the "key" exist on the fluent instance?
    ///
    pub fn has(&self, key: &str) -> bool {
        self.attributes.contains_key(key)
    }

    ///
    /// Get all attributes of the fluent instance
    ///
    pub fn all(&self) -> &HashMap<String, Value> {
        &self.attributes
    }

    ///
    /// Get and attribute from the fluent instance.
    ///
    pub fn value(&self, key: &str, default: Value) -> Value {
        self.get(key).cloned().unwrap_or(default)
    }

    ///
    /// Wrap the value of key in a new fluent instance.
    ///
    pub fn scope(&self, key: &str, default: Value) -> Self {
        let mut map = HashMap::new();
        map.insert(key.to_string(), self.value(key, default));

        Self { attributes: map }
    }

    ///
    /// Get the attributes on the fluent instance.
    ///
    pub fn get_attributes(&self) -> &HashMap<String, Value> {
        &self.attributes
    }

    ///
    /// Insert an optional value into the Fluent instance.
    /// If the value is Some, insert it as-is.
    /// If None, insert serde_json::Value::Null.
    ///
    pub fn set_from_option(&mut self, key: &str, value: Option<impl Into<Value>>) {
        match value {
            Some(v) => {
                self.attributes.insert(key.to_string(), v.into());
            }
            None => {
                self.attributes.insert(key.to_string(), Value::Null);
            }
        }
    }
}

impl Index<&str> for Fluent {
    type Output = Value;

    fn index(&self, key: &str) -> &Self::Output {
        self.attributes.get(key).expect("Key does not exist")
    }
}

impl IndexMut<&str> for Fluent {
    fn index_mut(&mut self, key: &str) -> &mut Self::Output {
        self.attributes.get_mut(key).expect("Key does not exist")
    }
}

impl JsonSerializable for Fluent {
    type Value = serde_json::Value;

    fn json_serialize(&self) -> Value {
        serde_json::Value::Object(self.attributes.clone().into_iter().collect())
    }
}

impl Jsonable for Fluent {
    fn to_json(&self) -> String {
        serde_json::to_string(&self.attributes).unwrap_or_else(|_| "{}".to_string())
    }

    fn to_json_pretty(&self) -> String {
        serde_json::to_string_pretty(&self.attributes).unwrap_or_else(|_| "{}".to_string())
    }
}

impl Vectorable for Fluent {
    type T = Value;

    fn to_vec(&self) -> Vec<Value> {
        self.attributes.iter().map(|(k, v)| v.clone()).collect()
    }
}

impl Arrayable for Fluent {
    type T = Value;

    fn to_array(&self) -> Vec<Value> {
        self.attributes.iter().map(|(k, v)| v.clone()).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::fluent;

    #[test]
    fn test_fluent_macro() {
        let result = fluent! {
            name: "John",
            age: 20,
            city: "Dallas"
        };

        let mut temp: HashMap<String, Value> = std::collections::HashMap::new();
        temp.insert("name".to_string(), "John".into());
        temp.insert("age".to_string(), 20.into());
        temp.insert("city".to_string(), "Dallas".into());

        let expected = Fluent { attributes: temp };

        assert_eq!(result, expected);
    }

    #[test]
    fn can_make_new_fluent_instance() {
        let result = Fluent::make();

        let expected = Fluent {
            attributes: HashMap::new(),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn can_make_filled_fluent_from_from() {
        let mut map: HashMap<String, Value> = std::collections::HashMap::new();
        map.insert("name".to_string(), "John".into());
        map.insert("age".to_string(), 20.into());
        map.insert("city".to_string(), "Dallas".into());

        let result = Fluent::from(map.to_owned());

        let expected = Fluent { attributes: map };

        assert_eq!(result, expected);
    }

    #[test]
    fn can_check_that_fluent_has_value_and_not_has_value() {
        let mut map: HashMap<String, Value> = std::collections::HashMap::new();
        map.insert("name".to_string(), "John".into());
        map.insert("age".to_string(), 20.into());
        map.insert("city".to_string(), "Dallas".into());

        let result = Fluent::from(map.to_owned()).has("name");

        assert!(result);

        let result = Fluent::from(map.to_owned()).has("job");

        assert!(!result);
    }

    #[test]
    fn can_set_and_get_value_on_instance() {
        let mut fluent = fluent! {
            name: "John",
            age: 20,
            city: "Dallas"
        };

        let result = fluent.get("name").unwrap().to_owned();

        let expected = Value::String("John".to_string());

        assert_eq!(result, expected);

        fluent.set("job", "Developer");

        let result = fluent.get("job").unwrap().to_owned();

        let expected = Value::String("Developer".to_string());

        assert_eq!(result, expected);
    }

    #[test]
    fn can_get_all_values_on_instance() {
        let mut fluent = fluent! {
            name: "John",
            age: 20,
            city: "Dallas"
        };

        let result = fluent.all().to_owned();

        let mut map: HashMap<String, Value> = HashMap::new();
        map.insert("name".to_string(), "John".into());
        map.insert("age".to_string(), 20.into());
        map.insert("city".to_string(), "Dallas".into());

        assert_eq!(result, map);
    }

    #[test]
    fn can_get_a_value_and_default_value_when_empty() {
        let mut fluent = fluent! {
            name: "John",
            age: 20,
            city: "Dallas"
        };

        let result = fluent.value("name", Value::String("Bob".to_string()));

        let expected = Value::String("John".to_string());

        assert_eq!(result, expected);

        let result = fluent.value("job", Value::String("Developer".to_string()));

        let expected = Value::String("Developer".to_string());

        assert_eq!(result, expected);
    }

    #[test]
    fn can_scope_value_to_new_fluent_instance() {
        let mut fluent = fluent! {
            name: "John",
            age: 20,
            city: "Dallas"
        };

        let result = fluent.scope("name", Value::String("Bob".to_string()));

        let expected = fluent! {
            name: "John"
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn can_get_all_attributes_on_instance() {
        let mut fluent = fluent! {
            name: "John",
            age: 20,
            city: "Dallas"
        };

        let result = fluent.get_attributes().to_owned();

        let mut map: HashMap<String, Value> = HashMap::new();
        map.insert("name".to_string(), "John".into());
        map.insert("age".to_string(), 20.into());
        map.insert("city".to_string(), "Dallas".into());

        assert_eq!(result, map);
    }
}
