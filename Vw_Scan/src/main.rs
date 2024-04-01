//modularization
mod archives;
use archives::archive;

mod helps;
use helps::help;

mod banner;
use banner::hi;
//modularization end

//libraries

use std::env;
//libraries end

#[tokio::main]
async fn main() {
    hi();//baner VWSCAN
    let args: Vec<String> = env::args().collect();// Coletando os argumentos
    let quantidade_argumentos = args.len();// Quantidade de argumentos
    
    if quantidade_argumentos < 2 {//verifica se foi passado algum argumento
        println!("Modo de uso: vw_scan -Opções Site.com");
        return;
    }
      
    for argumento in args.iter().skip(1) {//leitura de argumentos passados pelo usuario
        match argumento.as_str() {
            "-h" => help(),//menu help 
            "-s" | "-sS" => { // Scan Simples ou Scan Composto
                let site: String = args[quantidade_argumentos - 1].clone();
                let is_simple = argumento == "-s"; // Verifica se é um scan simples
                if let Err(err) = archive(site, if is_simple { 1 } else { 0 }).await {
                    eprintln!("Erro: {}", err);
                }
            },
            "-e" => println!("Argumento -e foi passado."),
            _ => println!(""),//printa por causa do ultimo parametro CORRIGIR
        }
    }
}