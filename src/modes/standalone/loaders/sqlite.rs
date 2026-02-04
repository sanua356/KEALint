use rusqlite::{Connection, Result};

use crate::checkers::Problem;

pub fn load_problems(config: String, problems: Vec<Problem>, connection: Connection) -> Result<()> {
    if problems.is_empty() {
        return Ok(());
    }

    let config_type = &problems.first().unwrap().config_type;

    let snapshot_id: u32 = connection.query_row(
        "INSERT INTO snapshot (config_type, data) VALUES (?1, ?2) RETURNING id",
        (config_type, &config),
        |row| row.get(0),
    )?;

    for problem in problems {
        connection.execute(
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
