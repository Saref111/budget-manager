use rusqlite::{params, Connection, Result as DBResult};

use crate::types::{BudgetTransaction, PartialBudgetTransaction};

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

pub fn add_transaction(conn: &Connection, t: PartialBudgetTransaction, budget_id: u32) -> DBResult<()> {
    conn.execute("
        INSERT INTO transactions (budget_id, sum, message) VALUES (?1, ?2, ?3)
    ", params![budget_id, t.sum, t.message])?;

    Ok(())
}

pub fn update_transaction(conn: &Connection, updated_transaction: BudgetTransaction) ->  DBResult<()> {
    conn.execute(
        "UPDATE transactions SET sum=?2, message=?3 WHERE id=?1", 
        params![updated_transaction.id, updated_transaction.sum, updated_transaction.message]
    )?;

    Ok(())
}

pub fn remove_transaction(conn: &Connection, t_id: u32) -> DBResult<()> {
    conn.execute(
        "DELETE FROM transactions WHERE id=?1", 
        params![t_id]
    )?;

    Ok(())
}