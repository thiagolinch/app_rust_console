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

pub fn listar_clientes(clientes: &mut Vec<Cliente>) {
    limpar_tela();

    if clientes.len() == 0 {
        println!("Nao existe cliente cadastrado");
        esperar(1);
        return;
    }
    println!("{}", "-".to_string().repeat(40));

    for cliente in clientes{
        mostrar_cliente(cliente);
        println!("{}", "-".to_string().repeat(40));
    }

    println!("Digite enter para continuar...");
    ler_dados();
}

fn mostrar_cliente(cliente: &mut Cliente) {
    println!("\
        ID: {}\n\
        Nome: {}\n\
        CPF: {}\n\
        Endereco: {}
    ", cliente.id, cliente.nome,cliente.cpf,cliente.endereco)
}