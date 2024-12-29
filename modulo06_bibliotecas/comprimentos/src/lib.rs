use anyhow::Error;
use std::io::{self, BufRead};

pub fn cumprimentar_usuario() -> Result<String, Error> {
    println!("Ola, Qual eh seu nome?");

    let stdin = io::stdin();
    let mut buffer = String::new();

    stdin.lock().read_line(&mut buffer)?;
    let buffer = buffer.trim().to_lowercase();

    if buffer != "bruno" {
        Err(Error::msg("Acesso Negado"))
    } else {
        Ok(buffer)
    }
}
