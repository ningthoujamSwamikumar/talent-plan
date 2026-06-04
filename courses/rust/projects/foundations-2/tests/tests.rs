use type_machinist::{
    CsvToJson, JsonPrettify, Lowercase, Pipeline, PipelineBuilder, PipelineError, Transform,
    TransformKind, TransformSpec, TrimWhitespace, Uppercase,
};

// =========================================================================
// Part 2 -- Individual transform tests
// =========================================================================

#[test]
fn uppercase_basic() {
    let t = Uppercase;
    assert_eq!(t.transform("hello world").unwrap(), "HELLO WORLD");
}

#[test]
fn uppercase_already_upper() {
    let t = Uppercase;
    assert_eq!(t.transform("HELLO").unwrap(), "HELLO");
}

#[test]
fn uppercase_empty() {
    let t = Uppercase;
    assert_eq!(t.transform("").unwrap(), "");
}

#[test]
fn lowercase_basic() {
    let t = Lowercase;
    assert_eq!(t.transform("HELLO WORLD").unwrap(), "hello world");
}

#[test]
fn lowercase_mixed() {
    let t = Lowercase;
    assert_eq!(t.transform("HeLLo").unwrap(), "hello");
}

#[test]
fn trim_whitespace_basic() {
    let t = TrimWhitespace;
    assert_eq!(t.transform("  hello  ").unwrap(), "hello");
}

#[test]
fn trim_whitespace_newlines() {
    let t = TrimWhitespace;
    assert_eq!(t.transform("\n\thello\n\t").unwrap(), "hello");
}

#[test]
fn trim_whitespace_no_change() {
    let t = TrimWhitespace;
    assert_eq!(t.transform("hello").unwrap(), "hello");
}

#[test]
fn csv_to_json_basic() {
    let t = CsvToJson;
    let input = "name,age\nAlice,30\nBob,25";
    let result = t.transform(input).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    let arr = parsed.as_array().unwrap();
    assert_eq!(arr.len(), 2);
    assert_eq!(arr[0]["name"], "Alice");
    assert_eq!(arr[0]["age"], "30");
    assert_eq!(arr[1]["name"], "Bob");
    assert_eq!(arr[1]["age"], "25");
}

#[test]
fn csv_to_json_single_row() {
    let t = CsvToJson;
    let input = "x,y\n1,2";
    let result = t.transform(input).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    let arr = parsed.as_array().unwrap();
    assert_eq!(arr.len(), 1);
    assert_eq!(arr[0]["x"], "1");
    assert_eq!(arr[0]["y"], "2");
}

#[test]
fn csv_to_json_mismatched_columns() {
    let t = CsvToJson;
    let input = "name,age\nAlice,30,extra";
    let result = t.transform(input);
    assert!(result.is_err(), "Should fail when column count mismatches");
}

#[test]
fn csv_to_json_empty_body() {
    let t = CsvToJson;
    let input = "name,age";
    let result = t.transform(input).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    let arr = parsed.as_array().unwrap();
    assert!(arr.is_empty());
}

#[test]
fn json_prettify_basic() {
    let t = JsonPrettify;
    let input = r#"{"name":"Alice","age":30}"#;
    let result = t.transform(input).unwrap();
    // Pretty-printed JSON should contain newlines and indentation.
    assert!(result.contains('\n'));
    assert!(result.contains("  "));
    // It should still parse back to the same value.
    let original: serde_json::Value = serde_json::from_str(input).unwrap();
    let pretty: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(original, pretty);
}

#[test]
fn json_prettify_invalid_json() {
    let t = JsonPrettify;
    let result = t.transform("not json at all");
    assert!(result.is_err(), "Should fail on invalid JSON");
}

// =========================================================================
// Part 2 -- Transform name tests
// =========================================================================

#[test]
fn transform_names() {
    assert_eq!(Uppercase.name(), "uppercase");
    assert_eq!(Lowercase.name(), "lowercase");
    assert_eq!(TrimWhitespace.name(), "trim_whitespace");
    assert_eq!(CsvToJson.name(), "csv_to_json");
    assert_eq!(JsonPrettify.name(), "json_prettify");
}

// =========================================================================
// Part 3 -- Error handling / From conversions
// =========================================================================

#[test]
fn csv_parse_error_converts_to_pipeline_error() {
    use type_machinist::error::CsvParseError;
    let csv_err = CsvParseError {
        message: "bad csv".to_string(),
    };
    let pipeline_err: PipelineError = csv_err.into();
    let msg = format!("{}", pipeline_err);
    assert!(msg.contains("bad csv"));
}

#[test]
fn json_error_converts_to_pipeline_error() {
    use type_machinist::error::JsonError;
    let json_err = JsonError {
        message: "bad json".to_string(),
    };
    let pipeline_err: PipelineError = json_err.into();
    let msg = format!("{}", pipeline_err);
    assert!(msg.contains("bad json"));
}

// =========================================================================
// Part 4 -- Pipeline composition
// =========================================================================

#[test]
fn pipeline_single_transform() {
    let mut p = Pipeline::new();
    p.add_transform(Uppercase);
    assert_eq!(p.run("hello").unwrap(), "HELLO");
}

#[test]
fn pipeline_chain_two() {
    let mut p = Pipeline::new();
    p.add_transform(TrimWhitespace);
    p.add_transform(Uppercase);
    assert_eq!(p.run("  hello  ").unwrap(), "HELLO");
}

#[test]
fn pipeline_chain_three() {
    let mut p = Pipeline::new();
    p.add_transform(TrimWhitespace);
    p.add_transform(Lowercase);
    p.add_transform(Uppercase);
    // trim -> "hello" -> "hello" -> "HELLO"
    assert_eq!(p.run("  Hello  ").unwrap(), "HELLO");
}

#[test]
fn pipeline_error_propagation() {
    let mut p = Pipeline::new();
    p.add_transform(JsonPrettify);
    let result = p.run("not json");
    assert!(result.is_err());
}

#[test]
fn pipeline_empty_returns_input() {
    let p = Pipeline::new();
    assert_eq!(p.run("hello").unwrap(), "hello");
}

#[test]
fn pipeline_len_and_is_empty() {
    let mut p = Pipeline::new();
    assert!(p.is_empty());
    assert_eq!(p.len(), 0);
    p.add_transform(Uppercase);
    assert!(!p.is_empty());
    assert_eq!(p.len(), 1);
}

// =========================================================================
// Part 5 -- FromStr for TransformKind
// =========================================================================

#[test]
fn transform_kind_from_str() {
    assert_eq!("uppercase".parse::<TransformKind>().unwrap(), TransformKind::Uppercase);
    assert_eq!("lowercase".parse::<TransformKind>().unwrap(), TransformKind::Lowercase);
    assert_eq!(
        "trim_whitespace".parse::<TransformKind>().unwrap(),
        TransformKind::TrimWhitespace
    );
    assert_eq!("csv_to_json".parse::<TransformKind>().unwrap(), TransformKind::CsvToJson);
    assert_eq!(
        "json_prettify".parse::<TransformKind>().unwrap(),
        TransformKind::JsonPrettify
    );
}

#[test]
fn transform_kind_from_str_unknown() {
    let result = "foobar".parse::<TransformKind>();
    assert!(result.is_err());
}

// =========================================================================
// Part 5 -- TryFrom<&str> for TransformSpec
// =========================================================================

#[test]
fn transform_spec_try_from() {
    let spec = TransformSpec::try_from("uppercase").unwrap();
    assert_eq!(spec.kind, TransformKind::Uppercase);
}

#[test]
fn transform_spec_try_from_invalid() {
    let result = TransformSpec::try_from("nonexistent");
    assert!(result.is_err());
}

// =========================================================================
// Part 6 -- PipelineBuilder
// =========================================================================

#[test]
fn builder_basic() {
    let pipeline = PipelineBuilder::new()
        .add(TrimWhitespace)
        .add(Uppercase)
        .build();
    assert_eq!(pipeline.run("  hello  ").unwrap(), "HELLO");
}

#[test]
fn builder_empty() {
    let pipeline = PipelineBuilder::new().build();
    assert_eq!(pipeline.run("pass-through").unwrap(), "pass-through");
}

#[test]
fn builder_single() {
    let pipeline = PipelineBuilder::new().add(Lowercase).build();
    assert_eq!(pipeline.run("SHOUT").unwrap(), "shout");
}

// =========================================================================
// Integration -- CSV to pretty JSON pipeline
// =========================================================================

#[test]
fn pipeline_csv_to_pretty_json() {
    let mut p = Pipeline::new();
    p.add_transform(CsvToJson);
    p.add_transform(JsonPrettify);
    let input = "name,age\nAlice,30";
    let result = p.run(input).unwrap();
    // Should be valid, pretty-printed JSON.
    assert!(result.contains('\n'));
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    let arr = parsed.as_array().unwrap();
    assert_eq!(arr.len(), 1);
    assert_eq!(arr[0]["name"], "Alice");
}
