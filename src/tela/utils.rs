use crate::{models::cliente::Cliente, tela::{ler::*, operacoes_basicas::*}};

pub fn digitar_dados_do_cliente(cliente: &mut Cliente) {
    println!("Digite o nome do cliente");
    cliente.nome = ler_dados();
    println!("Digite o CPF do cliente: ");
    cliente.cpf = ler_dados();
    println!("Digite o endereço do cliente: ");
    cliente.endereco = ler_dados();
}

pub fn buscar_cliente_id(clientes: &Vec<Cliente>, id: usize) -> Option<(usize, &Cliente)> {
    clientes.iter().enumerate().find(|(_, cliente  )| cliente.id == id)
}

pub fn captura_id() -> usize {
    limpar_tela();
    println!("Digite o ID do cliente:");
    ler_dados_int()
}

pub fn nao_tem_clientes(clientes: &[Cliente]) -> bool {
    if clientes.len() == 0 {
        println!("Nao existe cliente cadastrado");
        esperar(1);
        return true;
    }
    return false;
}

pub fn mostrar_cliente(cliente: &Cliente) {
    println!("\
        ID: {}\n\
        Nome: {}\n\
        CPF: {}\n\
        Endereco: {}
    ", cliente.id, cliente.nome,cliente.cpf,cliente.endereco)
}