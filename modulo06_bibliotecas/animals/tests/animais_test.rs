use animals::{Animal, Cachorro, Gato};

#[test]
fn test_cachorro_faz_barulho() {
    let cachorro: Box<dyn Animal> = Box::new(Cachorro {});

    assert_eq!(cachorro.fazer_barulho(), ())
}

#[test]
fn test_gato_faz_barulho() {
    let gato: Box<dyn Animal> = Box::new(Gato {});

    assert_eq!(gato.fazer_barulho(), ())
}
