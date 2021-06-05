use std::collections::HashMap;
fn main() {
    let mut general_map: HashMap<&str, i8> = HashMap::new();
    general_map.insert("test", 25);

    // we use Option because it was expecting std::option::Option when finding i8
    let outcome: Option<&i8> = general_map.get("test");
    
    // unwrap() crashes the program if no value is presented
    println!("{}", outcome.unwrap());
}  
