
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug)]
enum Gender {
    Male,
    Female,
    NoBinary,
    Other
}

#[derive(Debug)]
enum CarModel {
    Fiat,
    Ford,
    Renault
}

enum Pagamento {
    Dinheiro(f32),
    Credito(f32),
    Debito(f32),
    Pix(f32),
    Parcelado(f32, u8)
}

fn main() {

    // -- Tuplas:

    let mut tupla = (12, "valores", 3.14, (1, 2, 3));
    println!("{}", tupla.0);
    println!("{}", tupla.1);
    println!("{}", tupla.2);
    println!("({}, {}, {})", tupla.3.0, tupla.3.1, tupla.3.2);

    //tupla.0 = 15;
    let (a, b, c, d) = tupla;

    tupla.0 = 15;

    println!( "valor de a = {}", a );
    println!( "valor de b = {}", b );
    println!( "valor de c = {}", c );
    println!( "valor de d = {:?}", d );

    // -----------------------------------------

    // -- Enums:

    let direcao: Direction = Direction::Right;
    match direcao {
        Direction::Up => println!("up"),
        Direction::Down => println!("down"),
        Direction::Left => println!("left"),
        Direction::Right => println!("right")
    }

    let pessoa: Gender = Gender::NoBinary;
    match pessoa {
        Gender::Male => println!("Masculino"),
        Gender::Female => println!("Feminino"),
        Gender::NoBinary => println!("Nao Binario"),
        Gender::Other => println!("Outro")
    }

    println!( "{:?}", pessoa );

    nacionalidade_carro( CarModel::Ford );

    let compra1: Pagamento = Pagamento::Parcelado(500.0, 3);
    match compra1 {
        Pagamento::Dinheiro(valor) => println!("Dinheiro - {}", valor),
        Pagamento::Credito(valor) => println!("Credito - {}", valor),
        Pagamento::Debito(valor) => println!("Debito - {}", valor),
        Pagamento::Pix(valor) => println!("Pix - {}", valor),

        _ => println!("Outro metodo")
    }

    let compra2: Pagamento = Pagamento::Parcelado(1250.0, 12);
    match compra2 {
        Pagamento::Dinheiro(valor) => println!("Dinheiro - {}", valor),
        Pagamento::Credito(valor) => println!("Credito - {}", valor),
        Pagamento::Debito(valor) => println!("Debito - {}", valor),
        Pagamento::Pix(valor) => println!("Pix - {}", valor),
        Pagamento::Parcelado(valor, parcelas) => println!("Parcelado - {} em {}x", valor, parcelas)
    }

    // -----------------------------------------


}

fn nacionalidade_carro( car: CarModel ) {

    match car {
        CarModel::Fiat => println!("Carro italiano"),
        CarModel::Ford => println!("Carro americano"),
        CarModel::Renault => println!("Carro frances"),
    }

}