use chrono::Utc;
use rand::{distributions::Alphanumeric, Rng};

/// Helper struct to generate ID strings for resources.
pub struct ID;

impl ID {
    /// Generate a hexadecimal ID based on the current timestamp.
    fn hex_timestamp() -> String {
        let now = Utc::now();
        let sec = now.timestamp();
        let usec = now.timestamp_subsec_micros();
        format!("{:x}{:05x}", sec, usec)
    }

    /// Generate a unique ID with padding to create a longer ID.
    pub fn unique(padding: usize) -> String {
        let mut id = Self::hex_timestamp();

        if padding > 0 {
            let mut rng = rand::thread_rng();
            let padding_string: String = (0..padding)
                .map(|_| rng.sample(Alphanumeric))
                .filter(|c| c.is_ascii_alphanumeric()) // Ensure valid characters
                .map(|c| c as char)
                .collect();
            id.push_str(&padding_string);
        }

        // Truncate to 36 characters if needed
        id.chars().take(36).collect()
    }

    /// Have Appwrite generate a unique ID for you.
    pub fn unique_old() -> &'static str {
        "unique()"
    }

    /// Uses [id] as the ID for the resource.
    pub fn custom(id: &str) -> &str {
        id
    }
}
