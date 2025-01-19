use rusqlite::{params, Connection, Result as DBResult};

use crate::types::{Budget, PartialBudget};
use super::transaction::get_budget_transactions;

pub fn add_budget(conn: &Connection, b: Budget) -> DBResult<()> {
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