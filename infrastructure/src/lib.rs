use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    SqliteConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tempfile::NamedTempFile;

    #[test]
    fn test_establish_connection_success() {
        // Create a temporary SQLite database file
        let tmp_file = NamedTempFile::new().unwrap();
        let tmp_path = tmp_file.path().to_str().unwrap();

        // Set the DATABASE_URL to our temporary file
        unsafe { env::set_var("DATABASE_URL", tmp_path); }

        // Test that we can establish a connection
        let mut connection = establish_connection();

        // Verify it's a valid connection by running a simple query
        let result = diesel::sql_query("SELECT 1").execute(&mut connection);
        assert!(result.is_ok(), "Should be able to execute query on valid connection");
    }
}