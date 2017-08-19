#[macro_use]
extern crate query_params;

#[derive(QueryParams)]
struct ExampleStruct {
    pub field_1: i32,
    pub field_2: String,
    pub field_3: bool,
    pub field_4: i64,
    pub field_5: Vec<i32>,
} 

#[derive(QueryParams)]
struct EmptyStruct {} 

#[derive(QueryParams)]
struct OptsStruct {
    field_1: Option<i64>,
    field_2: Option<String>,
    field_3: Option<i32>,
} 

#[test]
fn test_query_params_is_eql() {
    let example_struct = ExampleStruct {
        field_1: 4,
        field_2: "hello".to_string(),
        field_3: true,
        field_4: 1000,
        field_5: vec!(4, 5, 8, 10),
    };

    assert_eq!(
        example_struct.to_query_params(), 
        "?field_1=4&field_2=hello&field_3=true&field_4=1000&field_5=4,5,8,10"
    ); 
}

#[test]
fn test_empty_struct() {
    let empty_struct = EmptyStruct{};
    assert_eq!(empty_struct.to_query_params(), ""); 
}

#[test]
fn test_struct_with_opt_fields() {
    let opts_struct = OptsStruct {
        field_1: Some(42),
        field_2: Some("test".to_string()),
        field_3: None,
    };

    assert_eq!(opts_struct.to_query_params(), "?field_1=42&field_2=test"); 
}