use crate::{models::cliente::Cliente, tela::{ler, operacoes_basicas::*, servico_cliente::*}};

pub fn mostra_menu(clientes: &mut Vec<Cliente>) {
    loop {
        limpar_tela();

        println!("\
            ==============Menu=============\n\
            Escolha uma das op;ões abaixo:\n\
            1 - Cadastrar cliente\n\
            2 - Alterar cliente\n\
            3 - Excluir cliente\n\
            4 - Listar clientes\n\
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
                println!("Listar clientes selecionado.");
            }
            3 => {
                // Logica para excluir o cliente selecionado
                println!("Excluir cliente selecionado.");
            }
            4 => {
                // Logica para listar clientes
                println!("Listar clientes")
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
        esperar(1);
    }
}