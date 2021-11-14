use std::io;
use rusqlite::{Connection, Result};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("engram.db")?;

    conn.execute(
        "create table if not exists notes (
             id integer primary key,
             body text not null unique
         )",
        [],
    )?;

    let mut running = true;
    while running == true {
    let mut buffer = String::new();
      io::stdin().read_line(&mut buffer)?;
      if buffer.trim() == "" {
        running = false;
      } else {
        conn.execute("INSERT INTO notes (body) values (?1)", [buffer]);
      }
    }

    Ok(())
}