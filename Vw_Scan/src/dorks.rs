use std::fs::read_to_string;
use std::io::{Error, ErrorKind};
use regex::Regex;
use reqwest;

/*
    Função 
    Revisão 
    Print 
    verificar funcionalidade do codigo
*/
pub(crate) async fn dorks(site: &str) -> Result<(), Error> {
    // Lendo o conteúdo do arquivo dorks.txt
    let dorks = read_to_string("dorks.txt").expect("Could not read the file dorks.txt.");

    for dork in dorks.lines() {
        // Construindo a URL de busca com o dork e o site
        let url = format!("https://www.google.com/search?q={}+site:{}", dork, site);
        
        // Realizando a requisição HTTP para a URL construída
        let response = match reqwest::get(&url).await {
            Ok(response) => response,
            Err(err) => {
                return Err(Error::new(ErrorKind::Other, err));
            }
        };

        // Verificando se a requisição foi bem-sucedida (status 200 OK)
        if response.status().is_success() {
            // Lendo o corpo da resposta como texto
            let body_result = response.text().await.map_err(|err| Error::new(ErrorKind::Other, err))?;
            
            let re = Regex::new(r#"(http[^"]*meusite[^"]*)""#).unwrap();//r#"href="([^"]*meusite\.br[^"]*)""#
            
            for cap in re.captures_iter(&body_result) {
                println!("Link encontrado: {}", &cap[1]);
            }
        } else {
            println!("Dork: {}\nRequest failed with status: {}", dork, response.status());
        }
    }

    Ok(())
}
