use reqwest;
use std::fs::File;
use std::io::Write;
use colorful::{Color, Colorful};
use std::env;

async fn archive() -> Result<(), Box<dyn std::error::Error>> {//função para enviar uma requisição GET a api do archive.org e retornar um arquivo de texto com o conteudo retornado pela api
    
    let url = "https://web.archive.org/cdx/search/cdx?url=*.meusite.com.br/*&output=json&fl=original&collapse=urlkey";//Url de pesquisa
    let response = reqwest::get(url).await?;//Requisição GET

    if response.status().is_success() {//response code 200 
        
        let body = response.text().await?;//response body
        let caminho_arquivo = "archive.txt";//cria o caminho do arquivo
        let mut arquivo = File::create(caminho_arquivo)?;//cria o arquivo
        arquivo.write_all(body.as_bytes())?;//escreve o conetudo 
        println!("Arquivo criado: {}", caminho_arquivo);

    } 
    
    else {
        println!("Erro na requisição. Código de status: {:?}", response.status());//mensagem de erro
    }

    Ok(())
}

fn hi(){
    let s = format!(
        "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
        r#"                                                                                                 "#,
        r#"--------------------------------------------------------------------------------------------"#,
        r#"░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░  ░▒▓███████▓▒░  ░▒▓██████▓▒░   ░▒▓██████▓▒░  ░▒▓███████▓▒░"#,
        r#"░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░        ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░"#,
        r#"░▒▓█▓▒▒▓█▓▒░  ░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░        ░▒▓█▓▒░        ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░"#,
        r#"░▒▓█▓▒▒▓█▓▒░  ░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░  ░▒▓██████▓▒░  ░▒▓█▓▒░        ░▒▓████████▓▒░ ░▒▓█▓▒░░▒▓█▓▒░"#,
        r#"░▒▓█▓▓█▓▒░   ░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░        ░▒▓█▓▒░ ░▒▓█▓▒░        ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░"#,
        r#"░▒▓█▓▓█▓▒░   ░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░        ░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░"#,
        r#" ░▒▓██▓▒░     ░▒▓█████████████▓▒░  ░▒▓███████▓▒░   ░▒▓██████▓▒░  ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░"#,
        r#"--------------------------------------------------------------------------------------------"#,
        r#"                                                                                                 "#,
    );
    println!("{}", s.gradient(Color::SkyBlue1).bold());
}

fn help(){
    println!("Modo de uso: VwScan [Scan Type(s)] [Options] [target specification]");
    println!("Funções basicas:");
    println!("-h: Tela de ajuda");
    println!("Modo de uso: VwScan [Scan Type(s)] [Options] [target specification]");
}

#[tokio::main]
async fn main() {
    hi();
    /*
        Criação do menu()
        passagem de parametro [site] ao archive()

        trabalhar com o archive.txt
        
     */

    // Coletando os argumentos da linha de comando
    let args: Vec<String> = env::args().collect();

    //println!("{}",args[0]);//nome do programa
    //println!("{}",args[1]);//primeiro argumento

    // Verificando se foi passado o nome do programa como argumento
    if args.len() < 2 {
        println!("Uso: {} <NomeDoPrograma>", args[0]);
        return;
    }

    help();
    if let Err(err) = archive().await {
        eprintln!("Erro: {}", err);
    }
}