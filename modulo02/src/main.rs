mod coin;
mod block;
mod bank;

use std::collections::{HashMap, HashSet};
use std::f32::consts::PI;
use std::ops::Deref;
use coin::Coin;
use block::Block;
use crate::bank::{Account, Currency};

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

//             Nome,  Email,  Idade, Altura, Peso, Genero
struct People( String, String, u8, f32, f32, Gender );

// -- Implementantion:

impl User {

    fn nome_do_usuario(&self) {
        println!( "O nome do usuario eh {}", self.username );
    }

    fn esta_ativo(&self) {
        println!( "O usuario esta ativo? {}", self.active );
    }

}


enum Asset {
    Stocks,
    Bonds,
    Funds,
    Cash
}

impl Asset {
    fn price(&self) -> f32 {
        match self {
            Asset::Stocks => 10.0,
            Asset::Bonds => 20.0,
            Asset::Funds => 30.0,
            Asset::Cash => 40.0
        }
    }
}

struct Produto {
    nome: String,
    preco: f32,
    quantidade: u16
}

impl Produto {

    fn new(nome: &str, preco: f32, quantidade: u16) -> Self {
        Self { nome: String::from(nome), preco, quantidade }
    }

    fn set_nome(&mut self, nome: &str) {
        self.nome = String::from(nome);
    }

    fn set_preco(&mut self, preco: f32) {
        self.preco = preco;
    }

    fn set_quantidade(&mut self, quantidade: u16) {
        self.quantidade = quantidade;
    }

    fn debug(&self) {
        println!( "(Nome: {}, Preco: {}, Quantidade: {})", self.nome, self.preco, self.quantidade );
    }

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

    // -- Tuple Structs:

    let pessoa = People(
        String::from( "Bruno" ),
        String::from( "bruno@email.com" ),
        18,
        1.70,
        87.9,
        Gender::Male
    );

    println!( "\nNome: {}", pessoa.0 );
    println!( "Email: {}", pessoa.1 );
    println!( "Idade: {}", pessoa.2 );
    println!( "Altura: {}", pessoa.3 );
    println!( "Peso: {}", pessoa.4 );
    println!( "Genero: {:?}", pessoa.5 );


    // -----------------------------------------


    // -- Arrays:

    let numeros_inteiros = [1, 2, 3, 4, 5];
    println!( "\n{}", numeros_inteiros[0] );
    println!( "{:?}", numeros_inteiros );

    for i in 0..numeros_inteiros.len() {
        println!( "{}", numeros_inteiros[i] );
    }

    for num in numeros_inteiros {
        println!( "{}", num );
    }


    // -----------------------------------------


    // -- Tarefa Rotação de Array:

    let mut array = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    println!( "\nAntes de rotacionar: {:?}", array );

    //rotacionar_array( &mut array, 3 );
    rotation_array( &mut array, 4 );

    println!( "Depois de rotacionar: {:?}", array );

    // -----------------------------------------


    // -- Implementantion:

    let user = User{
        username: String::from( "Amanda" ),
        email: String::from( "amanda@email.com" ),
        gender: Gender::Female,
        active: true
    };

    user.nome_do_usuario();
    user.esta_ativo();

    // -----------------------------------------


    // -- Vetores:

    let vetor1 = vec![1, 2, 3, 4];
    let mut vetor2: Vec<i32> = Vec::new();


    println!( "\n{}", vetor1[0] );
    println!( "{}", vetor1.get(1).unwrap() );
    println!( "{}", vetor1[2] );
    println!( "{}", vetor1.get(3).unwrap() );

    vetor2.push(10);
    vetor2.push(20);
    vetor2.push(30);
    vetor2.push(40);

    vetor2.remove(3);

    println!( "\n{:?}", vetor2 );


    // -----------------------------------------


    // -- Tarefa Encontrar k-esimo Maior:

    let valores = vec![12, 3, 124, 5, 44, 3, 9, 2, 7, 5];

    println!( "\nO primeiro maior do vetor eh {}", encontrar_kesimo_maior( &valores, 1 ) );
    println!( "O segundo maior do vetor eh {}", encontrar_kesimo_maior( &valores, 2 ) );
    println!( "O terceiro maior do vetor eh {}", encontrar_kesimo_maior( &valores, 3 ) );
    println!( "Valores: {:?}", valores );

    // -----------------------------------------


    // -- Tarefa Melhor momento para comprar e vender uma acao:

    let valores_semana1 = vec![7, 1, 5, 3, 6, 4, 1];
    let valores_semana2 = vec![7, 6, 6, 3, 3, 1, 1];

    let maior_semana1 = maior_lucro_acoes( &valores_semana1 );
    let maior_semana2 = maior_lucro_acoes( &valores_semana2 );

    println!( "\nO maior lucro possivel: {}", maior_semana1 );
    println!( "O maior lucro possivel: {}", maior_semana2 );

    // -----------------------------------------


    // -- Tarefa Movendo os Zeros:

    let mut nums01 = vec![0, 1, 0, 3, 12];
    let mut nums02 = vec![0, 4, 7, 0, 23];

    move_zeros( &mut nums01 );
    mover_zeros( &mut nums02 );

    println!( "\nZeros movidos: {:?}", nums01 );
    println!( "Zeros movidos: {:?}", nums02 );

    // -----------------------------------------


    // -- HashMap:

    let mut notas: HashMap<&str, f32> = HashMap::new();

    notas.insert("Matematica", 5.0);
    notas.insert("Portugues", 7.2);
    notas.insert("Historia", 6.8);
    notas.insert("Ciencia", 9.2);

    match notas.get("Matematica") {
        Some(val) => { println!( "\nNota em de matematica: {}", val ) },
        None => { println!( "Sem nota em Matematica" ) }
    }

    notas.remove("Matematica");
    println!( "Valores: {:?}", notas.values() );
    println!( "Chaves {:?}", notas.keys() );
    println!( "Aluno estudou informatica? {}", notas.contains_key("Informatica") );

    for (materia, nota) in notas {
        println!( "Media de {}: {}", materia, nota );
    }

    // -----------------------------------------


    // -- Tarefa: Encontrar Duplicatas em um Vetor:

    let nums1 = vec![1, 2, 3, 4, 5];
    let nums2 = vec![1, 2, 3, 4, 1];

    println!( "\nArray 1 contém duplicatas: {}", contains_duplicates(&nums1) );
    println!( "Array 2 contém duplicatas: {}", contains_duplicates(&nums2) );

    // -----------------------------------------


    // -- Tarefa: Construção de Strings

    let (ransomNote, magazine) = ("a", "b");
    println!( "\nransomNote = '{}', magazine = '{}' ? {} ", ransomNote, magazine, construcao_de_string(ransomNote, magazine) );

    let (ransomNote, magazine) = ("aa", "ab");
    println!( "ransomNote = '{}', magazine = '{}' ? {} ", ransomNote, magazine, can_construct(ransomNote, magazine) );

    let (ransomNote, magazine) = ("aa", "aab");
    println!( "ransomNote = '{}', magazine = '{}' ? {} ", ransomNote, magazine, construcao_de_string(ransomNote, magazine) );

    // -----------------------------------------


    // -- Tarefa:  Verificação de Padrão de Palavras

    let pattern1 = "abba";
    let str1 = "dog cat cat dog";
    println!("\nSegue o padrão: {}", word_pattern(pattern1, str1)); // Saída: true

    let pattern2 = "abba";
    let str2 = "dog cat cat fish";
    println!("Segue o padrão: {}", identificar_padrao(pattern2, str2)); // Saída: false

    let pattern3 = "aaaa";
    let str3 = "cat cat cat cat";
    println!("Segue o padrão: {}", identificar_padrao(pattern3, str3)); // Saída: false

    let pattern4 = "abba";
    let str4 = "dog dog dog dog";
    println!("Segue o padrão: {}", identificar_padrao(pattern4, str4)); //

    // -----------------------------------------


    // -- Tarefa: Intersecção de Dois Vetores

    let (nums1, nums2) = (vec![1, 2, 2, 1], vec![2, 2]);
    println!( "\nInterseccao de {:?} e {:?}: {:?}", nums1, nums2, calcular_intersecao(&nums1, &nums2) );

    let (nums1, nums2) = (vec![1, 3, 2, 1], vec![2, 3]);
    println!( "\nInterseccao de {:?} e {:?}: {:?}", nums1, nums2, intersecao(&nums1, &nums2) );

    // -----------------------------------------


    // -- Tarefa: Encontrar a Soma Mais Próxima

    let nums = vec![-1, 2, 1, -4, 4];
    let target = 1;

    let result = three_sum_closest(nums, target);

    println!("\nA soma mais próxima do alvo é: {}", result);

    // -----------------------------------------


    // -- Tarefa: Todos os Subsets

    let nums = vec![1,2,3,4,5];
    println!( "\nSubconjubtos de {:?} -> {:?}", nums, subconjuntos(&nums) );

    let nums = vec![0];
    println!( "Subconjubtos de {:?} -> {:?}", nums.clone(), subsets(nums) );

    // -----------------------------------------


    // -- Tarefa: Contando as letras maiúsculas em uma string

    let texto = String::from("Contando as Letras Maiúsculas Em Uma String");
    println!( "\nQuantidade de letras maisculas: {}", conta_maiusculas( &texto ) );


    // -----------------------------------------


    // -- Tarefa: Criar um Programa de Portfólio para o Mercado Financeiro.

    let portifolio = [Asset::Stocks, Asset::Bonds, Asset::Cash, Asset::Funds];
    let total: f32 = portifolio.iter().map( | x | x.price() ).sum();

    println!("\nO valor total de seu portfólio é de R$ {}.", total);

    // -----------------------------------------


    // -- Tarefa: Criando uma moeda

    let mut dogcoin = Coin::new( 10.45 );
    println!( "\nDogcoin esta valendo: {}", dogcoin.get_value() );
    dogcoin.set_value( 0.45 );
    println!( "Dogcoin esta valendo: {}", dogcoin.get_value() );

    // -----------------------------------------


    // -- Tarefa: Criando structs e implementando métodos

    let mut produto1 = Produto::new( "Iphone 11", 2500.00, 10 );
    produto1.debug();

    // -----------------------------------------


    // -- Tarefa: Implementação de um Block struct para uma blockchain

    let my_block = Block::new(0, 1605991464000, "dados do bloco".to_string(),  "hash".to_string()  , "hash anterior".to_string());
    let size = my_block.data_size();
    let time = my_block.creation_time();
    println!("\nO tamanho do dado do bloco é: {} e foi criado em: {} segundos", size, time);

    // -----------------------------------------


    // -- Tarefa: Implementação de uma conta bancaria

    let mut conta = Account::new( Currency::USD, 100.0 );
    println!( "\nSaldo inicial: {}", conta.check_balance() );

    conta.deposit( 50.0 );
    println!( "Saldo apos deposito: {}", conta.check_balance() );

    println!( "Saldo convertido em BRL: {}", conta.convert_to( Currency::BRL ) )

    // -----------------------------------------

}

fn conta_maiusculas( input: &String ) -> u32 {
    let mut qtd = 0;
    for ch in input.chars() { if ch.is_uppercase() { qtd += 1; } }
    qtd
}

fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut resultado = Vec::new();
    let mut subset_atual = Vec::new();

    fn backtrack(nums: &Vec<i32>, inicio: usize, subset_atual: &mut Vec<i32>, resultado: &mut Vec<Vec<i32>>) {
        resultado.push(subset_atual.clone());

        for i in inicio..nums.len() {
            subset_atual.push(nums[i]);
            backtrack(nums, i + 1, subset_atual, resultado);
            subset_atual.pop();
        }
    }

    backtrack(&nums, 0, &mut subset_atual, &mut resultado);
    resultado
}


fn subconjuntos( array: &Vec< i32 > ) -> Vec< Vec< i32 > > {
    if (array.len() <= 0) || (array.len() > 10)  { return vec![ vec![] ] }

    let mut subconjuntos: HashSet< Vec<i32> > = HashSet::new();
    subconjuntos.insert( vec![] );

    if array.len() > 1 {
        subconjuntos.insert( vec![ *array.first().unwrap(),  *array.last().unwrap() ] );
    }

    for i in 0..array.len() {

        let start = i;
        let mut end = start+1;

        while end <= array.len() {
            subconjuntos.insert( array[start..end].to_vec() );
            end += 1;
        }

    }

    let mut res: Vec<_> = subconjuntos.iter().map( |x| x.clone() ).collect();
    res.sort();
    res
}

fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    if nums.len() < 3 { return 0; }
    let mut results: HashSet<i32> = HashSet::new();


    for start in 0..nums.len() {

        let end = start + 2;
        if end > ( nums.len()-1 ) { break; }

        let res: i32 = nums[start..end+1].iter().sum();
        results.insert( res );

    }

    let mut res = *results.iter().min().unwrap();
    for &num in results.iter() {
        if num == target { return num; }

        let last_dif = ( res - target ).abs();
        let dif = ( num - target ).abs();

        if dif <= last_dif  { res = num }
    }

    res
}

fn intersecao(nums1: &Vec<i32>, nums2: &Vec<i32>) -> Vec<i32> {
    // Converte os arrays para HashSet para busca eficiente
    let conjunto1: HashSet<_> = nums1.into_iter().collect();
    let conjunto2: HashSet<_> = nums2.into_iter().collect();

    // Utiliza o método intersection em HashSet para encontrar elementos comuns
    let resultado: Vec<_> = conjunto1.intersection(&conjunto2).cloned().collect();

    resultado.iter().map( |x| **x ).collect()
}

fn calcular_intersecao( vec1: &Vec<i32>, vec2: &Vec<i32> ) -> Vec< i32 > {

    let qtd = {
        if vec1.len() < vec2.len() { vec1.len() }
        else { vec2.len() }
    };

    let mut intersecao: HashSet< i32 > = HashSet::new();

    for i in 0..qtd{

        let num = vec1[i];
        if vec2.contains(&num) {
            intersecao.insert( num );
        }

    }

    intersecao.iter().map( |x| *x ).collect()
}

fn word_pattern(pattern: &str, str_val: &str) -> bool {

    let pattern_chars: Vec<char> = pattern.chars().collect();
    let words: Vec<&str> = str_val.split_whitespace().collect();

    if pattern_chars.len() != words.len() { return false; }

    let mut char_to_word = HashMap::new();
    let mut word_to_char = HashMap::new();
    let mut used_words = HashSet::new();

    for (i, &ch) in pattern_chars.iter().enumerate() {
        match (char_to_word.get(&ch), word_to_char.get(&words[i])) {
            (Some(&word), Some(&character)) => {
                if word != words[i] || character != ch {
                    return false;
                }
            }
            (None, None) => {
                char_to_word.insert(ch, words[i]);
                word_to_char.insert(words[i], ch);
                used_words.insert(words[i]);
            }
            _ => return false,
        }
    }

    // Check if each character corresponds to a unique word and vice versa
    char_to_word.len() == used_words.len() && word_to_char.len() == used_words.len()
}

fn identificar_padrao( pattern: &str, input: &str ) -> bool {

    let mut configuracao:HashMap<char, &str> = HashMap::new();
    let palavras = input.split(" ").collect::< HashSet<&str>>();
    let padroes = pattern.chars().collect::< HashSet<char> >();

    if ( palavras.len() != padroes.len() ) { return false }

    for i in 0..palavras.len() {
        let words = palavras.iter().collect::< Vec<&&str> >();
        let chs = padroes.iter().collect::< Vec<&char> >();

        let p = **words.get(i).unwrap();
        let c = **chs.get(i).unwrap();

        configuracao.insert( c, p );
    }

    let mut text = String::new();
    for ch in pattern.chars() {
        let &palavra = configuracao.get( &ch ).unwrap();
        text.push_str( &palavra );
        text.push(' ');
    }

    text.trim() == input
}

fn can_construct(ransom_note: &str, magazine: &str) -> bool {
    let mut magazine_chars = HashMap::new();

    // Contagem de caracteres na revista
    for ch in magazine.chars() {
        *magazine_chars.entry(ch).or_insert(0) += 1;
    }

    // Verificação da construção da nota de resgate
    for ch in ransom_note.chars() {
        if let Some(count) = magazine_chars.get_mut(&ch) {
            if *count == 0 {
                return false; // Não há caracteres suficientes na revista
            }
            *count -= 1;
        } else {
            return false; // Caractere não encontrado na revista
        }
    }

    true
}

fn construcao_de_string( ransomNote: &str, magazine: &str ) -> bool {

    if ( ransomNote.is_empty() || magazine.is_empty() ) || magazine.len() > 105 { return false }
    let mut caracteres_restantes = ransomNote.chars().collect::<Vec<char>>();

    for c in magazine.chars() {

        if caracteres_restantes.contains( &c ) {
            let i = caracteres_restantes.iter().position( | x | *x == c ).unwrap();
            caracteres_restantes.remove(i);
        }

    }

    caracteres_restantes.is_empty()
}

fn contains_duplicates( input: &Vec<i32> ) -> bool {
    if input.is_empty() { return false }

    let set: HashSet<_> = input.iter().collect();

    !(set.len() == input.len())
}

fn mover_zeros(nums: &mut Vec<i32>) {
    let mut indice_nao_zero = 0;

    // Iterar através do array
    for i in 0..nums.len() {
        // Se o elemento atual for não zero
        if nums[i] != 0 {
            // Trocar o elemento não zero com o elemento no índice indice_nao_zero
            nums.swap(i, indice_nao_zero);
            // Avançar o índice indice_nao_zero
            indice_nao_zero += 1;
        }
    }
}

fn move_zeros( input: &mut Vec<i32> ) {
    if input.is_empty() { return; }

    for index in 0..input.len() {

        let value = input[index];
        if value == 0 {
            input.remove(index);
            input.push(0);
        }

    }
}

fn lucro_maximo(prices: Vec<i32>) -> i32 {
    if prices.is_empty() {
        return 0;
    }

    let mut preco_minimo = prices[0];
    let mut lucro_maximo = 0;

    for preco in prices.iter().skip(1) {
        let lucro_atual = preco - preco_minimo;
        lucro_maximo = lucro_maximo.max(lucro_atual);
        preco_minimo = preco_minimo.min(*preco);
    }

    lucro_maximo
}

fn maior_lucro_acoes( values: &Vec<u32> ) -> u32 {

    let mut lucros: Vec<u32> = vec![];
    let days = values.len();

    for day in 0..days-1 {

        let compra = values[day];
        let mut maior_lucro: u32 = 0;

        for vende in values.split_at(day+1).1 {
            if *vende > compra {
                let dif = vende - compra;
                if dif > maior_lucro { maior_lucro = dif; }
                continue;
            }
        }

        lucros.push( maior_lucro );
    }

    lucros.sort();
    *lucros.last().unwrap()
}

fn encontrar_kesimo_maior( nums: &Vec<i32>, k: i32 ) -> i32 {

    let mut copy = nums.clone();
    copy.sort();
    copy.reverse();

    copy[ (k - 1) as usize ]
}

fn rotation_array( origin: &mut [i32], destination: usize ) {

    if ( origin.len() < destination ) || ( origin.is_empty() ) { return }
    origin[0..destination].reverse();

}

fn rotacionar_array( origin: &mut [i32], destination: usize ) {

    if ( origin.len() < destination ) || ( origin.is_empty() ) { return }

    let mut index_d = destination - 1;
    for index_o in 0..origin.len() {
        if index_o >= index_d  { break }

        let value_o = origin[ index_o ];
        let value_d = origin[ index_d ];

        origin[index_o] = value_d;
        origin[index_d] = value_o;

        index_d -= 1;
    }

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