use comprimentos::cumprimentar_usuario;
use rand::{self, Rng};
use rayon::prelude::*;
use reqwest::Error;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::{Duration, Instant};
use todo::ToDoList;
use tokio::time::{self, sleep};

// Traits
trait Draw {
    fn draw(&self);
}

trait OptionExt<T> {
    fn my_unwrap_or_else<F>(self, f: F) -> T 
    where F: FnOnce() -> T;
    
    fn my_map<U, F>(self, f: F) -> Option<U>
    where F: FnOnce(T) -> U; 
}

// Structs
struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

struct Rectangle {
    pub height: f32,
    pub width: f32,
}

struct Circle {
    pub radius: f32,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

struct MyOption<T>(Option<T>);

struct MyResult<T, E>(Result<T, E>);

// Impls
impl Screen {
    pub fn run(&self) {
        for c in self.components.iter() {
            c.draw();
        }
    }
}

impl Draw for Rectangle {
    fn draw(&self) {
        println!(
            "Rectangle [ height: {}, width: {} ]",
            self.height, self.width
        )
    }
}

impl Draw for Circle {
    fn draw(&self) {
        println!("Circle [ radius: {} ]", self.radius)
    }
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut red = 0;
        let mut blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Blue => blue += 1,
                ShirtColor::Red => red += 1,
            }
        }

        if red > blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

impl<T> MyOption<T> {
    fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        match self.0 {
            Some(value) => value,
            None => f(),
        }
    }
}

impl<T, E> MyResult<T, E> {
    fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce(E) -> T,
    {
        match self.0 {
            Ok(x) => x,
            Err(e) => f(e),
        }
    }
}

impl<T> OptionExt<T> for Option<T> {
    fn my_unwrap_or_else<F>(self, f: F) -> T 
        where F: FnOnce() -> T {
        
        match self {
            Some(x) => x,
            None => f()
        }
    }

    fn my_map<U, F>(self, f: F) -> Option<U>
        where F: FnOnce(T) -> U {
        
        match self {
            Some(x) => Some( f(x) ),
            None => None
        }

    }
}

// Enums
#[derive(Debug)]
enum ShirtColor {
    Red,
    Blue,
}

#[tokio::main]
async fn main() {

    // (Tarefa) Extensão para o Enum Option
        
        let clousure1 = || 2;
        let option1_number: Option<i32> = None;
        let result1 = option1_number.my_unwrap_or_else(clousure1);
        println!("Resultado1: {result1}");    

        let clousure2 = |v: i32| v * 2;
        let option2_number: Option<i32> = Some(3);
        let result2 = option2_number.my_map(clousure2);
        println!("Resultado2: {result2:?}");

    // - - -



    // (Tarefa) Soma Parcial com Closures
    let numbers = vec![
        1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2,
        4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5,
        6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7,
        8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9,
        10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1,
        2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4,
        5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6,
        7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8,
        9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10,
        1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2,
        4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5,
        6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7,
        8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9,
        10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1,
        2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4,
        5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6,
        7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8,
        9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10,
        1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2,
        4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5,
        6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7,
        8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9,
        10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1,
        2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4,
        5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6,
        7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8,
        9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10,
        1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2,
        4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5,
        6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7,
        8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9,
        10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1,
        2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4,
        5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6,
        7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8,
        9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10,
        1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2,
        4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5,
        6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7,
        8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9,
        10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1,
        2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4,
        5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6,
        7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8,
        9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10,
        1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2,
        4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5,
        6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7,
        8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9,
        10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1,
        2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4,
        5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6,
        7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8,
        9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10,
        1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2,
        4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5,
        6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7,
        8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9,
        10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1,
        2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4,
        5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6,
        7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8,
        9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10,
        1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2,
        4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5,
        6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7,
        8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9,
        10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1,
        2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4,
        5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6,
        7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8,
        9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10,
        1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2,
        4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5,
        6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7,
        8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9,
        10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1,
        2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4,
        5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6,
        7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8,
        9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10,
        1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2,
        4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5,
        6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7,
        8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9,
        10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1,
        2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4,
        5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6,
        7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8,
        9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10,
        1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2,
        4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5,
        6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7,
        8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9,
        10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1,
        2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4,
        5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6,
        7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8,
        9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10,
        1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2,
        4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5,
        6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7,
        8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9,
        10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1,
        2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4,
        5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6,
        7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8,
        9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10,
        1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2,
        4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5,
        6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7,
        8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9,
        10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1,
        2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4,
        5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6,
        7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8,
        9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10,
        1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2,
        4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5, 6, 7, 8, 9, 10, 1, 2, 4, 5,
        6, 7, 8, 9, 10,
    ];
    let num_threads = 8;

    let chunk_size = numbers.len() / num_threads;
    let mut threads = vec![];

    for chunk in numbers.chunks(chunk_size) {
        let chunk = chunk.to_vec();

        let handle = thread::spawn(move || {
            let sum = chunk.iter().fold(0, |acc, &x| acc + x);
            sum
        });

        threads.push(handle);
    }

    let mut res = 0;
    for th in threads {
        let value = th.join().unwrap_or(0);
        res += value;
    }
    println!("Soma Parcial Finalizada: {res}");

    // ---

    // (Tarefa) Closure como Argurmento

    let callback = |err| {
        println!("Error: {err}");
        0
    };

    let ok_value = MyResult::<i32, &str>(Result::Ok(10));
    let result_ok = ok_value.unwrap_or_else(callback);
    println!("Ok Value: {result_ok}");

    let err_value = MyResult::<i32, &str>(Err("Error"));
    let result_err = err_value.unwrap_or_else(callback);
    println!("Err Value: {result_err}");

    // ---

    let fallback_value = || 42;

    let some_value = MyOption(Some(10));
    let result_some = some_value.unwrap_or_else(fallback_value);
    println!("Res: {result_some}");

    let some_value = MyOption(None);
    let result_some = some_value.unwrap_or_else(fallback_value);
    println!("Res: {result_some}");

    // ---

    // Definindo uma clousure unwrap_or_else

    let fallback_value = || 42;

    let some_value = MyOption(Some(10));
    let result_some = some_value.unwrap_or_else(fallback_value);
    println!("Res: {result_some}");

    let some_value = MyOption(None);
    let result_some = some_value.unwrap_or_else(fallback_value);
    println!("Res: {result_some}");

    // ----

    // Emprestimos / Movendo valores em Closures
    let example_closure = |x| x;
    let _s = example_closure(String::from("hello"));
    //let v = example_closure(3); // Nao pode pos a Closure ja inferiu a tipagem

    let mut list = vec![1, 2, 3];
    println!("Antes da closure: {:?}", list);
    let mut only_borrows = || list.push(4);
    only_borrows();
    println!("Depois da closure: {:?}", list);

    let list = vec![4, 5, 6];
    println!("Antes da closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    println!("Depois da closure: --- a lista nao existe mais pois foi movida");

    // ----

    // Introducao a Closures (Funcoes anonimais)
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Red],
    };

    let giveaway1 = store.giveaway(Some(ShirtColor::Red));
    let giveaway2 = store.giveaway(None);

    println!("{giveaway1:?}");
    println!("{giveaway2:?}");

    let expensive_clouse = |num| -> u32 {
        println!("Calculor lento");
        thread::sleep(Duration::from_secs(2));
        num
    };

    expensive_clouse(12);

    // ----

    // Objetos de Trait
    let screen = Screen {
        components: vec![
            Box::new(Rectangle {
                height: 1.0,
                width: 2.0,
            }),
            Box::new(Circle { radius: 5.0 }),
            Box::new(Rectangle {
                height: 10.0,
                width: 4.0,
            }),
        ],
    };

    screen.run();
    // ----

    // Tipo Generico de Dado
    let x = 5;
    let y = 10;
    println!("O maior numero eh: {}", maior(x, y));

    let x = "Bandecley";
    let y = "Vina";
    println!("O maior texto eh: {}", maior(x, y));
    // ----

    // --/> Biblioteca para To-Do-List

    let mut todo_list = ToDoList::new();

    todo_list.add_task("Lavar a louca");
    todo_list.add_task("Dar banho na nina");
    todo_list.add_task("Treinar");

    let _ = todo_list.complete_task(1);

    let tasks = todo_list.get_tasks();
    println!("Tarefas: {tasks:?}");

    // ----

    // --/> Introducao a Bibliotecas

    let result = cumprimentar_usuario();
    match result {
        Ok(user_name) => println!("Bem-Vindo, {user_name}!"),
        Err(err) => eprintln!("Error: {err}"),
    }

    // ----

    // --/> (Tarefa) Execucao Assincrona

    let tarefa1 = tokio::spawn(tarefa(1, 3));
    let tarefa2 = tokio::spawn(tarefa(2, 2));

    let _ = tokio::try_join!(tarefa1, tarefa2);

    println!();
    // ----

    // --/> Métodos Assíncronos (Parte 2)

    let solicitacao1 = tokio::spawn(fazer_solicitacao(1, "https://www.example.com"));
    let solicitacao2 = tokio::spawn(fazer_solicitacao(2, "https://www.rust-lang.org"));

    let _ = solicitacao1.await;
    let _ = solicitacao2.await;

    println!();
    // ----

    // --/> Métodos Assíncronos (Parte 1)

    let tarefa1 = tokio::spawn(tarefa_assincrona(1, 3));
    let tarefa2 = tokio::spawn(tarefa_assincrona(2, 1));

    let res1 = tarefa1.await.unwrap();
    let res2 = tarefa2.await.unwrap();

    println!("Res1: {res1:?}");
    println!("Res2: {res2:?}");

    println!();
    // ----

    // --/> (Tarfea) Numeros Primos com Paralerimos

    let numeros: Vec<u64> = (1..=1_000_000).collect();

    // Contagem Sequencial
    let start = Instant::now();
    let primos_seq: usize = numeros.iter().filter(|&&x| is_prime(x)).count();
    println!(
        "Sequencial: {} primos encontrados em {:?}",
        primos_seq,
        start.elapsed()
    );

    // Contagem Paralela
    let start = Instant::now();
    let primos_par: usize = numeros.par_iter().filter(|&&x| is_prime(x)).count();
    println!(
        "Paralelo: {} primos encontrados em {:?}",
        primos_par,
        start.elapsed()
    );

    println!();
    // ----

    // --/> Metodos Sequencias e Paraleros

    let numeros = vec![1, 2, 3, 4, 5];

    // Medindo o tempo da soma paralera
    let start_par = Instant::now();
    let soma_par: u32 = numeros.iter().par_bridge().sum(); // Soma paralera
    let duration_par = start_par.elapsed();
    println!("Soma Sequencial: {soma_par}, (Tempo: {duration_par:?}) ");

    // Medindo tempo da soma sequencial
    let start_seq = Instant::now();
    let soma_seq: u32 = numeros.iter().sum();
    let duration_seq = start_seq.elapsed();
    println!("Soma Sequencial: {soma_seq}, (Tempo: {duration_seq:?}) ");

    println!();
    // ----

    // --/> (Tarefa) Gerenciando Threads

    let (transmissor, recepetor) = mpsc::channel::<(usize, i32)>();

    let handles: Vec<_> = (0..5)
        .map(|i| {
            let tx = transmissor.clone();

            let h = thread::spawn(move || {
                let mut rng = rand::thread_rng();
                thread::sleep(Duration::from_secs(2));
                let value: i32 = rng.gen_range(0..=100);
                tx.send((i, value * value)).unwrap();
            });

            h
        })
        .collect();

    let mut values: Vec<(usize, i32)> = vec![];
    handles.into_iter().for_each(|_| {
        let value = recepetor.recv().unwrap();
        values.push(value);
    });

    println!("{values:?}");

    println!();
    // ----

    // --/> (Tarefa) Concorrencia com Valores Aleatorios

    let (transmissor, receptor) = mpsc::channel::<i32>();
    let mut values = vec![];

    thread::spawn(move || {
        let mut r = rand::thread_rng();

        for _ in 0..5 {
            transmissor.send(r.gen_range(0..=10)).unwrap();
            thread::sleep(Duration::from_secs(3));
        }
    });

    for _ in 0..5 {
        let v = receptor.recv().unwrap();
        values.push(v);
    }

    println!("Valores gerados: {values:?}");

    println!();
    // ----

    // --/> Tempo de Duracao em Threads

    let headles: Vec<_> = (0..5)
        .map(|i| {
            thread::spawn(move || {
                thread::sleep(Duration::from_secs(i));
                println!("Thread '{i}' finalizada...");
                i * i
            })
        })
        .collect();

    headles.into_iter().for_each(|h| {
        let v = h.join().unwrap();
        println!("{v}");
    });

    println!();
    // ----

    // --/> Canal de Comunicacao entre as Threads

    //Criando um canal de comunicao(transmissor e receptor)
    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || {
        let msg = String::from("Ola da thread 1");
        println!("{:?}", tx.send(msg).unwrap());
    });

    let recebido = rx.recv().unwrap();
    println!("Mensagem recebida: {recebido}");

    // Criando um contador compartilhado com Arc e Mutex
    let contador = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for i in 0..10 {
        //Criando uma referencia compartilhada usando Arc
        let c = Arc::clone(&contador);

        let h = thread::spawn(move || {
            let mut value = c.lock().unwrap();
            //println!( "{value} || {i}" );
            *value += i;
        });

        handles.push(h);
    }

    // Aguardando todas as threads terminarem
    for h in handles {
        h.join().unwrap();
    }

    println!("Contador: {}", contador.lock().unwrap());

    println!();
    // ----

    // --/> Encadeamento de Threads

    let result1 = thread::spawn(|| 42).join().unwrap();

    let result2 = thread::spawn(move || result1 * 2).join().unwrap();

    println!("Resultado final: {result2}");

    println!();
    // ----

    // --/> Exemplos de Uso das Threads

    let handle1 = thread::spawn(|| {
        hello_thread(1);
    });

    let handle2 = thread::spawn(|| {
        hello_thread(2);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Fim das Threads...");

    // A funcao join do JoinHandle, ira retorna um result, que caso de tudo certo na
    // "esperar" na thread atual, ele ira retorna um Ok com o retorno da Thread execuatda
    // Caso aconteca algum erro, ele ira retorna um Err

    let result = thread::spawn(|| Err::<i32, &str>("Algo deu errado"))
        .join()
        .unwrap();

    match result {
        Ok(value) => println!("Thread bem sucedida com valor: {value:?}"),
        Err(err) => println!("Erro na thread: {err:?}"),
    }

    println!();
    // ----

    // --/> Introdução as Threads

    thread::spawn(|| {
        println!("Segunda Thread");
    });

    println!("Thread principal");

    // ----
}

fn maior<T>(a: T, b: T) -> T
where
    T: Ord,
{
    if a > b { a } else { b }
}

async fn tarefa(id: u32, duracao: u64) {
    println!("Iniciando a tarefa: {id}");

    sleep(time::Duration::from_secs(duracao)).await;

    println!("tarefa {id} foi concluida apos {duracao} segundos");
}

async fn fazer_solicitacao(id: u32, url: &str) -> Result<(), Error> {
    println!(" Iniciando a solicitação '{id}' para '{url}' ");

    sleep(time::Duration::from_secs(2)).await;

    let resposta = reqwest::get(url).await?;

    println!("RESPONSE '{id}': STATUS: {:?}", resposta.status());

    Ok(())
}

async fn tarefa_assincrona(id: u32, duracao: u64) {
    println!("Iniciando a tarfea: '{id}'");

    // Atraso propozital
    sleep(time::Duration::from_secs(duracao)).await;

    println!("Fim da tarfea: '{id}'");
}

fn is_prime(num: u64) -> bool {
    if num < 2 {
        return false;
    }

    for n in 2..=((num as f64).sqrt() as u64) {
        if num % n == 0 {
            return false;
        }
    }

    true
}

fn hello_thread(id: i32) {
    println!("Bem vindo a Thread: {id}");
}
