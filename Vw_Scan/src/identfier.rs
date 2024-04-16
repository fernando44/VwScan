use std::fs::File;
use std::io::{Error, Write};
use std::fs::read_to_string;
use std::collections::HashSet;

use reqwest::Identity;

/*
    -> Leitura do conteudo do archive.txt
    -> Leitura dos identificadores idf..
    -> Criação dos arquivos de textos finais
    -> Criação dos conjuntos para armazenar as strings de comparação
    -> Separa as URLs segundo sua extensão e salva nos arquivos corretos


    ###
    Leitura de apenas 1 arquivo contento todudo a ser utilizado na pesquisa 
    criar um delimitador para cada tipo de arquivo 
*/

pub(crate) fn identf() -> Result<(), Error> {

    // Ler o conteúdo do arquivo archive.txt
    let url_content = read_to_string("archive.txt").expect("\nIt was not possible to read the file archive.txt.\nUse any URL scan of your choice (rename the file) or run vwscan -s or -sS <target>.");

    let identfier:String = read_to_string("identifier.txt").expect("\nIt was not possible to read the file identifier.txt.\nCreate a new identifier.txt, look on the github the model");

    //criação dos arquivos
    let mut images_file = File::create("images.txt")?;
    let mut dev_file = File::create("dev.txt")?;
    let mut files_file = File::create("files.txt")?;
    let mut data_file = File::create("data.txt")?;
    let mut extra_file = File::create("extra.txt")?;
    
    //criação dos auxiliares
    let mut images: Vec<String> = Vec::new();
    let mut dev: Vec<String> = Vec::new();
    let mut files: Vec<String> = Vec::new();
    let mut data: Vec<String> = Vec::new();
    let mut extra: Vec<String> = Vec::new();

    let mut controle = 0;//auxiliar do for ident

    //percorre o arquivo de identificadores e salva em seus respectivos vetores
    for ident in identfier.lines() {
        if ident.contains("#images") {
            controle = 1;
        } else if ident.contains("#dev") {
            controle = 2;
        } else if ident.contains("#files") {
            controle = 3;
        } else if ident.contains("#data") {
            controle = 4;
        } else if ident.contains("#extra") {
            controle = 5;
        }
    
        match controle {
            1 if ident != "#images" => images.push(String::from(ident)),
            2 if ident != "#dev" => dev.push(String::from(ident)),
            3 if ident != "#files" => files.push(String::from(ident)),
            4 if ident != "#data" => data.push(String::from(ident)),
            5 if ident != "#extra" => extra.push(String::from(ident)),
            _ => {} // Caso controle não corresponda a nenhum caso, não faz nada
        }
    }
    println!("identificadores separados");

    //Busca e separação 
    for line in url_content.lines() {
        for item in &images {//bloco De Verificação idfImage.txt
            if line.contains(item) {
                writeln!(images_file, "{}", line)?;
            }
        }

        for item in &dev {//bloco De Verificação idfDev.txt
            if line.contains(item) {
                writeln!(dev_file, "{}", line)?;
            }
        }

        for item in &files {//bloco De Verificação idfFiles.txt
            if line.contains(item) {
                writeln!(files_file, "{}", line)?;
            }
        }

        for item in &data {//bloco De Verificação idfData.txt
            if line.contains(item) {
                writeln!(data_file, "{}", line)?;
            }
        }

        for item in &extra {//bloco De Verificação idfExtra.txt
            if line.contains(item) {
                writeln!(extra_file, "{}", line)?;
            }
        }
    }




    /*bloco antigo
    //leitura dos identificadores
    let idf_image_content = read_to_string("idfImage.txt").expect("\nIt was not possible to read the file idfImage.txt.\nCreate it and add the extensions of your preference.");
    let idf_dev_content = read_to_string("idfDev.txt").expect("\nIt was not possible to read the file idfDev.txt.\nCreate it and add the extensions of your preference.");
    let idf_files_content = read_to_string("idfFiles.txt").expect("\nIt was not possible to read the file idfFiles.txt.\nCreate it and add the extensions of your preference.");
    let idf_data_content = read_to_string("idfData.txt").expect("\nIt was not possible to read the file idfData.txt.\nCreate it and add the extensions of your preference.");
    let idf_extra_content = read_to_string("idfExtra.txt").expect("\nIt was not possible to read the file idfExtra.txt.\nCreate it and add the extensions of your preference.");

    //criação dos arquivos
    let mut images_file = File::create("images.txt")?;
    let mut dev_file = File::create("dev.txt")?;
    let mut files_file = File::create("files.txt")?;
    let mut data_file = File::create("data.txt")?;
    let mut extra_file = File::create("extra.txt")?;
    
    //criação de conjuntos para armazenar as strings de comparação
    let idf_images_set: HashSet<String> = idf_image_content.lines().map(|s| s.to_string()).collect();
    let idf_dev_set: HashSet<String> = idf_dev_content.lines().map(|s| s.to_string()).collect();

    let idf_files_set: HashSet<String> = idf_files_content.lines().map(|s| s.to_string()).collect();
    let idf_data_set: HashSet<String> = idf_data_content.lines().map(|s| s.to_string()).collect();
    let idf_extra_set: HashSet<String> = idf_extra_content.lines().map(|s| s.to_string()).collect();
    
    //Busca e separação 
    for line in url_content.lines() {
        for item in &idf_images_set {//bloco De Verificação idfImage.txt
            if line.contains(item) {
                writeln!(images_file, "{}", line)?;
            }
        }

        for item in &idf_dev_set {//bloco De Verificação idfDev.txt
            if line.contains(item) {
                writeln!(dev_file, "{}", line)?;
            }
        }

        for item in &idf_files_set {//bloco De Verificação idfFiles.txt
            if line.contains(item) {
                writeln!(files_file, "{}", line)?;
            }
        }

        for item in &idf_data_set {//bloco De Verificação idfData.txt
            if line.contains(item) {
                writeln!(data_file, "{}", line)?;
            }
        }

        for item in &idf_extra_set {//bloco De Verificação idfExtra.txt
            if line.contains(item) {
                writeln!(extra_file, "{}", line)?;
            }
        }
    }
*/
    Ok(())
}