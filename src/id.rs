/// Helper struct to generate ID strings for resources.
pub struct ID;

impl ID {
    /// Have Appwrite generate a unique ID for you.
    pub fn unique() -> &'static str {
        "unique()"
    }

    /// Uses [id] as the ID for the resource.
    pub fn custom(id: &str) -> &str {
        id
    }
}
