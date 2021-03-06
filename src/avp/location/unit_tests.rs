#![allow(non_snake_case, unused_imports)]

use std::borrow::BorrowMut;
use std::mem::discriminant;
use crate::avp::*;
use std::io::{Error as IoError, ErrorKind, Stdin, Read};
use std::{fs};
use std::ops::RangeInclusive;
use crate::{avp::climate::Climate, error::Error, avp::visited::Visited, avp};
use std::ptr::read;
use serde_yaml::from_slice;
use crate::avp::climate::Climate::{Cold, Hot, Moderate, Warm};
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
fn getters_can_get_climate() {
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
fn setters_can_set_climate() {
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
fn getters_can_get_distance() {
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

    assert_eq!(result, expected_result);
}
//#[test] /// Set Distance
//fn setters_can_set_distance() {
//    // Given
//    let expected_result = 250; // Not same as initialized
//    let location_setup_data = r#"---
//    locations:
//      - name: "Some Name"
//        climate: Warm
//        distance: 500
//        visited: Yes"#.as_bytes();
//    let mut locations = Locations::new(location_setup_data).unwrap();
//    let location = locations
//}

#[test] // How do we want to access location
fn find_name_returns_valid_location() {
    // Given a Locations
    let data = r#"---
        name: "Another Name"
        climate: Warm
        distance: 500
        visited: Yes"#.as_bytes();
    let expected_result = serde_yaml::from_slice(data.as_ref()).unwrap();
    let location_setup_data = r#"---
    locations:
      - name: "Some Name"
        climate: Cold
        distance: 250
        visited: No
      - name: "Another Name"
        climate: Warm
        distance: 500
        visited: Yes"#.as_bytes();
    let locations = Locations::new(location_setup_data).unwrap();

    // When
    let result = locations.find_name("Another Name");

    // Then
    assert_eq!(result, Some(&expected_result));
}

    /*  #[test] // Robust find can use any parameter, this tests with Visited
    fn robust_find_can_use_visited() {
        // Given
        let location_setup_data = r#"---
    locations:
      - name: "Some Name"
        climate: Cold
        distance: 250
        visited: No
      - name: "Another Name"
        climate: Warm
        distance: 500
        visited: Yes"#.as_bytes();
        let locations = Locations::new(location_setup_data).unwrap();
//
        // When
        // name, climate, distance, name
        let result = locations.find_robust("<some_str>", Climate::<some_variant>, 1..=10, Visited::<some_variant>);
//
        // future code looks like
        fn find_robust(&self, name: Option<&'static str>, climate: Option<Climate>, distance: Option<RangeInclusive<u32>>, visited: Option<Visited>) { ... }
        // which locations have I visited?
        let result = locations.find_robust(None, None, None, Some(Yes));
        let result2 = locations.find_robust(Some("Calgary"), None, None, Some(Yes));
    } */

/*#[test]
fn find_can_use_visited() {
        // Given
        let location_setup_data = r#"---
    locations:
      - name: "Some Name"
        climate: Cold
        distance: 250
        visited: No
      - name: "Another Name"
        climate: Warm
        distance: 500
        visited: Yes"#.as_bytes();
        let locations = Locations::new(location_setup_data).unwrap();
        let search_data = r#"---
        name: "Another Name"
        climate: Warm
        distance: 500
        visited: Yes"#.as_bytes();
        let expected_result = serde_yaml::from_slice(search_data.as_ref()).unwrap();

        // When
        let result = locations.find(Yes); // find only knows name

        // Then
        assert_eq!(result, Some(&expected_result));
    }
*/

/*#[test]
fn find_can_use_climate() {
    // Given
    let location_setup_data = r#"---
    locations:
      - name: "Some Name"
        climate: Cold
        distance: 250
        visited: No
      - name: "Another Name"
        climate: Warm
        distance: 500
        visited: Yes"#.as_bytes();
    let locations = Locations::new(location_setup_data).unwrap();
    let search_data = r#"---
        name: "Another Name"
        climate: Warm
        distance: 500
        visited: Yes"#.as_bytes();
    let expected_result = serde_yaml::from_slice(search_data.as_ref()).unwrap();

        // When
    let result = locations.find(Warm); // find only knows name

        // Then
    assert_eq!(result, Some(&expected_result));
}*/

/*#[test]
fn find_can_use_distance() {
    // Given
    let location_setup_data = r#"---
locations:
  - name: "Some Name"
    climate: Cold
    distance: 250
    visited: No
  - name: "Another Name"
    climate: Warm
    distance: 500
    visited: Yes"#.as_bytes();
    let locations = Locations::new(location_setup_data).unwrap();
    let search_data = r#"---
    name: "Another Name"
    climate: Warm
    distance: 500
    visited: Yes"#.as_bytes();
    let expected_result = serde_yaml::from_slice(search_data.as_ref()).unwrap();

    // When
    let result = locations.find(500); // find only knows name

    // Then
    assert_eq!(result, Some(&expected_result));
}*/

#[test]
fn find_stuff_will_return_exactly_one_location() {
    // Given setup data and known implementations
    let location_setup_data = r#"---
locations:
  - name: "Some Name"
    climate: Cold
    distance: 250
    visited: No
  - name: "Another Name"
    climate: Warm
    distance: 500
    visited: Yes"#.as_bytes();
    let locations = Locations::new(location_setup_data).unwrap();
    let search_data = "Another Name".as_bytes(); // Search data represented as bytes
    let expected_result = locations.find_name("Another Name");
    assert_eq!(expected_result.unwrap().name(), "Another Name");

    //When we use find_stuff
    let result = locations.find_stuff("Another Name", Warm, 500, Yes);

    //Then we check the results match our expectations
    assert_eq!(result, expected_result);
}