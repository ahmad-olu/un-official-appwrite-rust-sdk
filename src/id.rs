/// Helper struct to generate ID strings for resources.
struct ID;

impl ID {
    /// Have Appwrite generate a unique ID for you.
    fn unique() -> String {
        String::from("unique()")
    }

    /// Uses [id] as the ID for the resource.
    fn custom(id: &str) -> String {
        id.to_string()
    }
}
