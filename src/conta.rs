use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Conta {
    pub nome: String,
    pub senha: String,
    pub saldo: f64,
}

impl Conta {
    pub fn new(nome: String, senha: String) -> Conta {
        Conta {
            nome,
            senha,
            saldo: 0.0,
        }
    }

    pub fn exibir_saldo(&self) {
        println!("O saldo da conta é: ${:.2}", self.saldo);
    }

    pub fn adicionar_fundos(&mut self, valor: f64) {
        self.saldo += valor;
        println!("${:.2} adicionados à conta.", valor);
    }

    pub fn transferir(&mut self, valor: f64, destinatario: &mut Conta) {
        if self.saldo >= valor {
            self.saldo -= valor;
            destinatario.saldo += valor;
            println!("Transferência de ${:.2} realizada com sucesso.", valor);
        } else {
            println!("Saldo insuficiente para transferência.");
        }
    }
}

pub fn carregar_contas() -> Vec<Conta> {
    if Path::new("contas.json").exists() {
        let file = File::open("contas.json").expect("Falha ao abrir arquivo de contas");
        serde_json::from_reader(file).expect("Falha ao ler dados de contas")
    } else {
        Vec::new()
    }
}

pub fn salvar_contas(contas: &Vec<Conta>) {
    let file = File::create("contas.json").expect("Falha ao criar arquivo de contas");
    serde_json::to_writer(file, contas).expect("Falha ao salvar dados de contas");
}
