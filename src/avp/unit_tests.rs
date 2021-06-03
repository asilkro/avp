#![allow(non_snake_case, unused_imports)]
use crate::Error;
use std::mem::discriminant;
use super::*;
use std::io::{Error as IoError, ErrorKind};
use std::fs;

/*
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
*/

#[test]
fn new__non_existent_file_causes_error() {
    // Given
    let non_existent_file = "fakefile.yml".as_ref();

    // When
    let actual = Location::new(non_existent_file);

    // Then
    assert!(actual.is_err());
}

#[test]
fn new__existing_file_succeeds() {
    // Given
    let temp_file = tempfile::NamedTempFile::new().unwrap();

    // When -
    let result = Location::new(temp_file.path());

    // Then
    assert!(result.is_ok());
}