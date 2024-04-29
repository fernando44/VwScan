use std::fs::read_to_string;
use std::io::{Error, ErrorKind, Write};
use regex::Regex;
use std::fs::File;
use reqwest;

/*
    Função 
    Revisão 
    Print 
    verificar funcionalidade do codigo
*/
pub(crate) async fn dorks(site: &str) -> Result<(), Error> {
    println!("Função buscas relevantes na internet iniciada\n");
    // Lendo o conteúdo do arquivo dorks.txt
    let dorks = read_to_string("dorks.txt").expect("Could not read the file dorks.txt.");
    println!("\tCriando arquivo dorksUrls.txt");
    let mut dorks_file = File::create("dorksUrls.txt")?;
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
            let re_str = format!(r#"(http[^"]*{}[^"]*)""#, site);
            let re = Regex::new(&re_str).unwrap();
            for cap in re.captures_iter(&body_result) {
                writeln!(dorks_file, "{}", &cap[1])?;
            }
        } else {
            println!("Error: {}\nRequest failed with status: {}", dork, response.status());
        }
    }
    println!("\tDados Separados");
    println!("Função buscas relevantes na internet finalizada\n");
    Ok(())
}
