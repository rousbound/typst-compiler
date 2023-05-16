#[cfg(test)]
mod tests {

    use typst_genpdf::Compiler;
    use serde_json::{Value, json};
    use std::fs;

    pub fn is_pdf(bytes: &[u8]) -> bool {
        let pdf_signature: &[u8] = b"%PDF-";
        bytes.starts_with(pdf_signature)
    }

    #[test]
    fn test_api() {
        let mut compiler: Compiler = Compiler::new(".".into());
        if let Ok(data) = compiler.compile("tests/test.typ".into(), None) {
            if is_pdf(&data) {
                fs::write("tests/test.pdf", data);
            } else {
                panic!("Test failed");
            }

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

        
        let mut compiler: Compiler = Compiler::new(".".into());
        if let Ok(data) = compiler.compile("tests/test.typ".into(), None) {
            if is_pdf(&data) {
                fs::write("tests/test.pdf", data);
            } else {
                panic!("Test failed");
            }
        } else {
            panic!("Test failed");

        }
    }

}
