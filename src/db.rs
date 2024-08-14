use rusqlite::{Connection, Result};

#[allow(dead_code)]
pub(crate) fn connect() -> Connection {
    Connection::open("kv.db").expect("Failed to open connection")
}

fn check_if_table_exists(conn: &Connection, name: &str) -> bool {
    let qry = format!("SELECT name FROM sqlite_master WHERE type='table' AND name='{}'", name);
    let mut prep_stmt = conn.prepare(&qry).expect("Failed to prepare statement");
    let mut stmt = prep_stmt.query([]).expect("Failed to execute query");
    if let Ok(None) = stmt.next() {
        false
    } else {
        true
    }
}

#[allow(dead_code)]
pub(crate) fn create_schema(conn: &Connection) {
    if !check_if_table_exists(conn, "kv") {
        conn.execute(
            "CREATE TABLE kv (
                id    INTEGER PRIMARY KEY,
                key  TEXT NOT NULL,
                data  TEXT
            )",
            (), // empty list of parameters.
        ).expect("Failed to create table: kv");
    }
}

#[allow(dead_code)]
pub(crate) fn put_data(conn: &Connection, key: &str, data: &str) -> Result<usize> {
    if get_data(conn, key).is_some() {
        conn.execute(
            "UPDATE kv SET data = ?1 WHERE key = ?2",
            &[&data, &key],
        )
    } else {
        conn.execute(
            "INSERT INTO kv (key, data) VALUES (?1, ?2)",
            &[&key, &data],
        )
    }
}

#[allow(dead_code)]
pub(crate) fn get_data(conn: &Connection, key: &str) -> Option<String> {
    let stmt = conn.prepare("SELECT data FROM kv WHERE key = ?1");
    if stmt.is_err() {
        return None;
    }
    let mut stmt = stmt.unwrap();
    let rows = stmt.query(&[&key]);
    if rows.is_err() {
        return None;
    }

    let mut rows = rows.unwrap();
    let row = rows.next();
    if row.is_err() {
        println!("Error: {:?}", row.err());
        return None;
    }

    let row = row.unwrap();
    if row.is_none() {
        return None;
    }
    let row = row?;
    let data: String = row.get(0).unwrap();
    Some(data)
}



#[cfg(test)]
mod tests {
    use rusqlite::{Connection};
    use super::*;

    #[test]
    fn test_db() {
        let conn = Connection::open("kv.db").unwrap();

        create_schema(&conn);

        put_data(&conn, "name", "{ 'hi': 'world' }").unwrap();
        put_data(&conn, "key1", "{ 'goodbye': 'cruel world' }").unwrap();

        let value = get_data(&conn, "name").unwrap();

        assert_eq!(value, "{ 'hi': 'world' }");

        conn.close().unwrap();
    }
}