use std::fs::File;
use std::io::{Error, Write};
use std::fs::read_to_string;

/*
    Função OK
    Revisão OK
    Print OK
    -> Busca por vulnerabilidades, coleta urls susetiveis a vulnerabilidades

*/

pub(crate) fn vulns() -> Result<(), Error> {
    println!("Função URLs susetíveis a vulnerabilidades iniciada\n");
    let url_content = read_to_string("archive.txt").expect("\nIt was not possible to read the file archive.txt.\nUse any URL scan of your choice (rename the file) or run vwscan -s or -sS <target>.");
    
    let mut vuln = File::create("vulns.txt")?;
    println!("\tCriando arquivo vulns.txt");

    for line in url_content.lines() {
        if line.contains("=") {
            writeln!(vuln, "{}", line)?;
        }
    }
    println!("\tDados Separados");
    println!("\nFunção URLs susetíveis a vulnerabilidades finalizada\n");
    Ok(())
}