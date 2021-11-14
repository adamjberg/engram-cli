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

      let trimmed_body = buffer.trim();
      let split_whitespace = trimmed_body.split_whitespace();
      if trimmed_body == "/list" {
        let mut stmt = conn.prepare("SELECT id, body from notes")?;
        let mut rows = stmt.query(rusqlite::params![])?;
        while let Some(row) = rows.next()? {
          let id: i32 = row.get(0)?;
          let body: String = row.get(1)?;
          println!("{} {}", id, body.to_string());
        }
      } else if trimmed_body.contains("/del") {
        let id = split_whitespace.last();
        conn.execute("delete from notes where id = (?1)", [id])?;
      }
      else if trimmed_body == "" {
        running = false;
      } else {
        conn.execute("INSERT INTO notes (body) values (?1)", [trimmed_body])?;
      }
    }

    Ok(())
}