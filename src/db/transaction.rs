use rusqlite::{params, Connection, Result as DBResult};

use crate::types::BudgetTransaction;

pub fn get_budget_transactions(conn: &Connection, budget_id: u32) -> DBResult<Vec<BudgetTransaction>> {
    let mut stmt = conn.prepare("
        SELECT * FROM transactions WHERE budget_id=?
    ")?;

    let raw_transactions = stmt.query_map(params![budget_id], |row| {
        Ok(
            BudgetTransaction {
                id: row.get(0)?,
                sum: row.get(2)?,
                message: row.get(3)?,
            }
        )
    })?;

    Ok(raw_transactions.map(|t| t.unwrap()).collect())
}