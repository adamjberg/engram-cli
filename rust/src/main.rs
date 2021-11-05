use std::io;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut username = String::new();
    let stdin = io::stdin();
    println!("Username?");
    stdin.read_line(&mut username)?;

    let mut password = String::new();
    println!("Password?");
    stdin.read_line(&mut password)?;
    
    let mut login_post_map = HashMap::new();
    login_post_map.insert("username", username.trim());
    login_post_map.insert("password", password.trim());

    let client = reqwest::Client::builder()
        .cookie_store(true).build().unwrap();
    let resp = client.post("https://engram.xyzdigital.com/api/users/login")
        .json(&login_post_map)
        .send()
        .await?;
    if resp.status() != 200 {
      println!("failed to log in");
      return Ok(());
    }

    let mut running = true;
    while running == true {
      let mut buffer = String::new();
      stdin.read_line(&mut buffer)?;
      if buffer.trim() == "" {
        running = false;
      } else {
        let mut note_post_map = HashMap::new();
        note_post_map.insert("date", "2021-11-04");
        note_post_map.insert("body", &buffer);

        let create_note_res = client.post("https://engram.xyzdigital.com/api/notes")
            .json(&note_post_map)
            .send()
            .await?;

        if create_note_res.status() != 200 {
          println!("failed to save note");
        }
      }
    }
    Ok(())
}
