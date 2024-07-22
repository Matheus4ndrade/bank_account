use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Conta {
    nome: String,
    senha: String,
    saldo: f64,
}

impl Conta {
    fn new(nome: String, senha: String) -> Conta {
        Conta {
            nome,
            senha,
            saldo: 0.0,
        }
    }

    fn exibir_saldo(&self) {
        println!("O saldo da conta é: ${:.2}", self.saldo);
    }

    fn adicionar_fundos(&mut self, valor: f64) {
        self.saldo += valor;
        println!("${:.2} adicionados à conta.", valor);
    }

    fn transferir(&mut self, valor: f64) {
        if self.saldo >= valor {
            self.saldo -= valor;
            println!("${:.2} transferidos da conta.", valor);
        } else {
            println!("Saldo insuficiente para transferência.");
        }
    }
}

fn main() {
    let mut conta = carregar_ou_criar_conta();
    loop {
        exibir_menu();
        let escolha = obter_escolha();
        match escolha.as_str() {
            "1" => conta.exibir_saldo(),
            "2" => {
                let valor = obter_valor("Digite o valor para adicionar à conta: ");
                conta.adicionar_fundos(valor);
                salvar_conta(&conta);
            },
            "3" => {
                let valor = obter_valor("Digite o valor para transferir: ");
                conta.transferir(valor);
                salvar_conta(&conta);
            },
            "0" => {
                salvar_conta(&conta);
                break;
            },
            _ => println!("Escolha inválida, tente novamente."),
        }
    }
}

fn carregar_ou_criar_conta() -> Conta {
    let nome = obter_input("Digite seu nome: ");
    let senha = obter_input_escondido("Digite sua senha: ");
    
    if Path::new(&nome).exists() {
        let file = File::open(&nome).expect("Falha ao abrir arquivo de conta");
        let conta: Conta = serde_json::from_reader(file).expect("Falha ao ler dados da conta");
        if conta.senha == senha {
            println!("Conta carregada com sucesso.");
            conta
        } else {
            println!("Senha incorreta. Criando nova conta.");
            Conta::new(nome, senha)
        }
    } else {
        println!("Conta não encontrada. Criando nova conta.");
        Conta::new(nome, senha)
    }
}

fn salvar_conta(conta: &Conta) {
    let file = File::create(&conta.nome).expect("Falha ao criar arquivo de conta");
    serde_json::to_writer(file, &conta).expect("Falha ao salvar dados da conta");
}

fn exibir_menu() {
    println!("\nMenu:");
    println!("1. Ver saldo");
    println!("2. Adicionar à conta");
    println!("3. Transferir");
    println!("0. Sair");
}

fn obter_escolha() -> String {
    obter_input("Escolha uma opção: ")
}

fn obter_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha ao ler entrada");
    input.trim().to_string()
}

fn obter_input_escondido(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Falha ao ler entrada");
    input.trim().to_string()
}

fn obter_valor(prompt: &str) -> f64 {
    loop {
        let input = obter_input(prompt);
        match input.parse::<f64>() {
            Ok(valor) => return valor,
            Err(_) => println!("Valor inválido, tente novamente."),
        }
    }
}
