pub struct Permission;

///Must be one of: any, guests, users, user, team, member, label.
impl Permission {
    /// Read permission for provided [role]
    pub fn read(role: &str) -> String {
        format!("read(\"{}\")", role)
    }

    /// Write permission for provided [role]
    ///
    /// This is an alias of update, delete, and possibly create.
    /// Don't use write in combination with update, delete, or create.
    pub fn write(role: &str) -> String {
        format!("write(\"{}\")", role)
    }

    /// Create permission for provided [role]
    pub fn create(role: &str) -> String {
        format!("create(\"{}\")", role)
    }

    /// Update permission for provided [role]
    pub fn update(role: &str) -> String {
        format!("update(\"{}\")", role)
    }

    /// Delete permission for provided [role]
    pub fn delete(role: &str) -> String {
        format!("delete(\"{}\")", role)
    }
}
