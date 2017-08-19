#[macro_use]
extern crate query_params;

#[derive(QueryParams)]
struct ExampleStruct {
    pub server: String,
    pub id: i32,
    running: bool,
    tags: Vec<String>,
} 

#[test]
fn test_ser_query_params_with_primitive_types() {
    let example_struct = ExampleStruct {
        server: "All might".to_string(),
        id: 42,
        running: true,
        tags: vec!["latest".to_string(), "linux".to_string()],
    };

    assert_eq!(
        example_struct.to_query_params(), 
        "?server=All might&id=42&running=true&tags=latest,linux"
    ); 
}


#[derive(QueryParams)]
struct EmptyStruct {} 

#[test]
fn test_ser_for_empty_struct() {
    let empty_struct = EmptyStruct{};

    assert_eq!(empty_struct.to_query_params(), ""); 
}


#[derive(QueryParams)]
struct OptsStruct {
    pretty: Option<bool>,
    format: Option<String>,
    depth: Option<i32>,
} 

#[test]
fn test_ser_with_optional_fields() {
    let opts_struct = OptsStruct {
        pretty: Some(true),
        format: Some("json".to_string()),
        depth: None,
    };

    assert_eq!(opts_struct.to_query_params(), "?pretty=true&format=json"); 
}