mod conta;
mod autenticacao;
mod utils;

use crate::conta::{Conta, carregar_contas, salvar_contas};
use crate::autenticacao::login_ou_criar_conta;
use crate::utils::{exibir_menu, obter_escolha, obter_valor, obter_input};

fn main() {
    let mut contas = carregar_contas();

    let conta_atual_idx = login_ou_criar_conta(&mut contas);

    loop {
        exibir_menu();
        let escolha = obter_escolha();
        match escolha.as_str() {
            "1" => contas[conta_atual_idx].exibir_saldo(),
            "2" => {
                let valor = obter_valor("Digite o valor para adicionar à conta: ");
                contas[conta_atual_idx].adicionar_fundos(valor);
                salvar_contas(&contas);
            },
            "3" => {
                let destinatario_nome = obter_input("Digite o nome da conta de destino: ");
                if let Some(destinatario_idx) = contas.iter().position(|c| c.nome == destinatario_nome) {
                    let valor = obter_valor("Digite o valor para transferir: ");

                    if conta_atual_idx == destinatario_idx {
                        println!("Não é possível transferir para a própria conta.");
                    } else {
                        if conta_atual_idx < destinatario_idx {
                            let (parte_antes, parte_depois) = contas.split_at_mut(destinatario_idx);
                            parte_antes[conta_atual_idx].transferir(valor, &mut parte_depois[0]);
                        } else {
                            let (parte_antes, parte_depois) = contas.split_at_mut(conta_atual_idx);
                            parte_depois[0].transferir(valor, &mut parte_antes[destinatario_idx]);
                        }
                    }
                    salvar_contas(&contas);
                } else {
                    println!("Conta de destino não encontrada.");
                }
            },
            "0" => {
                salvar_contas(&contas);
                break;
            },
            _ => println!("Escolha inválida, tente novamente."),
        }
    }
}
