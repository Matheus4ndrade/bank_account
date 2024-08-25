use crate::conta::Conta;
use crate::utils::{obter_input, obter_input_escondido};

pub fn login_ou_criar_conta(contas: &mut Vec<Conta>) -> usize {
    let nome = obter_input("Digite seu nome: ");
    let senha = obter_input_escondido("Digite sua senha: ");

    if let Some((idx, conta)) = contas.iter_mut().enumerate().find(|(_, c)| c.nome == nome) {
        if conta.verificar_senha(&senha) {
            println!("Login bem-sucedido.");
            return idx;
        } else {
            println!("Senha incorreta.");
            std::process::exit(1);
        }
    } else {
        println!("Conta não encontrada. Criando nova conta.");
        contas.push(Conta::new(nome.clone(), senha));
        contas.len() - 1
    }
}

impl Conta {
    pub fn verificar_senha(&self, senha: &str) -> bool {
        self.senha == senha
    }
}
