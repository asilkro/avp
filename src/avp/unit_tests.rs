#![allow(unused_imports)]
use super::*;

#[test]
fn two_plus_two() {
    // Given (minimal setup for test)
    let expected = 4;
    let a = 2;
    let b = 2;

    // When (execution of test, capture result)
    let result = a+b;

    // Then (check the actual matches what is expected)
    assert_eq!(result, expected);
}
#[test]
fn () {

}
