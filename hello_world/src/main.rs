fn main() {


    // return_error().unwrap();
    println!("{}", return_error());
}

fn return_error() -> i64 {
    let val = "u";

    let duration: i64 = val.parse().unwrap_or(1);

    duration
}