use std::collections::HashMap;
fn main() {
    let mut x = HashMap::new();

    x.entry(String::from("Blue")).or_insert(10);

    x.entry(String::from("Yellow")).or_insert(50);
    x.entry(String::from("Blue")).or_insert(50);

    println!("{x:?}");

    let demo_string = "hello world wonderful world";

    let mut x = HashMap::new();

    for i in demo_string.split_whitespace(){
        let count = x.entry(i).or_insert(0);
        *count+=1;
    }
    println!("{x:?}");
}
