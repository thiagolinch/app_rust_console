
use crate::{models::cliente::Cliente, tela::{ler::*, operacoes_basicas::*, utils::*}};

pub fn incluir_cliente(clientes: &mut Vec<Cliente>) {
    limpar_tela();
    
    let mut cliente: Cliente = Cliente::default();
    cliente.id = clientes.len() + 1;

    digitar_dados_do_cliente(&mut cliente);

    clientes.push(cliente);
    limpar_tela();

    println!("Cliente cadastrado com sucesso");
    esperar(1);
}

pub fn listar_clientes(clientes: &mut Vec<Cliente>) {
    limpar_tela();
    if nao_tem_clientes(clientes) {
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

pub fn alterar_cliente(clientes: &mut Vec<Cliente>) {
    limpar_tela();
    if nao_tem_clientes(clientes) {
        return;
    }

    let id = captura_id();
    if let Some((indice, cliente)) =  buscar_cliente_id(clientes, id){
        println!("{}", "-".to_string().repeat(40));
        println!("Alterando o cliente");
        println!("{}", "-".to_string().repeat(40));
        mostrar_cliente(cliente);
        println!("{}", "-".to_string().repeat(40));
        digitar_dados_do_cliente(&mut clientes[indice]);

        limpar_tela();
        println!("Cliente alterado com sucesso");
        esperar(1);
    } else {
        limpar_tela();
        println!("Cliente nao encontrado")
    }
    esperar(1);
}

pub fn excluir_cliente(clientes: &mut Vec<Cliente>) {
    limpar_tela();
    if nao_tem_clientes(clientes) {
        return;
    }

    let id = captura_id();
    if let Some((indice, cliente)) =  buscar_cliente_id(clientes, id){
        println!("{}", "-".to_string().repeat(40));
        println!("Confirme a exclusao do cliente abaixo?");
        println!("{}", "-".to_string().repeat(40));
        mostrar_cliente(cliente);
        println!("{}", "-".to_string().repeat(40));
        println!("\
            1 - Sim\n\
            2 - Não
        ");
        let opcao = ler_dados();
        if opcao.trim() == "1" {
            clientes.remove(indice);
            limpar_tela();
            println!("Cliente excluido com sucesso");
            esperar(1);
        }
    } else {
        limpar_tela();
        println!("Cliente nao encontrado")
    }
    esperar(1);
}
