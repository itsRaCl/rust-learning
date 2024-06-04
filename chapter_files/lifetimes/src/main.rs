fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

fn main() {
    /*
     *    let r;
     *    {
     *        let x = 5;
     *        r = &x;
     *    }
     *
     *    println!("this is r {r}");
     */

    let string1 = String::from("abcd");
    {
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);

        println!("result : {}", result)
    }
}
