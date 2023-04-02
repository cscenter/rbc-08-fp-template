#![deny(warnings)]

fn main() {
    println!("Hello, world!");

    execute1(/* no closure required, just execute to_string() */);

    let mut my_string = String::from("something");

    execute2(/* you should return my_string.len() in lambda */);

    execute3(/* execute my_string.push_str(&append) in lambda */);

    execute4(/* use move semantic */);
}

fn execute1(func: ???) {
    let result = func(42);
    assert_eq!("42", result);
}

fn execute2(func: ???) {
    let string_len = func();
    assert_eq!(9, string_len)
}

fn execute3(mut func: ???) {
    func(String::from(" good"));
}

fn execute4(func: ???) {
    let result: String = func();
    assert_eq!("something good", result);
}
