fn main() {
    let mut s = String::from("bar");

    s.push_str("foo");

    println!("{s}");

    let s1 = String::from("foo");
    let s2 = String::from("bar");

    let s3 = s1 + &s2[..];

    println!("s3 is {s3}");

    println!("s2 still can be used like so {s2}");

    //println!("but s1 can't like so {s1}");
    //
}
