#![cfg(test)]

use std::{
    collections::{BTreeMap, HashMap},
    fmt::Debug,
};

use once_cell::sync::Lazy;
use prost::Message;
use prost_dynamic::{DynamicMessage, FileDescriptor};
use prost_types::FileDescriptorSet;

include!(concat!(env!("OUT_DIR"), "/test.rs"));

pub static TEST_FILE_DESCRIPTOR: Lazy<FileDescriptor> = Lazy::new(|| {
    FileDescriptor::new(
        FileDescriptorSet::decode(
            include_bytes!(concat!(env!("OUT_DIR"), "/file_descriptor_set.bin")).as_ref(),
        )
        .unwrap(),
    )
    .unwrap()
});

#[test]
fn roundtrip_scalars() {
    roundtrip(
        &Scalars {
            double: 1.1,
            float: 2.2,
            int32: 3,
            int64: 4,
            uint32: 5,
            uint64: 6,
            sint32: 7,
            sint64: 8,
            fixed32: 9,
            fixed64: 10,
            sfixed32: 11,
            sfixed64: 12,
            r#bool: true,
            string: "5".to_owned(),
            bytes: b"6".to_vec(),
        },
        ".test.Scalars",
    );
}

#[test]
fn roundtrip_scalar_arrays() {
    roundtrip(
        &ScalarArrays {
            double: vec![1.1, 2.2],
            float: vec![3.3f32, 4.4f32],
            int32: vec![5, -6],
            int64: vec![7, -8],
            uint32: vec![9, 10],
            uint64: vec![11, 12],
            sint32: vec![13, -14],
            sint64: vec![15, -16],
            fixed32: vec![17, 18],
            fixed64: vec![19, 20],
            sfixed32: vec![21, -22],
            sfixed64: vec![23, 24],
            r#bool: vec![true, false],
            string: vec!["25".to_owned(), "26".to_owned()],
            bytes: vec![b"27".to_vec(), b"28".to_vec()],
        },
        ".test.ScalarArrays",
    );
}

#[test]
fn roundtrip_complex_type() {
    roundtrip(
        &ComplexType {
            string_map: HashMap::from([
                (
                    "1".to_owned(),
                    Scalars {
                        double: 1.1,
                        float: 2.2,
                        int32: 3,
                        ..Default::default()
                    },
                ),
                (
                    "2".to_owned(),
                    Scalars {
                        int64: 4,
                        uint32: 5,
                        uint64: 6,
                        ..Default::default()
                    },
                ),
            ]),
            int_map: HashMap::from([
                (
                    3,
                    Scalars {
                        sint32: 7,
                        sint64: 8,
                        fixed32: 9,
                        ..Default::default()
                    },
                ),
                (
                    4,
                    Scalars {
                        sint64: 8,
                        fixed32: 9,
                        fixed64: 10,
                        ..Default::default()
                    },
                ),
            ]),
            nested: Some(Scalars {
                sfixed32: 11,
                sfixed64: 12,
                r#bool: true,
                string: "5".to_owned(),
                bytes: b"6".to_vec(),
                ..Default::default()
            }),
            my_enum: vec![0, 1, 2, 3],
        },
        ".test.ComplexType",
    );
}

#[test]
fn roundtrip_well_known_types() {
    roundtrip(
        &WellKnownTypes {
            timestamp: Some(prost_types::Timestamp {
                seconds: 63_108_020,
                nanos: 21_000_000,
            }),
            duration: Some(prost_types::Duration {
                seconds: 1,
                nanos: 340_012,
            }),
            r#struct: Some(prost_types::Struct {
                fields: BTreeMap::from([
                    (
                        "number".to_owned(),
                        prost_types::Value {
                            kind: Some(prost_types::value::Kind::NumberValue(42.)),
                        },
                    ),
                    (
                        "null".to_owned(),
                        prost_types::Value {
                            kind: Some(prost_types::value::Kind::NullValue(0)),
                        },
                    ),
                ]),
            }),
            float: Some(42.1),
            double: Some(12.4),
            int32: Some(1),
            int64: Some(-2),
            uint32: Some(3),
            uint64: Some(4),
            bool: Some(false),
            string: Some("hello".to_owned()),
            bytes: Some(b"hello".to_vec()),
            mask: Some(prost_types::FieldMask {
                paths: vec!["field_one".to_owned(), "field_two.b.d".to_owned()],
            }),
            list: Some(prost_types::ListValue {
                values: vec![
                    prost_types::Value {
                        kind: Some(prost_types::value::Kind::StringValue("foo".to_owned())),
                    },
                    prost_types::Value {
                        kind: Some(prost_types::value::Kind::BoolValue(false)),
                    },
                ],
            }),
            null: 0,
            empty: Some(()),
        },
        ".test.WellKnownTypes",
    );
}

fn roundtrip<T>(message: &T, message_name: &str)
where
    T: PartialEq + Debug + Message + Default,
{
    let prost_bytes = message.encode_to_vec();

    let mut dynamic_message = DynamicMessage::new(
        TEST_FILE_DESCRIPTOR
            .get_message_by_name(message_name)
            .expect("message not found"),
    );
    dynamic_message.merge(prost_bytes.as_slice()).unwrap();
    let dynamic_bytes = dynamic_message.encode_to_vec();

    let roundtripped_message = T::decode(dynamic_bytes.as_slice()).unwrap();
    assert_eq!(message, &roundtripped_message);
}
