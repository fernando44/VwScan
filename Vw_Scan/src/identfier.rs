use std::fs::File;
use std::io::{Error, Write};
use std::fs::read_to_string;
use std::collections::HashSet;

pub(crate) fn identf() -> Result<(), Error> {

    // Ler o conteúdo dos arquivos comp e url
    let url_content = read_to_string("archive.txt").expect("Não foi possível ler o arquivo archive.txt");

    //leitura dos identificadores
    let idf_image_content = read_to_string("idfImage.txt").expect("Não foi possível ler o arquivo idfImage");
    let idf_dev_content = read_to_string("idfDev.txt").expect("Não foi possível ler o arquivo idfImage");
    let idf_files_content = read_to_string("idfFiles.txt").expect("Não foi possível ler o arquivo idfImage");
    let idf_data_content = read_to_string("idfData.txt").expect("Não foi possível ler o arquivo idfImage");
    let idf_extra_content = read_to_string("idfExtra.txt").expect("Não foi possível ler o arquivo idfImage");

    //criação dos arquivos
    let mut images_file = File::create("images.txt")?;
    let mut dev_file = File::create("dev.txt")?;
    let mut files_file = File::create("files.txt")?;
    let mut data_file = File::create("data.txt")?;
    let mut extra_file = File::create("extra.txt")?;
    // Criar um conjunto para armazenar as strings de comp
    let idf_images_set: HashSet<String> = idf_image_content.lines().map(|s| s.to_string()).collect();
    let idf_dev_set: HashSet<String> = idf_dev_content.lines().map(|s| s.to_string()).collect();

    let idf_files_set: HashSet<String> = idf_files_content.lines().map(|s| s.to_string()).collect();
    let idf_data_set: HashSet<String> = idf_data_content.lines().map(|s| s.to_string()).collect();
    let idf_extra_set: HashSet<String> = idf_extra_content.lines().map(|s| s.to_string()).collect();
    
    // Iterar sobre cada linha do arquivo url e verificar se está no conjunto de comp
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

    Ok(())
}