use serde::de::{Deserialize, DeserializeOwned, Visitor};
use crate::io::csv::Reader;
use serde::Deserializer;
use serde::export::Formatter;

impl <'a,'de> Deserialize<'de> for Reader<'a>{
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de> {
        deserializer.deserialize_bool()
    }
}
// TODO