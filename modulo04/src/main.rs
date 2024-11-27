mod it;
mod viagens;

use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::os::unix::raw::blkcnt_t;
use modulo04::executar_estatisticas_descritivas;
use it::Router;
use it::Network;
use crate::viagens::{add_passageiro, add_voo, exibir_passageiros, exibir_voos, Passageiro, Voo};

struct User {
    name: String,
    age: u8,
    friends: Vec< String >,
}

fn main() {

    // -- Media, Mediana e Moda
    println!();

    let notas: Vec<u8> = vec![10, 5, 3, 5, 6, 7];
    println!( "Media: {}", media( &notas ) );
    println!( "Mediana: {}", mediana( &notas ) );
    println!( "Moda: {}", moda( &notas ) );

    // --------------------------------

    // -- Tarefa: Estatísticas Descritivas com Structs
    println!();

    let estatistica =
        executar_estatisticas_descritivas( ler_numeros( "dados.txt".to_string() ) );

    // --------------------------------

    // -- Algoritimo Pig-Latin
    println!();

    let palavra1 = String::from( "amor" );
    let palavra2 = String::from( "time" );

    println!( "{}", convert_to_pig_latin( &palavra1 ) );
    println!( "{}", convert_to_pig_latin( & palavra2 ) );

    // --------------------------------


    // -- Departamentos
    println!();

    departamento();

    // --------------------------------


    // -- Tarefa: Rede Socias Simples

    println!();

    let mut users: HashMap<String, User> = HashMap::new();

    loop {

        println!( "\n -- REDE SOCIAL MENU -- " );
        println!( "1 -> Adicionar Usuário");
        println!( "2 -> Adicionar Amigo");
        println!( "3 -> Ver Lista De Amigos");
        println!( "4 -> Sair");

        let mut op = String::new();
        stdin().read_line( &mut op ).expect( "Failed to read line!" );
        let choose = op.trim().parse().unwrap();

        match choose {
            1 => {
                println!("\nNome do novo Usuario: ");
                let mut name = String::new();
                stdin().read_line( &mut name ).expect( "Failed to read line!" );

                println!("\nIdade do novo Usuario: ");
                let mut age = String::new();
                stdin().read_line( &mut age ).expect( "Failed to read line!" );
                let age: u8 = age.trim().parse().unwrap();


                let user = User {  name: name.clone() , age, friends: Vec::new() };
                users.insert( name, user );
            },

            2 => {

                println!("\nNome do Usuario: ");
                let mut user_name = String::new();
                stdin().read_line( &mut user_name ).expect( "Failed to read line!" );

                println!("\nNome do Amigo");
                let mut friend_name = String::new();
                stdin().read_line( &mut friend_name ).expect( "Failed to read line!" );


                let friend_option = users.remove( &friend_name );
                let user_option = users.get_mut( &user_name );

                match ( user_option, &friend_option ) {
                    ( Some(user), Some(friend) ) => {
                        user.friends.push( friend.name.clone() );
                    }

                    _ => { println!( "Usuario ou Amigo nao encontrado!!!" ); }
                }

                match friend_option {
                    Some(user) => { users.insert( user.name.clone(), user ); },
                    _ => {}
                }

            },

            3 => {
                println!("\nNome do Usuario: ");
                let mut user_name = String::new();
                stdin().read_line( &mut user_name ).expect( "Failed to read line!" );

                match users.get_mut( &user_name ) {
                    Some(user) => {
                        for (pos, friend) in user.friends.iter().enumerate() {
                           println!( "{} - {}", pos, friend );
                        }
                    }
                    None => { println!( "Usuario nao encontrado!!!" ) }
                }

            },

            4 => { break; },

            _ => { break; }
        }

    }

    // --------------------------------


    // -- Tarefa: Implementação de uma trait de rede com Rust
    println!();

    let router = Router::new( "192.133.3.1" );

    let is_alive = router.ping( "google.com" );
    let trace = router.traceroute("google.com" );
    let ip = router.nslookup( "google.com" );

    println!("O host está online: {}", is_alive);
    println!("Trace de rota: {:?}", trace);
    println!("Endereço IP: {}", ip);

    // --------------------------------


    // -- Tarefa: Companhia De Viagens
    println!();

    let mut passageiros: Vec<Passageiro> = Vec::new();
    let mut voos: Vec<Voo> = Vec::new();

    // Adicione alguns passageiros
    add_passageiro(&mut passageiros, "João", "123ABC", 18);
    add_passageiro(&mut passageiros, "Maria", "456DEF", 21);
    add_passageiro(&mut passageiros, "Pedro", "789GHI", 25);

    // Adicione alguns vôos
    add_voo(&mut voos, "Voo101", "São Paulo", "Rio de Janeiro", "30/06/2020", "09:00");
    add_voo(&mut voos, "Voo102", "Rio de Janeiro", "São Paulo", "01/07/2020", "09:00");

    // Exiba os passageiros
    exibir_passageiros(&passageiros);

    // Exiba os vôos
    exibir_voos(&voos);

    // --------------------------------


    // -- Tarefa: Caça-Palavra
    println!();

    let board = [
        ['A', 'B', 'E', 'E'],
        ['S', 'F', 'C', 'S'],
        ['A', 'B', 'C', 'D']
    ];
    let word = "SEE";


    let board = [
        ['I', 'H', 'G', 'F'],
        ['J', 'K', 'F', 'E'],
        ['A', 'B', 'C', 'D']
    ];
    let word = "ABCDEFGHIJK";

    let contain = search_word_in_board( &word, &board );
    println!( "A palavra {}, contem no tabuleiro? {}", word, contain );


    // --------------------------------

}

fn search_word_in_board( word: &str, board: &[ [char; 4]; 3 ] ) -> bool {

    let l = board.len();
    let c = board[0].len();

    for i in 0..l {
        for j in 0..c {

            let current_char = board[i][j];
            let mut chars = word.chars().collect::<VecDeque<char>>();

            if current_char == chars.pop_front().unwrap() {

                let mut historic: HashSet< (usize, usize) > = HashSet::new();
                let mut check: Vec< (usize, usize) > = Vec::new();

                historic.insert( (i, j) );
                check.push( (i, j) );

                while !check.is_empty() {

                    let mut results: Vec< Option<_> > = Vec::new();
                    let target_char = {
                        match chars.pop_front() {
                            Some(c) => c,
                            None => return true
                        }
                    };

                    for (x, y) in check.iter() {
                        results.push( search_nearby_board( target_char, board, (*x as i32, *y as i32) ) );
                    }

                    check.clear();
                    for r in results {
                        match r {
                            Some( positions ) => {
                                for (x, y) in positions {
                                    if historic.contains( &(x, y) ) { continue; }

                                    check.push( (x, y) );
                                    historic.insert( (i, y) );
                                }
                            }
                            None => {}
                        }
                    }

                }

            }

        }
    }

    false
}

fn search_nearby_board( char: char, board: &[ [char; 4]; 3 ], pos: (i32, i32) ) -> Option< Vec< (usize, usize) > > {


    let mut positions = Vec::new();

    let mut add = |linha: i32, coluna: i32| {
        let l = linha as usize;
        let c = coluna as usize;

        if !(l < 0 || l >= board.len()) && !(c < 0 || c >= board[0].len()) {
            if board[l][c] == char { positions.push((l, c)); }
        }
    };

    { // Direita
        let linha = pos.0;
        let coluna = pos.1 + 1;
        add(linha, coluna);
    }

    { // Esquerda
        let linha = pos.0;
        let coluna = pos.1 - 1;
        add(linha, coluna);
    }

    { // Cima
        let linha = pos.0 - 1;
        let coluna = pos.1;
        add(linha, coluna);
    }

    { // Baixo
        let linha = pos.0 + 1;
        let coluna = pos.1;
        add(linha, coluna);
    }

    // Diagonais
    { // Direita + Baixo
        let linha = pos.0 + 1;
        let coluna = pos.1 + 1;
        add(linha, coluna);
    }

    { // Direita + Cima
        let linha = pos.0 - 1;
        let coluna = pos.1 + 1;
        add(linha, coluna);
    }

    { // Esquerda + Baixo
        let linha = pos.0 + 1;
        let coluna = pos.1 - 1;
        add(linha, coluna);
    }

    { // Esquerda + Cima
        let linha = pos.0 - 1;
        let coluna = pos.1 - 1;
        add(linha, coluna);
    }

    if positions.is_empty() { None }
    else { Some( positions ) }
}

fn departamento() {

    let mut departamentos: HashMap< String, Vec<String> > = HashMap::new();


    loop {

        println!( "Digite o comando do tipo: Adicione <Pessoa> para <Departamento>" );
        let mut comando = String::new();

        stdin().read_line( &mut comando ).expect("Erro ao ler entrada");
        let comando = comando.trim();

        let mut token_comando = comando.split_whitespace();

        let pessoa = match token_comando.nth(1) {
            Some( pessoa ) => pessoa,
            None => { println!("O Comando digitado não é valido"); break; }
        };

        let departamento =  match token_comando.last() {
            Some( departamento ) => departamento,
            None => { println!("O Comando digitado não é valido"); break; }
        };

        let empregados = departamentos.entry( departamento.to_string() ).or_insert( Vec::new() );
        empregados.push( pessoa.to_string() );

        println!( "O {} foi adicionado ao departamento {}", pessoa, departamento );
    }

}

fn convert_to_pig_latin( word: &String ) -> String {

    let vogais = ['a', 'e', 'i', 'o', 'u'];
    let mut new_word = word.clone();
    let first_char = word.chars().next().unwrap();

    if vogais.contains( &first_char ) { new_word.push_str( "-hay" ); }
    else {
        new_word.remove(0);
        new_word.push_str( format!("-{}ay", first_char).as_str() );
    }

    new_word
}

fn ler_numeros( path: String ) -> Vec<i32> {

    let mut file = File::open( path ).expect( "file not found" );
    let mut buffer = BufReader::new( &file );
    let mut nums: Vec<i32> = Vec::new();

    for linha in buffer.lines() {
        let value = linha.unwrap().parse::<i32>().unwrap();
        nums.push(value);
    }

    nums
}

fn moda( vec: &Vec<u8> ) -> u8 {

    let mut moda: u8 = 0;
    let nums = vec.iter().map( |v| *v ).collect::<HashSet<u8>>();

    for &n in nums.iter() {
        let sum: u8 = vec.iter().map(|v| { if *v == n { 1 } else { 0 } }).sum();
        if (sum > moda) && (sum > 1) { moda = n }
    }

    moda
}

fn mediana( vec: &Vec<u8> ) -> f32 {
    let len = vec.len();

    if len == 0 { return 0.0 }
    if len == 1 { return vec[0] as f32 }

    let mut new_vec = vec.clone();
    new_vec.sort();

    let left: u8 = new_vec[ (len / 2) - 1 ];
    let right: u8 = new_vec[ len / 2 ];

    if len % 2 == 0 {
        media( &vec![left, right] )
        // ( (left + right) as f32 ) / 2.0
    }
    else { right as f32 }
}

fn media( vec: &Vec<u8> ) -> f32 {
    let soma: u8 = vec.iter().sum();
    ( soma as f32 ) / ( vec.len() as f32 )
}