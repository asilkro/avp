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
use std::ptr::read;
use serde_yaml::from_slice;

#[test]
 fn new__empty_reader_succeeds() {
     // Given
    let data_being_read = r#""#;

     // When -
     let result = Locations::new(data_being_read.as_bytes());

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
fn new__valid_yaml_data_deserializes_successfully() {
    // Given
    let data_being_read = r#"---
    locations:
      - name: "Some Name"
        climate: Moderate
        distance: 500
        visited: Yes"#;
    let expected = Locations {
        locations: vec![Location {
            name: String::from("Some Name"),
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
