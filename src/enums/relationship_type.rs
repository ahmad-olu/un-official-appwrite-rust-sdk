use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum RelationshipType {
    #[serde(rename = "oneToOne")]
    OneToOne,
    #[serde(rename = "manyToOne")]
    ManyToOne,
    #[serde(rename = "manyToMany")]
    ManyToMany,
    #[serde(rename = "oneToMany")]
    OneToMany,
}
