use colorful::{Color, Colorful};
/*
    Função OK
    Revisão OK
    Print OK
    -> Banner do sistema 
*/
pub(crate) fn hi(){
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