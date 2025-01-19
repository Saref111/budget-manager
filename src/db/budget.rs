use rusqlite::{params, Connection, Result as DBResult};

use crate::types::{Budget, PartialBudget, SavableBudget};
use super::transaction::get_budget_transactions;

pub fn add_budget(conn: &Connection, b: Box<dyn SavableBudget>) -> DBResult<()> {
    let b = b.prepare_for_db();
    conn.execute("
        INSERT INTO budgets (total, name) VALUES (?1, ?2)
    ", params![b.total, b.name])?;

    Ok(())
} 

pub fn get_all_budgets(conn: &Connection) -> DBResult<Vec<Budget>> {
    let mut stmt = conn.prepare("
        SELECT * FROM budgets
    ")?;

    let raw_result = stmt.query_map([], |row| {
        Ok(PartialBudget {
            id: row.get(0)?,
            total: row.get(1)?,
            name: row.get(3)?,
        } )
    })?;

    let result = raw_result.map(|b| {
        let partial_budget = b?; 

        let transaction = get_budget_transactions(conn, partial_budget.id)?;

        Ok(Budget {
            id: partial_budget.id,
            total: partial_budget.total,
            name: partial_budget.name,
            transactions: transaction,
        })
    }).collect();

    result
}

pub fn remove_budget(conn: &Connection, budget_id: u32) -> DBResult<()> {
    conn.execute("
        DELETE FROM budgets WHERE id=?
    ", params![budget_id])?;

    Ok(())
}

pub fn update_budget(conn: &Connection, b: Box<dyn SavableBudget>) -> DBResult<()> {
    let budget = b.get_without_transactions();

    conn.execute("
        UPDATE budgets SET total=?2, name=?3 WHERE id=?1
    ", params![budget.id, budget.total, budget.name])?;

    Ok(())
}