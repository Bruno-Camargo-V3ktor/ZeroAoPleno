pub struct Card<'a, VALUE> {
    pub value: &'a VALUE,
    pub suit: Suit,
}

pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

pub struct Deck<'a, VALUE> {
    pub cards: Vec<Card<'a, VALUE>>,
}

impl<'a, VALUE> Deck<'a, VALUE> {
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }

    pub fn add_cards(&mut self, card: Card<'a, VALUE>) {
        self.cards.push(card);
    }
}
