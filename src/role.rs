/// Helper struct to generate role strings for [Permission].
struct Role;

impl Role {
    /// Grants access to anyone.
    ///
    /// This includes authenticated and unauthenticated users.
    fn any() -> String {
        "any".to_string()
    }

    /// Grants access to a specific user by user ID.
    ///
    /// You can optionally pass verified or unverified for
    /// [status] to target specific types of users.
    fn user(id: &str, status: Option<String>) -> String {
        if status.is_none() {
            return format!("user:{}", id);
        }
        format!("user:{}/{}", id, status.unwrap())
    }

    /// Grants access to any authenticated or anonymous user.
    ///
    /// You can optionally pass verified or unverified for
    /// [status] to target specific types of users.
    fn users(status: Option<String>) -> String {
        if status.is_none() {
            return String::from("users");
        }
        format!("users/{}", status.unwrap())
    }

    /// Grants access to any guest user without a session.
    ///
    /// Authenticated users don't have access to this role.
    fn guests() -> String {
        "guests".to_string()
    }

    /// Grants access to a team by team ID.
    ///
    /// You can optionally pass a role for [role] to target
    /// team members with the specified role.
    fn team(id: &str, role: Option<String>) -> String {
        if role.is_none() {
            return format!("team:{}", id);
        }
        format!("team:{}/{}", id, role.unwrap())
    }

    /// Grants access to a specific member of a team.
    ///
    /// When the member is removed from the team, they will
    /// no longer have access.
    fn member(id: &str) -> String {
        format!("member:{}", id)
    }

    /// Grants access to a user with the specified label.
    fn label(name: &str) -> String {
        format!("label:{}", name)
    }
}
