#![allow(non_snake_case, unused_imports)]

use std::borrow::BorrowMut;
use std::mem::discriminant;
use crate::avp::*;
use std::io::{Error as IoError, ErrorKind, Stdin, Read};
use std::fs;
use crate::{avp::climate::Climate, error::Error, avp::visited::Visited, avp};
use std::ptr::read;
use serde_yaml::from_slice;
use crate::avp::climate::Climate::{Cold, Moderate, Warm};
use crate::avp::visited::Visited::{No, Yes};

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
    assert!(result.is_ok(), "{:?}", result);
    assert_eq!(result.unwrap(), expected);
}

#[test] /// Starting with visited since it's a simple enum
fn getters_can_get_visited() {
    // Given a location, initialized with a Yes as visited, determine if getter can accurately reflect
    let expected_result = Yes;
    let location_setup_data = r#"---
    locations:
      - name: "Some Name"
        climate: Moderate
        distance: 500
        visited: Yes"#.as_bytes();
    let locations = Locations::new(location_setup_data).unwrap();
    let location = locations.locations.first().unwrap();

    // When
    let result = location.visited();
    
    // Then
    assert_eq!(result, expected_result);
}

#[test] /// Starting with visited since it's a simple enum
fn setters_can_set_visited() {
    // Given setup values, can an updated location modify the setup value for visited
    let expected_result = Yes;
    let location_setup_data = r#"---
    locations:
      - name: "Some Name"
        climate: Moderate
        distance: 500
        visited: No"#.as_bytes();
    let mut locations = Locations::new(location_setup_data).unwrap();
    let location = locations.locations.first_mut().unwrap();

    // When
    let updated_location = location.set_visited(Visited::Yes);

    // Then
    assert_eq!(expected_result, location.visited());
}

#[test] /// Get climate from data
fn getters_can_get_climate(){
    // Given a location, initialized with a Warm as climate, determine if getter can accurately reflect
    let expected_result = Warm;
    let location_setup_data = r#"---
    locations:
      - name: "Some Name"
        climate: Warm
        distance: 500
        visited: Yes"#.as_bytes();
    let locations = Locations::new(location_setup_data).unwrap();
    let location = locations.locations.first().unwrap();

    // When
    let result = location.climate.clone();

    // Then
    debug_assert_eq!(result, expected_result);
}
#[test] /// Set Climate
fn setters_can_set_climate(){
    // Given setup values, can an updated location modify the setup value for climate
    let expected_result = Cold;
    let location_setup_data = r#"---
    locations:
      - name: "Some Name"
        climate: Warm
        distance: 500
        visited: Yes"#.as_bytes();
    let mut locations = Locations::new(location_setup_data).unwrap();
    let location = locations.locations.first_mut().unwrap();

    // When
    let updated_location = location.set_climate(Climate::Cold);

    // Then
    assert_eq!(expected_result, location.climate)
    }

#[test] /// Distance next
fn getters_can_get_distance(){
    // Given a location, initialized with 500 as distance, determine if getter can accurately reflect
    let expected_result = 500;
    let location_setup_data = r#"---
    locations:
      - name: "Some Name"
        climate: Warm
        distance: 500
        visited: Yes"#.as_bytes();
    let locations = Locations::new(location_setup_data).unwrap();
    let location = locations.locations.first().unwrap();

    // When
    let result = location.distance;

    // Then

    assert_eq!(expected_result, result);
}