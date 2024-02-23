struct Permission;

impl Permission {
    /// Read permission for provided [role]
    fn read(role: &str) -> String {
        format!("read(\"{}\")", role)
    }

    /// Write permission for provided [role]
    ///
    /// This is an alias of update, delete, and possibly create.
    /// Don't use write in combination with update, delete, or create.
    fn write(role: &str) -> String {
        format!("write(\"{}\")", role)
    }

    /// Create permission for provided [role]
    fn create(role: &str) -> String {
        format!("create(\"{}\")", role)
    }

    /// Update permission for provided [role]
    fn update(role: &str) -> String {
        format!("update(\"{}\")", role)
    }

    /// Delete permission for provided [role]
    fn delete(role: &str) -> String {
        format!("delete(\"{}\")", role)
    }
}
