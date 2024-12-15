use std::thread;

fn main() {

    // --/> Introdução as Threads

    thread::spawn( || {
        println!("Segunda Thread");
    } );

    println!("Thread principal");

    // ----

}
