use std::error::Error;

use db::{budget::add_budget, db::init_db};
use types::{App, Budget, BudgetTransaction};
use ui::ui::run;

mod types;
mod  ui;
mod db;

fn main() -> Result<(), Box<dyn Error>>{
    let conn = init_db()?;

    let mut app = App::new(conn);
    app.update()?;
    let ui = run(app);
    Ok(())
}
