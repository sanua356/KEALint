use rusqlite::{Connection, OptionalExtension, Result};

use crate::checkers::Problem;

use super::types::{DatabaseResults, DatabaseResultsChecker};

fn get_schema_version(connection: &Connection) -> Result<Option<u32>> {
    connection
        .query_row("SELECT * FROM schema_version LIMIT 1", [], |row| row.get(0))
        .optional()
}

impl DatabaseResultsChecker<Result<()>> for DatabaseResults<Connection> {
    fn run_migrations(&self) -> Result<()> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS schema_version (version INTEGER NOT NULL)",
            (),
        )?;

        let mut schema_version: Option<u32> = get_schema_version(&self.connection)?;

        if schema_version.is_none() {
            self.connection
                .execute("INSERT INTO schema_version (version) VALUES (1)", ())?;
        }

        schema_version = get_schema_version(&self.connection)?;

        // The warning is disabled because there can be many schemas versions.
        #[allow(clippy::while_let_loop)]
        loop {
            #[allow(clippy::single_match)]
            match schema_version.unwrap_or(1) {
                1 => {
                    self.connection.execute(
                        "CREATE TABLE IF NOT EXISTS snapshot (
			            id INTEGER PRIMARY KEY,
			            config_type	TEXT NOT NULL,
			            data TEXT NOT NULL,
						created_at DATETIME DEFAULT (datetime('now', 'localtime'))
                )",
                        (),
                    )?;

                    self.connection.execute(
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

                    self.connection
                        .execute("UPDATE schema_version SET version = 2", ())?;
                }
                _ => {
                    break;
                }
            }
            schema_version = get_schema_version(&self.connection)?;
        }

        Ok(())
    }

    fn load_results(&self, config: String, problems: Vec<Problem>) -> Result<()> {
        if problems.is_empty() {
            return Ok(());
        }

        let config_type = &problems.first().unwrap().config_type;

        let snapshot_id: u32 = self.connection.query_row(
            "INSERT INTO snapshot (config_type, data) VALUES (?1, ?2) RETURNING id",
            (config_type, &config),
            |row| row.get(0),
        )?;

        for problem in problems {
            self.connection.execute(
                "INSERT INTO problem (snapshot_id, name, importance, description, places, links) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                (
                snapshot_id,
                problem.name,
                problem.importance,
                problem.description,
                serde_json::to_string(&problem.places.unwrap_or_default()).unwrap(),
                serde_json::to_string(&problem.links.unwrap_or_default()).unwrap(),
                ),
            )?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use rusqlite::Connection;

    use crate::checkers::Problem;

    use super::{DatabaseResults, DatabaseResultsChecker};

    #[test]
    fn apply_migrations_test() {
        let connection = Connection::open_in_memory().unwrap();

        let database: DatabaseResults<Connection> = DatabaseResults { connection };
        let migrations = database.run_migrations();

        assert_eq!(migrations, Ok(()));
    }

    #[test]
    fn load_results_test() {
        let connection = Connection::open_in_memory().unwrap();

        let database: DatabaseResults<Connection> = DatabaseResults { connection };
        database.run_migrations().unwrap();
        let results = database.load_results(
            "Dhcp4".to_string(),
            vec![Problem {
                config_type: "Dhcp4".to_string(),
                description: "Description".to_string(),
                importance: "WARNING".to_string(),
                links: None,
                name: "NAME".to_string(),
                places: None,
            }],
        );

        assert_eq!(results, Ok(()));
    }
}
