/*
    Função OK
    Revisão OK
    Print OK
    -> Menu de ajuda sobre a utilização do sistema 
*/

pub(crate) fn help(){
    println!("Modo de uso: VwScan [Scan Type(s)] [Options] [target specification]\n");
    println!("Funções basicas:\n");
    println!("-h: Tela de ajuda");
    println!("-s: Captura de URLs sem subdomínio");
    println!("-sS: Captura de URLs com subdomínios");
    println!("-i: Separação das URLs por tipos de conteúdo");
    println!("-d: Captura de URLs via Dorks e armazenamento das mesmas");
    println!("-vA: Analise de possíveis URLs susetíveis a vulnerabilidades");
}