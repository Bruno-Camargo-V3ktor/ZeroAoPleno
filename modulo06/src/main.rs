use std::thread;

fn main() {


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
