use serde::{Deserialize, Serialize,Deserializer};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    #[serde(rename="person_id")]
    pub id:i32,
    pub isim: String,
    pub soyisim: String,
    pub email: String,
    pub yas: i32,
}
//#[serde(rename = "_id")]
//pub id: Option<ObjectId>,
#[derive(Deserialize)]
pub struct UpdatePerson {
    pub isim: String,
    pub soyisim: String,
    pub email: String,
    pub yas: i32,
}

#[derive(Deserialize)]
pub struct NewPerson {
    pub id: Option<i32>, //i32 yapmayı dene  string ti
    pub isim: String,
    pub soyisim: String,
    pub email: String,
    #[serde(deserialize_with = "deserialize_age")]
    pub yas: i32,
}
fn deserialize_age<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse().map_err(serde::de::Error::custom)
}
