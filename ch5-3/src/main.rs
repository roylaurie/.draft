use gtk::prelude::*;

fn main() {
    let integer_value = 10.to_value();

    let integer = integer_value
        .get::<i32>()
        .expect("Value not i32");

    assert_eq!(10, integer);

    let string_value = "Hello!".to_value();
    let string = string_value
        .get::<String>()
        .expect("String type");

    assert_eq!("Hello!".to_string(), string);

    let string_some_value = "Howdy!".to_value();
    let string_none_value = None::<String>.to_value();

    let string_some = string_some_value
        .get::<Option<String>>()
        .expect("Some type");
    let string_none = string_none_value
        .get::<Option<String>>()
        .expect("None type");

    assert_eq!(Some("Howdy!".to_string()), string_some);
    assert_eq!(None, string_none);

    let integer_variant = 10.to_variant();
    let integer_v = integer_variant
        .get::<i32>()
        .expect("Type not i32");

    assert_eq!(10, integer_v);

    let vec_variant = vec!["Hi", "there"].to_variant();
    let vec = vec_variant
        .get::<Vec<String>>()
        .expect("Type vec string");

    assert_eq!("Hi", vec[0]);

    println!("passed");
}
