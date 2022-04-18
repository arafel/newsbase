use rusqlite::{Connection, params, Result};

#[derive(Debug)]
struct Group {
    id: u32,
    name: String,
    low: u32,
    high: u32,
}

fn main() -> Result<()> {
    // let conn = Connection::open_in_memory()?;
    let conn = Connection::open("test.db")?;

    println!("Create table");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS newsgroups (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL UNIQUE,
            low   INTEGER NOT NULL,
            high  INTEGER NOT NULL
        )",
        [],
    )?;

    let group = Group {
        id: 0,
        name: "alt.sysadmin.recovery".to_string(),
        low: 0,
        high: 42,
    };
    println!("Insert group");
    conn.execute("INSERT INTO newsgroups (name, low, high) VALUES (?1, ?2, ?3)",
                 params![group.name, group.low, group.high])?;

    let mut stmt = conn.prepare("SELECT id, name, low, high FROM newsgroups")?;
    println!("Query for groups");
    let group_iter = stmt.query_map([], |row| {
        Ok(Group {
            id: row.get(0)?,
            name: row.get(1)?,
            low: row.get(2)?,
            high: row.get(3)?,
        })
    })?;

    for group in group_iter {
        println!("Found group {:?}", group.unwrap());
    }
    Ok(())
}