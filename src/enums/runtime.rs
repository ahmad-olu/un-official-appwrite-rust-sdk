use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub enum Runtime {
    #[default]
    #[serde(rename = "node-14.5")]
    Node145,
    #[serde(rename = "node-16.0")]
    Node160,
    #[serde(rename = "node-18.0")]
    Node180,
    #[serde(rename = "node-19.0")]
    Node190,
    #[serde(rename = "node-20.0")]
    Node200,
    #[serde(rename = "node-21.0")]
    Node210,
    #[serde(rename = "php-8.0")]
    Php80,
    #[serde(rename = "php-8.1")]
    Php81,
    #[serde(rename = "php-8.2")]
    Php82,
    #[serde(rename = "php-8.3")]
    Php83,
    #[serde(rename = "ruby-3.0")]
    Ruby30,
    #[serde(rename = "ruby-3.1")]
    Ruby31,
    #[serde(rename = "ruby-3.2")]
    Ruby32,
    #[serde(rename = "ruby-3.3")]
    Ruby33,
    #[serde(rename = "python-3.8")]
    Python38,
    #[serde(rename = "python-3.9")]
    Python39,
    #[serde(rename = "python-3.10")]
    Python310,
    #[serde(rename = "python-3.11")]
    Python311,
    #[serde(rename = "python-3.12")]
    Python312,
    #[serde(rename = "deno-1.40")]
    Deno140,
    #[serde(rename = "dart-2.15")]
    Dart215,
    #[serde(rename = "dart-2.16")]
    Dart216,
    #[serde(rename = "dart-2.17")]
    Dart217,
    #[serde(rename = "dart-2.18")]
    Dart218,
    #[serde(rename = "dart-3.0")]
    Dart30,
    #[serde(rename = "dart-3.1")]
    Dart31,
    #[serde(rename = "dart-3.3")]
    Dart33,
    #[serde(rename = "dotnet-3.1")]
    Dotnet31,
    #[serde(rename = "dotnet-6.0")]
    Dotnet60,
    #[serde(rename = "dotnet-7.0")]
    Dotnet70,
    #[serde(rename = "java-8.0")]
    Java80,
    #[serde(rename = "java-11.0")]
    Java110,
    #[serde(rename = "java-17.0")]
    Java170,
    #[serde(rename = "java-18.0")]
    Java180,
    #[serde(rename = "java-21.0")]
    Java210,
    #[serde(rename = "swift-5.5")]
    Swift55,
    #[serde(rename = "swift-5.8")]
    Swift58,
    #[serde(rename = "swift-5.9")]
    Swift59,
    #[serde(rename = "kotlin-1.6")]
    Kotlin16,
    #[serde(rename = "kotlin-1.8")]
    Kotlin18,
    #[serde(rename = "kotlin-1.9")]
    Kotlin19,
    #[serde(rename = "cpp-17")]
    Cpp17,
    #[serde(rename = "cpp-20")]
    Cpp20,
    #[serde(rename = "bun-1.0")]
    Bun10,
}

impl Runtime {
    /// Converts the enum to its serialized string representation
    pub fn as_serialized(&self) -> String {
        serde_json::to_string(self)
            .unwrap_or_else(|_| panic!("Failed to serialize Runtime: {:?}", self))
            .trim_matches('"') // Remove surrounding quotes from the JSON string
            .to_string()
    }
}
