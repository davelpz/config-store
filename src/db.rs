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

        conn.execute(
            "create index kv_key_index on kv (key);",
            (),
        ).expect("Failed to create index: idx_key");
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

pub(crate) fn delete_data(conn: &Connection, key: &str) -> Result<usize> {
    conn.execute(
        "DELETE FROM kv WHERE key = ?1",
        &[&key],
    )
}

pub(crate) fn get_all_keys(conn: &Connection) -> Vec<String> {
    let mut stmt = conn.prepare("SELECT key FROM kv").unwrap();
    let rows = stmt.query_map([], |row| {
        row.get(0)
    }).expect("Failed to query keys");
    rows.filter_map(Result::ok).collect()
}

pub(crate) fn count_keys(conn: &Connection) -> usize {
    let mut stmt = conn.prepare("SELECT count(*) FROM kv").unwrap();
    let count: usize = stmt.query_row([], |row| {
        row.get(0)
    }).expect("Failed to count keys");
    count
}

#[cfg(test)]
mod tests {
    use rusqlite::{Connection};
    use super::*;

    #[test]
    fn test_db() {
        let conn = Connection::open("kv.db").unwrap();

        create_schema(&conn);

        put_data(&conn, "dev/app1", "{ \"name\": \"eric\" }").unwrap();
        put_data(&conn, "dev/app2", "{ \"goodbye\": \"cruel world\" }").unwrap();

        let value = get_data(&conn, "dev/app1").unwrap();

        assert_eq!(value, "{ \"name\": \"eric\" }");

        conn.close().unwrap();
    }
}