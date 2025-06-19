#[cfg(test)]
mod tests {
    use ig_client::impl_json_display;
    use serde::Serialize;
    use std::fmt;

    // Test struct to verify the macro implementation
    #[derive(Serialize)]
    struct TestStruct {
        field1: String,
        field2: i32,
        field3: Option<bool>,
    }

    // Apply the macro to implement Display
    impl_json_display!(TestStruct);

    #[test]
    fn test_json_display_macro() {
        // Create a test struct instance
        let test_struct = TestStruct {
            field1: "test value".to_string(),
            field2: 42,
            field3: Some(true),
        };

        // Convert to string using Display trait
        let display_output = format!("{}", test_struct);

        // Parse the output back to verify it's valid JSON
        let parsed_json: serde_json::Value = serde_json::from_str(&display_output).unwrap();

        // Verify the JSON structure matches our struct
        assert_eq!(parsed_json["field1"], "test value");
        assert_eq!(parsed_json["field2"], 42);
        assert_eq!(parsed_json["field3"], true);
    }

    #[test]
    fn test_json_display_with_null() {
        // Create a test struct with None value
        let test_struct = TestStruct {
            field1: "another test".to_string(),
            field2: -10,
            field3: None,
        };

        // Convert to string using Display trait
        let display_output = format!("{}", test_struct);

        // Parse the output back to verify it's valid JSON
        let parsed_json: serde_json::Value = serde_json::from_str(&display_output).unwrap();

        // Verify the JSON structure matches our struct
        assert_eq!(parsed_json["field1"], "another test");
        assert_eq!(parsed_json["field2"], -10);
        assert_eq!(parsed_json["field3"], serde_json::Value::Null);
    }

    // Test with multiple structs to verify the macro can handle multiple types
    #[derive(Serialize)]
    struct AnotherStruct {
        name: String,
        value: f64,
    }

    impl_json_display!(AnotherStruct);

    #[test]
    fn test_json_display_multiple_structs() {
        let another_struct = AnotherStruct {
            name: "test name".to_string(),
            value: 3.14159,
        };

        // Convert to string using Display trait
        let display_output = format!("{}", another_struct);

        // Parse the output back to verify it's valid JSON
        let parsed_json: serde_json::Value = serde_json::from_str(&display_output).unwrap();

        // Verify the JSON structure matches our struct
        assert_eq!(parsed_json["name"], "test name");
        assert!(parsed_json["value"].as_f64().unwrap() - 3.14159 < f64::EPSILON);
    }
}
