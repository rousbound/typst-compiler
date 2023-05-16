#[cfg(test)]
mod tests {

    use typst_genpdf;
    use serde_json::{Value, json};
    use std::fs;

    #[test]
    fn test_api() {
        if let Ok(data) = typst_genpdf::genpdf("tests/test.typ".into(), ".".into(), None) {
            fs::write("tests/test.pdf", data);
        } else {
            panic!("Test failed");

        }
    }

    #[test]
    fn test_api_with_json() {
        let my_value: Value = json!({
            "name": "John",
            "age": 30,
            "is_student": true,
            "hobbies": ["reading", "coding", "hiking"],
            "address": {
                "street": "123 Main St",
                "city": "Anytown",
                "state": "CA",
                "zip": "12345"
            }
        });

        
        if let Ok(data) = typst_genpdf::genpdf("tests/test_with_json.typ".into(), ".".into(), Some(my_value)) {
            fs::write("tests/test_with_json.pdf", data);
        } else {
            panic!("Test failed");

        }
    }

}
