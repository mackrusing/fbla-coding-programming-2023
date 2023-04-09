// use serde::{Serialize, Serializer};
// #[macro_use] extern crate rocket;
use serde_repr::{Serialize_repr, Deserialize_repr};
// use rocket::form::FromFormValue;

#[derive(Debug, PartialEq, Clone)]
#[derive(Serialize_repr, Deserialize_repr)]
#[derive(rocket::FromFormField)]
#[repr(u8)]
pub enum GradeLvl {
    #[field(value = "9")]
    Nine = 9,
    #[field(value = "10")]
    Ten = 10,
    #[field(value = "11")]
    Eleven = 11,
    #[field(value = "12")]
    Twelve = 12,
}
