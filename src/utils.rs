use std::io;

pub fn exibir_menu() {
    println!("\nMenu:");
    println!("1. Ver saldo");
    println!("2. Adicionar à conta");
    println!("3. Transferir");
    println!("0. Sair");
}

pub fn obter_escolha() -> String {
    obter_input("Escolha uma opção: ")
}

pub fn obter_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha ao ler entrada");
    input.trim().to_string()
}

pub fn obter_input_escondido(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha ao ler entrada");
    input.trim().to_string()
}

pub fn obter_valor(prompt: &str) -> f64 {
    loop {
        let input = obter_input(prompt);
        match input.parse::<f64>() {
            Ok(valor) => return valor,
            Err(_) => println!("Valor inválido, tente novamente."),
        }
    }
}
