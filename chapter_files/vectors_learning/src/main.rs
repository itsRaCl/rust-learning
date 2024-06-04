fn main() {
    let mut v = vec![1, 2];

    // the other way to access is indexing but it will panic on accessing out of bounds elements

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third is: {third}"),
        None => println!("The third element doesn't exist"),
    }

    let first: &mut i32 = &mut v[0];

    println!("This is a test for mutable reference {first}");
    *first = 2;

    let first: Option<&i32> = v.get(0);
    match first{
        Some(first) => println!("{first}"),
        None => println!("Error!")
    }
}
