use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub enum RelationshipType {
    #[default]
    #[serde(rename = "oneToOne")]
    OneToOne,
    #[serde(rename = "manyToOne")]
    ManyToOne,
    #[serde(rename = "manyToMany")]
    ManyToMany,
    #[serde(rename = "oneToMany")]
    OneToMany,
}
