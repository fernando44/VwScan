use reqwest;
use std::fs::File;
use std::io::Write;
/*
    -> Função para enviar uma requisição GET a api do archive.org
        * Retorna um arquivo de texto com o conteudo solicitado a api
        * Arquivo no formato JSON
        * Arquivo criado archiveOriginal.txt

    -> Leitura e ajuste do JSON
        * Arquivo criado archive.txt
*/

pub(crate) async fn archive(site: String, control: u8) -> Result<(), Box<dyn std::error::Error>> {
    let url = if control == 1 {//verifica o tipo de busca selecionado
        format!("https://web.archive.org/cdx/search/cdx?url={}/*&output=json&fl=original&collapse=urlkey", site)//busca sem subdomínio
    } else {
        format!("https://web.archive.org/cdx/search/cdx?url=*.{}/*&output=json&fl=original&collapse=urlkey", site)//busca com subdomínio
    };
    
    let response = reqwest::get(url).await?;//Requisição GET

    if response.status().is_success() {//response code 200 
        
        println!("Requisição recebida\nSeparando em 2 arquivos, JSON original e URLs limpas");
        let body = response.text().await?;//response body
        let caminho_arquivo = "archiveOriginal.txt";//cria o caminho do arquivo
        let mut arquivo = File::create(caminho_arquivo)?;//cria o arquivo
        arquivo.write_all(body.as_bytes())?;//escreve o conetudo 
        println!("Arquivo original criado: {}", caminho_arquivo);

        let mut final_str = String::new();

        for line in body.lines() {//Cortar os delimitadores do JSON e selecionar apenas o que contenha http
            if let Some(start) = line.find("[\"") {
                if let Some(end) = line.find("\"]") {
                    let url = &line[start + 2..end];
                    if url.starts_with("http") {
                        final_str.push_str(&format!("{}\n", url));//salva a url limpa
                    }
                }
            }
        }

        let nome = "archive.txt";//Criação do arquivo
        let mut arq = File::create(nome)?;
        arq.write_all(final_str.as_bytes())?;//escrita no arquivo
        println!("Arquivo limpo criado: {}", nome);
        
    }
    else {
        println!("Erro na requisição. Código de status: {:?}", response.status());//mensagem de erro
    }

    Ok(())
}