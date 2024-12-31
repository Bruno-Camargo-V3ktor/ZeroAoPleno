use strsplit::StrSplit;

#[test]
fn it_works() {
    let haystack = "a b c d e";
    for letter in StrSplit::new(haystack, " ") {
        // a, b, c, d, e
        println!("{letter}");
    }

    assert!(StrSplit::new(haystack, " ").eq(vec!["a", "b", "c", "d", "e"].into_iter()))
}
