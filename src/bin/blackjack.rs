use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Clone)]
struct Card {
    value: Value,
    color: Color,
}

impl Card {
    fn new(value: Value, color: Color) -> Self {
        Self { value, color }
    }

    fn value(&self) -> &Value {
        &self.value
    }

    fn color(&self) -> &Color {
        &self.color
    }
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
        for _ in 0..6 { // 6 jeux de 52 cartes
            for color in &colors {
                for value in &values {
                    deck.cards.push(Card { value: value.clone(), color: color.clone() });
                    deck.length += 1;
                }
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
        if somme <= 11 && number_of_aces > 0 { // Si la somme est inférieure ou égale à 11 et qu'il y a des As
            somme += 10;
        }
        somme
    }

    // Ajoute un paramètre is_dealer
    fn show_value(&self, is_dealer: bool) -> u8 {
        if is_dealer {
            if let Some(card) = self.cards.first() {
                let val = card.value.clone().to_int();
                println!("{} + ?: [{:?} of {:?} + ?]", val, card.value, card.color);
                val
            } else {
                println!("Main vide");
                0
            }
        } else {
            let val = self.value();
            println!("{}: [{}]", val, self.cards.iter().map(|c| format!("{:?} of {:?}", c.value, c.color)).collect::<Vec<String>>().join(", "));
            val
        }
    }

    fn clear(&mut self) {
        self.cards.clear();
    }
}
#[derive(Debug)]
enum Who {
    Player,
    Dealer,
}
#[derive(Debug)]
struct Player {
    hand: Hand,
    money: u32,
    bet: u32,
    who: Who,
}

impl Player {
    fn new(who: Who) -> Self {
        Self {
            hand: Hand::new(),
            money: 1000,
            bet: 10,
            who,
        }
    }
}
#[derive(Debug)]
struct Dealer {
    hand: Hand,
    bank: u32,
}

impl Dealer {
    fn new() -> Self {
        Self {
            hand: Hand::new(),
            bank: 100000,
        }
    }
}
#[derive(Debug)]
struct Game {
    deck: Deck,
    players: Vec<Player>,
    dealer: Dealer,
}

impl Game {
    fn new() -> Self {
        let deck = Deck::initialize_deck();
        let players = Vec::new();
        let dealer = Dealer {
            hand: Hand::new(),
            bank: 1000,
        };
        Self { deck, players, dealer }
    }
    fn deal_cards(&mut self) {
        for _ in 0..2 {
            for player in &mut self.players {
                if let Some(card) = self.deck.draw() {
                    player.hand.add_card(card);
                }
            }
            if let Some(card) = self.deck.draw() {
                self.dealer.hand.add_card(card);
            }
        }
    }
    fn show_hands(&self) {
        for player in &self.players {
            player.hand.show_value(false);
        }
        self.dealer.hand.show_value(true);
    }
}

fn main() {
    let mut player1 = Player::new(Who::Player);
    let mut player2 = Player::new(Who::Player);
    let mut game = Game::new();

    game.deck.shuffle(); // Mélanger les cartes
    game.players.push(player1); // Ajoute le joueur à la partie
    game.players.push(player2); // Ajoute le deuxième joueur à la partie
    game.deal_cards(); // Distribue les cartes
    game.show_hands(); // Montre les mains des joueurs et du croupier
    
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_deck_initialization() {
        let deck = Deck::initialize_deck();
        assert_eq!(deck.length, 312);
    }

    #[test]
    fn test_ace_value() {
        let ace_one = Card::new(Value::Ace, Color::Hearts);
        let four = Card::new(Value::Four, Color::Diamonds);
        let mut hand_one = Hand::new();
        hand_one.add_card(ace_one);
        hand_one.add_card(four);


        let ace_ten = Card::new(Value::Ace, Color::Spades);
        let nine = Card::new(Value::Nine, Color::Clubs);
        let five = Card::new(Value::Five, Color::Hearts);
        let mut hand_ten = Hand::new();
        hand_ten.add_card(ace_ten);
        hand_ten.add_card(nine);
        hand_ten.add_card(five);

        assert_eq!(hand_one.value(), 15);
        assert_eq!(hand_ten.value(), 15);
        
    }
}