use std::fs::File;
use std::io::{Error, Write};
use std::fs::read_to_string;
use std::collections::HashSet;

use reqwest::Identity;

/*
    -> busca na web por urls que podem ser ou possuir conteudo vulneravel
    -> leitura do arquivo dorks.txt
    https://www.exploit-db.com/google-hacking-database
*/

pub(crate) fn dorks() -> Result<(), Error> {
}