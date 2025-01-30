use std::error::Error;

use db::db::init_db;
use types::App;
use ui::ui::run;

mod types;
mod  ui;
mod db;

fn main() -> Result<(), Box<dyn Error>>{
    let conn = init_db()?;

    let mut app = App::new(conn);
    app.update()?;
    let _ = run(app);
    Ok(())
}
