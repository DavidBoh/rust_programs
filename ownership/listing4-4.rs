fn main() {
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

}
// when scope is over
// s1 had the value of "hello" but its value is dropped. "hello" came from funcion
// s2 was moved (in s3 assignment and func calling), nothing happenes
// s3 had the value of "hello" but is dropped
fn gives_ownership() -> String {

    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}