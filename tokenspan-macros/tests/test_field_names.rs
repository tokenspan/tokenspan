use tokenspan_extra::FieldNamesExt;
use tokenspan_macros::FieldNames;

#[test]
fn test_field_names() {
    #[derive(FieldNames)]
    struct TestStruct {
        pub id: u32,
        pub name: String,
        pub age: Option<u32>,
    }

    println!("{:?}", TestStruct::FIELDS);
}
