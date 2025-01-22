use repositorio::Repositorio;

#[test]
fn test_soma_int() {
    let mut repo: Repositorio<i32> = Repositorio::new();
    repo.adicionar(4);
    repo.adicionar(1);
    repo.adicionar(5);
    repo.adicionar(10);

    assert_eq!(repo.total(), 20)
}

#[test]
fn test_produto_float() {
    let mut repo: Repositorio<f64> = Repositorio::new();
    repo.adicionar(4.0);
    repo.adicionar(1.0);
    repo.adicionar(5.0);
    repo.adicionar(10.0);

    assert_eq!(repo.produto(), 200.0)
}
