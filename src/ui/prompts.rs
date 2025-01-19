use dialoguer::Input;

use crate::types::{MinimalBudget, PartialBudgetTransaction};

pub fn prompt_for_new_transaction() -> PartialBudgetTransaction {
    let sum = Input::new()
        .with_prompt("Enter the transaction sum")
        .interact_text()
        .unwrap();

    let message = Input::new()
        .with_prompt("Enter the transaction message")
        .interact_text()
        .unwrap();

    PartialBudgetTransaction { sum, message }
}

pub fn prompt_for_new_budget() -> MinimalBudget {
    let name = Input::new()
        .with_prompt("Enter the budget name")
        .interact_text()
        .unwrap();

    let total: i32 = Input::new()
    .with_prompt("Enter the budget total amount of money")
    .interact_text()
    .unwrap();

    MinimalBudget { name, total }
}