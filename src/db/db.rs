use rusqlite::{
    Connection, 
    Result as DBResult
};

pub fn init_db() -> DBResult<Connection> {
    let conn = Connection::open("budget-db.db")?;

    conn.execute("CREATE TABLE IF NOT EXISTS budgets (
        id INTEGER PRIMARY KEY,
        total INTEGER,
        name TEXT NOT NULL
    );", [])?;


    conn.execute("CREATE TABLE IF NOT EXISTS transactions (
        id INTEGER PRIMARY KEY,
        budget_id INTEGER NOT NULL,
        sum INTEGER,
        message TEXT NOT NULL,
        FOREIGN KEY (budget_id) REFERENCES budgets (id) ON DELETE CASCADE
    );", [])?;

    Ok(conn)
}