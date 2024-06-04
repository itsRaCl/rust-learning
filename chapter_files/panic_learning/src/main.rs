use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f_result = File::open("test.txt");

    let _f = match f_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("test.txt") {
                Ok(fc) => fc,
                Err(error) => panic!("error creating file {error:?}"),
            },
            other_error => {
                panic!("Unknown Error occured! {other_error:?}");
            }
        },
    };
    // This is the other way, imo better way to do this if you don't expect errors too often and
    // don't have anything through which you can handle the errors without disrupting the program
    // execution
    //let f : File = File::open("test.txt").expect("the file test.txt should be included")
}
