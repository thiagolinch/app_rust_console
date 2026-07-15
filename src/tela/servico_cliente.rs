use crate::{models::cliente::Cliente, tela::{ler::ler_dados, operacoes_basicas::*}};

pub fn incluir_cliente(clientes: &mut Vec<Cliente>) {
    limpar_tela();
    
    let mut cliente: Cliente = Cliente::default();
    cliente.id = clientes.len() + 1;

    println!("Digite o nome do cliente");
    cliente.nome = ler_dados();
    println!("Digite o CPF do cliente: ");
    cliente.cpf = ler_dados();
    println!("Digite o endereço do cliente: ");
    cliente.endereco = ler_dados();

    clientes.push(cliente);
    limpar_tela();

    println!("Cliente cadastrado com sucesso");
    esperar(1);
}