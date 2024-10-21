use std::f32::consts::PI;
use std::ops::Deref;

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


// -- Constantes:
const NUMERO_PI: f32 = 3.14;


// -- VARIAVEIS STATICAS :
static mut STATIC_VARIAVEL: i32 = 15;

// -- Estruturas ( Struct ) :

#[ derive( Debug ) ]
struct User {
    username: String,
    email: String,
    active: bool,
    gender: Gender,
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


    // -- Constantes :

    println!( "A circuferencia de raio 2 possui comprimento de {}", comprimento_circuferencia(2.0) );

    // -----------------------------------------


    // -- Bloco de Codigos :

    let a = 10;

    let c = {
        println!( "{}", a );

        let a = 20;
        let b = 15;

        println!( "{}", a );

        b > a
    };

    println!( "{}", a );
    println!( "b maior que a? {}", c );

    // -----------------------------------------


    // -- Variaveis Staticas :

    unsafe {
        println!( "O valor de STATIC_VARIAVEL = {}", STATIC_VARIAVEL );
    }

    // -----------------------------------------


    // -- Referencias :

    let mut x = 10;
    let y = &mut x;

    *y += 1;

    println!( "{}", y );
    println!( "{}", x );

    // -----------------------------------------

    // -- Estruturas ( Struct ) :

    let mut pessoa = User {
        username : String::from( "V3ktor" ),
        email : String::from( "v3ktoryt@gmail.com" ),
        active : true,
        gender: Gender::Male
    };

    println!( "{:?}", pessoa );
    println!( " Nome de usuario: {} ", pessoa.username );

    pessoa.active = false;

    // -----------------------------------------

    // -- String ( Coleção )

    let mut minhaString = String::from("Olá meu nome é bruno");

    println!( "O tamnho da string eh {}", minhaString.len() );
    println!( "A string ta vazia? {}", minhaString.is_empty() );

    for token in minhaString.split_whitespace() {
        println!( "{}", token );
    }

    println!( "O nome bruno esta contido na string? {}", minhaString.contains("bruno") );

        // Tem que ser mutavel
    minhaString.push_str(", Bem vindo.");

    // -----------------------------------------

    // -- Tarefa Reversão de String:

    println!( "A string 'bruno' ao contraria eh {}", reverse_string( &String::from("bruno") ) );

    // -----------------------------------------

    // -- Tarefa Anagrama de String:

    println!( "A string 'roma' eh anagrama de 'amor' {}", is_anagram( &String::from("amor"), &String::from("roma") ) );
    println!( "A string 'amor' eh anagrama de 'amor' {}", is_anagram( &String::from("amor"), &String::from("amor") ) );
    println!( "A string 'tres' eh anagrama de 'amor' {}", is_anagram( &String::from("amor"), &String::from("tres") ) );

    // -----------------------------------------

    // -- Tarefa Palíndromo:

    println!( "A string 'ovo' eh palindromo ? {}", eh_palindromo( &String::from("ovo") ) );
    println!( "A string 'radar' eh palindromo ? {}", eh_palindromo( &String::from("radar") ) );
    println!( "A string 'bruno' eh palindromo ? {}", eh_palindromo( &String::from("bruno") ) );

    println!( "A string 'A dama admirou o rim da amada.' eh palindromo ? {}", eh_palindromo( &String::from("A dama admirou o rim da amada.") ) );

    // -----------------------------------------

    // -- Pass by Reference:

    let user1 = User{
        username: String::from( "Amanda" ),
        email: String::from( "amanda@email.com" ),
        gender: Gender::Female,
        active: true
    };

    print_username( &user1 );

    // -----------------------------------------


}

fn eh_palindromo( input: &String ) -> bool {

    let new_text = input.chars().filter( |c| -> bool {
        !c.is_ascii_punctuation() && !c.is_ascii_whitespace()
    } ).collect::<String>().to_lowercase();

    let rev_text = new_text.chars().rev().collect::<String>();

    new_text == rev_text
}

fn is_anagram( a: &String, b: &String ) -> bool {

    if ( a.to_lowercase() == b.to_lowercase() ) || (a.len() != b.len()) { return false }

    let mut a_chars = a.chars().collect::< Vec<char> >();
    let mut b_chars = b.chars().collect::< Vec<char> >();

    a_chars.sort();
    b_chars.sort();

    a_chars == b_chars
}

fn reverse_string( input: &String ) -> String {
    input.chars().rev().collect::<String>()
}

// -----------------------------------------

fn print_username( user: &User ) {
    println!( "Username: {}", user.username );
}

fn update_value( x: &i32, y: &mut i32 ) {
    *y = x + 10;
}

fn comprimento_circuferencia(r: f32) -> f32 {
    let c = 2.0 * r * NUMERO_PI;
    c
}

fn nacionalidade_carro( car: CarModel ) {

    match car {
        CarModel::Fiat => println!("Carro italiano"),
        CarModel::Ford => println!("Carro americano"),
        CarModel::Renault => println!("Carro frances"),
    }

}