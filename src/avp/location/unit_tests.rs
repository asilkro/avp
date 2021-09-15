#![allow(non_snake_case, unused_imports)]

use std::mem::discriminant;
use crate::avp::*;
use std::io::{Error as IoError, ErrorKind, Stdin, Read};
use std::fs;
use crate::{
    avp::climate::Climate,
    error::Error,
    avp::visited::Visited
};
use test::assert_test_result;
use std::ptr::read;
use serde_yaml::from_slice;

#[test]
 fn new__empty_reader_succeeds() {
     // Given
     let temp_file = tempfile::NamedTempFile::new().unwrap();

     // When -
     let result = Locations::new(temp_file);

     // Then
     // Unwrap for test only, plz no ship
     assert!(result.is_ok(), "{:?}", result);
     assert!(result.unwrap().is_empty());

 }

#[test]
fn new__invalid_data_returns_error() {
    // Given - broken file (it's a .png not a YML file)
    let invalid_file = [0x2e_u8, 0x5c, 0x62, 0x61];

    // When - a broken file is attempted
    let result = Locations::new(invalid_file.as_ref());

    // Then
    assert!(result.is_err(), "{:?}", result);
}

#[test]
fn new__reader_valid_data_parses_successfully_fragile() {
    // Given
    let data_being_read = r"---
    locations:
      - climate: Moderate
        distance: 500
        visited: Yes";
    let expected = Locations {
        locations: vec![Location {
            climate: Climate::Moderate,
            distance: 500,
            visited: Visited::Yes,
    }]
    };


    // When - data is being parsed
    let result = Locations::new(data_being_read.as_bytes());

    // Then
    assert!(result.is_ok(),"{:?}", result);
    assert_eq!(result.unwrap(), expected);
}
// TODO: Make a test that implements in a less "copy and paste your data in" way
#[test]
fn new__reader_valid_data_parses_successfully_robust() {
    // Given
    let data_being_read = Stdin::read_to_end(data_contents.as_bytes());
    let expected = the_thing_being_read_above_in_another_format;

    // When
    let result = data_being_read(&mut self, buf: &mut Vec<u8>);

    // Then
    assert!(result.is_ok(),"{:?}", result);
    assert_eq!(result.unwrap(), expected);
}