#![allow(non_snake_case, unused_imports)]
use crate::Error;
use std::mem::discriminant;
use super::*;
use std::io::{Error as IoError, ErrorKind};
use std::fs;

#[test]
fn new__non_existent_file_causes_error() {
    // Given
    let non_existent_file = "fakefile.yml".as_ref();

    // When
    let actual = Locations::new(non_existent_file);

    // Then
    assert!(actual.is_err());
}

#[test]
fn new__empty_existing_file_succeeds() {
    // Given
    let temp_file = tempfile::NamedTempFile::new().unwrap();

    // When -
    let result = Locations::new(temp_file.path());

    // Then
    // Unwrap for test only, plz no ship
    assert!(result.is_ok(), "{:?}", result);
    assert!(result.unwrap().is_empty());

}
//
// #[test]
// fn new__invalid_file_returns_error() {
//     // Given - broken file (it's a .png not a YML file)
//     let invalid_file = "bad-configfile.yml".as_ref();
//
//     // When - a broken file is attempted
//     let sut = Locations::new(invalid_file).unwrap();
//
//     // Then
//     assert_eq!(sut.is_ok(), "{:?}", "{}");
// }