use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Clone)]
struct Card {
    value: Value,
    color: Color,
}
#[derive(Debug, Clone)]
enum Color {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}
#[derive(Debug, Clone, PartialEq)]
enum Value {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Value {
    fn to_int(self) -> u8 {
        match self {
            Self::Ace => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
            Self::Six => 6,
            Self::Seven => 7,
            Self::Eight => 8,
            Self::Nine => 9,
            other => 10
        }
    }
}
#[derive(Debug)]
struct Deck {
    cards: Vec<Card>,
    length: usize,
}

impl Deck {
    fn initialize_deck() -> Self {
        let colors = vec![
            Color::Hearts,
            Color::Diamonds,
            Color::Clubs,
            Color::Spades,
        ];
        let values = vec![
            Value::Ace, Value::Two, Value::Three, Value::Four, Value::Five, Value::Six,
            Value::Seven, Value::Eight, Value::Nine, Value::Ten,
            Value::Jack, Value::Queen, Value::King,
        ];
        let mut deck = Deck {
            cards: Vec::new(),
            length: 0,
        };
        for color in colors {
            for value in &values {
                deck.cards.push(Card { value: value.clone(), color: color.clone() });
                deck.length += 1;
            }
        }
        deck
    }

    fn shuffle(&mut self) {
        
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }


    fn draw(&mut self) -> Option<Card> {
        if self.length == 0 {
            None
        } else {
            self.length -= 1;
            self.cards.pop()
        }
    }
}

#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn new() -> Self {
        Self { cards: Vec::new() }
    }

    fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn value(&self) -> u8 {
        let mut somme = self.cards.iter().map(|c| c.value.clone().to_int()).sum();
        let number_of_aces = self.cards.iter().filter(|c| c.value == Value::Ace).count();
        if somme <= 11 && number_of_aces > 0 {
            somme += 10;
        }
        somme
    }

    fn get_value(&self) -> u8 {
        self.value()
    }

    fn clear(&mut self) {
        self.cards.clear();
    }
}

fn main() {
    let mut deck = Deck::initialize_deck();
    deck.shuffle();
    println!("{:?}", deck.cards);
}