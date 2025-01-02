use listas::Lista;

#[test]
fn test_adicionar_e_buscar() {
    let mut l1 = Lista::<i32>::nova();
    l1.adicionar(5);
    l1.adicionar(10);
    l1.adicionar(15);

    assert_eq!(l1.buscar(&10), Some(1));
}

#[test]
fn test_remover_e_buscar() {
    let mut l1 = Lista::<i32>::nova();
    l1.adicionar(5);
    l1.adicionar(10);
    l1.adicionar(15);

    l1.remove(2);

    assert_eq!(l1.buscar(&15), None);
}
