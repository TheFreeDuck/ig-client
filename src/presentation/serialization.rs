// Custom serializer/deserializer for optional string to float conversion
pub mod string_as_float_opt {
    use serde::{self, Deserialize, Deserializer, Serializer};
    use serde_json::Value;

    pub fn serialize<S>(value: &Option<f64>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(v) => serializer.serialize_f64(*v), // Serializa como número
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;

        match value {
            Value::Null => Ok(None),
            Value::Number(num) => {
                if let Some(float) = num.as_f64() {
                    Ok(Some(float))
                } else {
                    Err(serde::de::Error::custom("Expected a float"))
                }
            }
            Value::String(s) => {
                if s.is_empty() {
                    return Ok(None);
                }
                s.parse::<f64>().map(Some).map_err(|_| {
                    serde::de::Error::custom(format!("Failed to parse string as float: {}", s))
                })
            }
            _ => Err(serde::de::Error::custom("Expected null, number or string")),
        }
    }
}

// Custom serializer/deserializer for optional string to bool conversion
pub mod string_as_bool_opt {
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(value: &Option<bool>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(v) => {
                let s = if *v { "1" } else { "0" };
                serializer.serialize_str(s)
            }
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt = Option::<String>::deserialize(deserializer)?;
        match opt {
            Some(s) => {
                // Handle empty strings as None
                if s.is_empty() {
                    return Ok(None);
                }
                match s.as_str() {
                    "0" => Ok(Some(false)),
                    "1" => Ok(Some(true)),
                    _ => Err(serde::de::Error::custom(format!(
                        "Invalid boolean value: {}",
                        s
                    ))),
                }
            }
            None => Ok(None),
        }
    }
}
