use rusqlite::{Connection, OptionalExtension, Result};

fn get_schema_version(connection: &Connection) -> Result<Option<u32>> {
    connection
        .query_row("SELECT * FROM schema_version LIMIT 1", [], |row| row.get(0))
        .optional()
}

pub fn run_migrations(database_path: String) -> Result<()> {
    let connection = Connection::open(database_path)
        .expect("An error occurred when connecting to an SQLite database.");

    connection.execute(
        "CREATE TABLE IF NOT EXISTS schema_version (version INTEGER NOT NULL)",
        (),
    )?;

    let mut schema_version: Option<u32> = get_schema_version(&connection)?;

    if schema_version.is_none() {
        connection.execute("INSERT INTO schema_version (version) VALUES (1)", ())?;
    }

    schema_version = get_schema_version(&connection)?;

    // The warning is disabled because there can be many schemas versions.
    #[allow(clippy::single_match)]
    match schema_version.unwrap_or(1) {
        1 => {
            connection.execute(
                "CREATE TABLE IF NOT EXISTS snapshot (
		            id INTEGER PRIMARY KEY,
		            config_type	TEXT NOT NULL,
		            data TEXT NOT NULL,
					created_at DATETIME DEFAULT (datetime('now', 'localtime'))
                )",
                (),
            )?;

            connection.execute(
                "CREATE TABLE IF NOT EXISTS problem (
		            id INTEGER PRIMARY KEY,
		            snapshot_id	INTEGER NOT NULL,
		            name TEXT NOT NULL,
					importance TEXT NOT NULL,
					description TEXT NOT NULL,
					places TEXT,
					links TEXT,
					FOREIGN KEY (snapshot_id) REFERENCES snapshot (id)
             )",
                (),
            )?;

            connection.execute("UPDATE schema_version SET version = 2", ())?;
        }
        _ => {}
    }

    Ok(())
}
