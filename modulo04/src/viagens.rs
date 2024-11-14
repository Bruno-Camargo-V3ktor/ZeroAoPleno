// Structs
#[derive(Debug)]
pub struct Passageiro {
    nome: String,
    passaporte: String,
    idade: u8
}

#[derive(Debug)]
pub struct Voo {
    codigo: String,
    partida: String,
    destino: String,
    data_partida: String,
    hora_partida: String,
}

// Functions
pub fn add_passageiro(array: &mut Vec<Passageiro>, nome_passageiro: &str, passaporte_passageiro: &str, idade_passageiro: u8) {

    let passageiro = Passageiro {
        nome: nome_passageiro.to_string(),
        passaporte: passaporte_passageiro.to_string(),
        idade: idade_passageiro
    };

    array.push( passageiro );
}


pub fn add_voo(array: &mut Vec<Voo>, codigo_voo: &str, partida: &str, destino: &str, data_partida: &str, hora_partida: &str) {

    let voo = Voo {
        codigo: codigo_voo.to_string(),
        partida: partida.to_string(),
        destino: destino.to_string(),
        data_partida: data_partida.to_string(),
        hora_partida: hora_partida.to_string()
    };

    array.push( voo );
}

pub fn exibir_voos(array: &Vec<Voo>) {
    for voo in array {
        println!("{:?}", voo);
    }
}

pub fn exibir_passageiros(array: &Vec<Passageiro>) {
    for passageiro in array {
        println!("{:?}", passageiro);
    }
}