
use crate::{models::cliente::Cliente, tela::{ler::{ler_dados, ler_dados_int}, operacoes_basicas::*}};

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

fn digitar_dados_do_cliente(cliente: &mut Cliente) {
    println!("Digite o nome do cliente");
    cliente.nome = ler_dados();
    println!("Digite o CPF do cliente: ");
    cliente.cpf = ler_dados();
    println!("Digite o endereço do cliente: ");
    cliente.endereco = ler_dados();
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

fn buscar_cliente_id(clientes: &Vec<Cliente>, id: usize) -> Option<(usize, &Cliente)> {
    clientes.iter().enumerate().find(|(_, cliente  )| cliente.id == id)
}

fn captura_id() -> usize {
    limpar_tela();
    println!("Digite o ID do cliente:");
    ler_dados_int()
}

fn nao_tem_clientes(clientes: &[Cliente]) -> bool {
    if clientes.len() == 0 {
        println!("Nao existe cliente cadastrado");
        esperar(1);
        return true;
    }
    return false;
}

fn mostrar_cliente(cliente: &Cliente) {
    println!("\
        ID: {}\n\
        Nome: {}\n\
        CPF: {}\n\
        Endereco: {}
    ", cliente.id, cliente.nome,cliente.cpf,cliente.endereco)
}