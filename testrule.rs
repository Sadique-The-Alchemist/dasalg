fn rule_tester(value: Box<i32>) -> bool {
    value > Box::new(0)
}
fn main() {
    let test_value = 5;
    let boxed_value = Box::new(test_value);
    if rule_tester(boxed_value) {
        println!("The value {} passes the rule.", test_value);
    } else {
        println!("The value {} does not pass the rule.", test_value);
    }
    println!("The value is: {}", test_value);
}