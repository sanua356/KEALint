use rusqlite::{Connection, OptionalExtension, Result};

fn get_schema_version(connection: &Connection) -> Result<Option<u32>> {
    connection
        .query_row("SELECT * FROM schema_version LIMIT 1", [], |row| row.get(0))
        .optional()
}

pub fn run_migrations(connection: Connection) -> Result<()> {
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
    #[allow(clippy::while_let_loop)]
    loop {
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
            _ => {
                break;
            }
        }
        schema_version = get_schema_version(&connection)?;
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use rusqlite::Connection;

    use super::run_migrations;

    #[test]
    fn apply_migrations_test() {
        let connection = Connection::open_in_memory().unwrap();

        let migrations = run_migrations(connection);

        assert_eq!(migrations, Ok(()));
    }
}
