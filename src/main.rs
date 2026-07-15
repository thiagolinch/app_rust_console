mod models;
mod tela;

use models::cliente::Cliente;
use tela::menu as menu;

fn main() {
    let mut clientes:Vec<Cliente> = Vec::new();
    menu::mostra_menu(&mut clientes);
}
