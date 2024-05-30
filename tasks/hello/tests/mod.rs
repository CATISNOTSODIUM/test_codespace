use hello::*;

#[test]
fn grader() {
    let result = hello();
    assert_eq!(result, "Hello world".to_string());
}
