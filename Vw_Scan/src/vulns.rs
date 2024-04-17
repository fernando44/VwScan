use std::fs::File;
use std::io::{Error, Write};
use std::fs::read_to_string;

/*
    -> Busca por vulnerabilidades, coleta urls susetiveis a vulnerabilidades

*/

pub(crate) fn vulns() -> Result<(), Error> {
    let url_content = read_to_string("archive.txt").expect("\nIt was not possible to read the file archive.txt.\nUse any URL scan of your choice (rename the file) or run vwscan -s or -sS <target>.");
    
    let mut vuln = File::create("vulns.txt")?;
    println!("Criando arquivo vulns.txt");

    for line in url_content.lines() {
        match line {
           "=" => writeln!(vuln, "{}", line)?,
            _ => {} // Caso controle não corresponda a nenhum caso, não faz nada
        } 
    }
    Ok(())
}