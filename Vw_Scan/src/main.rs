use reqwest;
use std::fs::File;
use std::io::Write;

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

#[tokio::main]
async fn main() {
    /*
        Criação do menu()
        passagem de parametro [site] ao archive()

        trabalhar com o archive.txt
        
     */
    if let Err(err) = archive().await {
        eprintln!("Erro: {}", err);
    }
}