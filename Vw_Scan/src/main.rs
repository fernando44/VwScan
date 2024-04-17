// Modularização
mod archives;
use archives::archive;

mod helps;
use helps::help;

mod banner;
use banner::hi;

mod identfier;
use identfier::identf;

mod vulns;
use vulns::vulns;

mod dorks;
use dorks::dorks;
// Fim da modularização

// Bibliotecas
use std::env;
// Fim das bibliotecas

/*
    -> Menu principal direciona o usuário para as diferentes funções 
*/

#[tokio::main]
async fn main() {
    hi(); // Banner VWSCAN
    let args: Vec<String> = env::args().collect(); // Coletando os argumentos
    let quantidade_argumentos = args.len(); // Quantidade de argumentos
    
    if quantidade_argumentos < 2 { // Verifica se foi passado algum argumento
        println!("Modo de uso: vw_scan -[Opções] Site.com");
        return;
    }
      
    for argumento in args.iter().skip(1) { // Leitura de argumentos passados pelo usuário
        match argumento.as_str() {
            "-h" => help(), // Menu help 
            "-s" | "-sS" => { // Scan Simples ou Scan Composto
                let site: String = args[quantidade_argumentos - 1].clone();
                let is_simple = argumento == "-s"; // Verifica se é um scan simples
                if let Err(err) = archive(site, if is_simple { 1 } else { 0 }).await {
                    eprintln!("Erro: {}", err);
                }
            },
            "-i" => {
                if let Err(err) = identf() {
                    eprintln!("Erro ao processar identf(): {}", err);
                }
            },
            "-d" => {
                let site: String = args[quantidade_argumentos - 1].clone();
                if let Err(err) = dorks(&site).await {
                    eprintln!("Erro ao processar dorks(): {}", err);
                }
            },
            "-vA" => {
                if let Err(err) = vulns() {
                    eprintln!("Erro ao processar vulns(): {}", err);
                }
            },
            _ => println!(""), // Printa por causa do último parâmetro CORRIGIR
        }
    }
}
