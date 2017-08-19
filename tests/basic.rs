#[macro_use]
extern crate query_params;

trait QueryParams {
    fn to_query_params(&self) -> String;
}

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