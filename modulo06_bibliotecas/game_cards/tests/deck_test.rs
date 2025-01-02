use game_cards::{Card, Deck, Suit};

#[test]
fn test_deck() {
    let mut deck: Deck<i32> = Deck::new();

    let card = Card {
        value: &10,
        suit: Suit::Hearts,
    };

    deck.add_cards(card);

    assert_eq!(deck.cards.len(), 1);
}
