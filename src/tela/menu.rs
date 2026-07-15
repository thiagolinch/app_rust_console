use crate::{models::cliente::Cliente, tela::{ler, operacoes_basicas::*, servico_cliente::*}};

pub fn mostra_menu(clientes: &mut Vec<Cliente>) {
    loop {
        limpar_tela();

        println!("\
            ==============Menu=============\n\
            Escolha uma das op;ões abaixo:\n\
            1 - Cadastrar cliente\n\
            2 - Listar clientes\n\
            3 - Alterar cliente\n\
            4 - Excluir cliente\n\
            0 - Sair\n\
        ");

        let opcao = ler::ler_dados_int();
        

        match opcao {
            1 => {
                // Lógica para cadastrar cliente
                incluir_cliente(clientes);
            }
            2 => {
                // Lógica para listar clientes
                listar_clientes(clientes);
            }
            3 => {
                // Logica para alterar o cliente selecionado
                alterar_cliente(clientes);
            }
            4 => {
                // Logica para excluir clientes
                excluir_cliente(clientes)
            }
            0 => {
                // Logica para sair do programa
                break;
            }
            _ => {
                println!("Opção inválida. Tente novamente.");
            }
        }

        // println!("Digite enter para continuar");
        // ler::ler_dados();
        // esperar(1);
    }
}