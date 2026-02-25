pub mod common_objects;
pub mod error;
pub mod http;

pub mod value_objects;
pub use value_objects::{
    address::{InterfaceAddress, InterfaceAddressBuilder, InterfaceAddressess},
    diff::InterfaceDiff,
    password::{InterfaceHashedPassword, InterfaceNoneHashedPassword, InterfacePassword},
    phone_number::{InterfacePhoneNumber, InterfacePhoneNumbers},
};
