#![cfg(feature = "serde")]

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Foo {
    #[serde(with = "hex")]
    bar: Vec<u8>,
}

#[test]
fn serialize() {
    let foo = Foo {
        bar: vec![1, 10, 100],
    };

    let ser = serde_json::to_string(&foo).expect("serialization failed");
    assert_eq!(ser, r#"{"bar":"010a64"}"#);
}

#[test]
fn dserialize() {
    let foo = Foo {
        bar: vec![1, 10, 100],
    };

    let de: Foo = serde_json::from_str(r#"{"bar":"010a64"}"#).expect("deserialization failed");
    assert_eq!(de, foo);
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Bar {
    #[serde(
        serialize_with = "hex::serialize_upper",
        deserialize_with = "hex::deserialize"
    )]
    foo: Vec<u8>,
}

#[test]
fn serialize_upper() {
    let bar = Bar {
        foo: vec![1, 10, 100],
    };

    let ser = serde_json::to_string(&bar).expect("serialization failed");
    assert_eq!(ser, r#"{"foo":"010A64"}"#);
}

#[test]
fn dserialize_upper() {
    let bar = Bar {
        foo: vec![1, 10, 100],
    };

    let de: Bar = serde_json::from_str(r#"{"foo":"010A64"}"#).expect("deserialization failed");
    assert_eq!(de, bar);
}