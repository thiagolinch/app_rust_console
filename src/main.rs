mod models;

use models::cliente::Cliente;

fn main() {
    let cliente: Cliente = Cliente {
        id: 1,
        nome: String::from("Thiago"),
        cpf: String::from("123.456.789-00"),
        endereco: String::from("Rua Exemplo, 123"),
    };

    println!("{}", cliente.id);
    println!("{}", cliente.nome);
    println!("{}", cliente.cpf);
    println!("{}", cliente.endereco);
}
