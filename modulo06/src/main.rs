use std::thread;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;

use rand::{self, Rng};

fn main() {

    // --/> (Tarefa) Concorrencia com Valores Aleatorios

    let ( transmissor, receptor ) = mpsc::channel::<i32>();
    let mut values = vec![];

    thread::spawn( move || {

        let mut r = rand::thread_rng();

        for _ in 0..5 {
            transmissor.send( r.gen_range( 0..=10 ) ).unwrap();
            thread::sleep( Duration::from_secs(3) );
        }

    });


    for _ in 0..5 {
        let v = receptor.recv().unwrap();
        values.push( v );
    }

    println!( "Valores gerados: {values:?}" );

    println!();
    // ----


    // --/> Tempo de Duracao em Threads

    let headles: Vec<_> = (0..5).map( |i| {
        thread::spawn( move || {
            thread::sleep( Duration::from_secs(i) );
            println!( "Thread '{i}' finalizada..." );
            i*i
        })
    }).collect();

    headles.into_iter().for_each( |h| {
        let v = h.join().unwrap();
        println!( "{v}" );
    } );

    println!();
    // ----


    // --/> Canal de Comunicacao entre as Threads

    //Criando um canal de comunicao(transmissor e receptor)
    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn( move || {
        let msg = String::from("Ola da thread 1");
        println!( "{:?}", tx.send( msg ).unwrap() );
    } );

    let recebido = rx.recv().unwrap();
    println!( "Mensagem recebida: {recebido}" );

    // Criando um contador compartilhado com Arc e Mutex
    let contador = Arc::new( Mutex::new(0) );

    let mut handles = vec![];

    for i in 0..10 {
        //Criando uma referencia compartilhada usando Arc
        let c = Arc::clone( &contador );

        let h = thread::spawn(move || {
            let mut value = c.lock().unwrap();
            //println!( "{value} || {i}" );
            *value += i;
        });

        handles.push( h );
    }

    // Aguardando todas as threads terminarem
    for h in handles { h.join().unwrap(); }

    println!( "Contador: {}", contador.lock().unwrap() );

    println!();
    // ----


    // --/> Encadeamento de Threads

    let result1 = thread::spawn( || {
        42
    } ).join().unwrap();

    let result2 = thread::spawn( move || {
        result1 * 2
    } ).join().unwrap();

    println!( "Resultado final: {result2}" );

    println!();
    // ----


    // --/> Exemplos de Uso das Threads

    let handle1 = thread::spawn( || {
        hello_thread(1);
    } );

    let handle2 = thread::spawn( || {
        hello_thread(2);
    } );

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!( "Fim das Threads..." );

    // A funcao join do JoinHandle, ira retorna um result, que caso de tudo certo na
        // "esperar" na thread atual, ele ira retorna um Ok com o retorno da Thread execuatda
        // Caso aconteca algum erro, ele ira retorna um Err

    let result = thread::spawn( || {
        Err::<i32, &str>("Algo deu errado")
    } ).join().unwrap();

    match result {
        Ok( value ) => println!( "Thread bem sucedida com valor: {value:?}" ),
        Err( err ) => println!( "Erro na thread: {err:?}" )
    }

    println!();
    // ----


    // --/> Introdução as Threads

    thread::spawn( || {
        println!("Segunda Thread");
    } );

    println!("Thread principal");

    // ----

}

fn hello_thread(id: i32) {
    println!("Bem vindo a Thread: {id}");
}
