use rand::seq::SliceRandom;
use rand::thread_rng;
use std::{io::{self, Write}, ops::Index};
use crossterm::event::{read, Event, KeyCode, KeyEventKind};
use std::{thread, time};


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
pub struct Deck {
    pub cards: Vec<Card>,
    pub length: usize,
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
        println!("Mélange du deck...");
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

    fn hit_cards(&mut self, deck: &mut Deck) {
        if let Some(card) = deck.draw() {
            self.add_card(card);
        }
    }

    fn value(&self) -> u8 {
        let mut somme = self.cards.iter().map(|c| c.value.clone().to_int()).sum();
        let number_of_aces = self.cards.iter().filter(|c| c.value == Value::Ace).count();
        if somme <= 11 && number_of_aces > 0 { // Si la somme est inférieure ou égale à 11 et qu'il y a des As
            somme += 10;
        }
        somme
    }

    fn show_value(&self) -> u8 {
        let value = self.value();
        println!("{}: [{}]", value, self.cards.iter().map(|c| format!("{:?} of {:?}", c.value, c.color)).collect::<Vec<String>>().join(", "));
        value
    }

    fn show_value_dealer(&self) -> u8 {
        if let Some(card) = self.cards.first() {
            let val = card.value.clone().to_int();
            println!("{} + ?: [{:?} of {:?} + ?]", val, card.value, card.color);
            val
        } else {
            println!("Main vide");
            0
        }
    }

    fn clear(&mut self) {
        self.cards.clear();
    }

}

#[derive(Debug)]
pub enum Who {
    Player,
    Dealer,
}


#[derive(Debug)]
pub struct Player {
    pub hand: Hand,
    pub money: u32,
    pub bet: u32,
    pub who: Who,
}

impl Player {
    pub fn new() -> Self {
        Self {
            hand: Hand::new(),
            money: 1000,
            bet: 10,
            who: Who::Player,
        }
    }
}
#[derive(Debug)]
pub struct Dealer {
    pub hand: Hand,
    pub bank: u32,
    pub who: Who,
}

impl Dealer {
    pub fn new() -> Self {
        Self {
            hand: Hand::new(),
            bank: 100000,
            who: Who::Dealer,
        }
    }
}



#[derive(Debug)]
pub struct Game {
    pub deck: Deck,
    pub players: Vec<Player>,
    pub dealer: Dealer,
    pub counter: u32
}

impl Game {
    pub fn new() -> Self {
        let deck = Deck::initialize_deck();
        let players = Vec::new();
        let dealer = Dealer::new();
        let mut counter = 0;
        Self { deck, players, dealer, counter }
    }

    fn deal_cards(&mut self) {
        println!("\nDistribution des cartes...\n");
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
            player.hand.show_value();
        }
        self.dealer.hand.show_value_dealer();
    }

    fn ask_nb_players(&mut self) {
        // Demande du nombre de joueur
        let num_players = loop {
            print!("Combien de joueurs ? (1-7): ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            if let Ok(n) = input.trim().parse::<usize>() {
                if n >= 1 && n <= 7 {
                    break n;
                }
            }
            println!("Entrée invalide. Veuillez entrer un nombre entre 1 et 7.");
        };
        
        // Création du nombre de joueurs
        for _ in 0..num_players {
            let player = Player::new();
            self.players.push(player);
        }
        thread::sleep(time::Duration::from_secs(1));
    }

    fn players_turns(&mut self) {
        self.dealer.hand.show_value_dealer();
        println!("\n\nAppuie sur 't' pour tirer ou 'r' pour rester :");
        for (i, player) in &mut self.players.iter_mut().enumerate() {
            println!("\nMain du joueur {}:", i + 1);
            player.hand.show_value();
            loop {
                if let Event::Key(event) = read().unwrap() {
                    if event.kind == KeyEventKind::Press {
                        match event.code {
                            KeyCode::Char('t') => {
                                player.hand.hit_cards(&mut self.deck);
                                player.hand.show_value();
                                if player.hand.value() > 21 {
                                    println!("\nTour terminé");
                                    println!("Vous avez dépassé 21, vous perdez.");
                                    thread::sleep(time::Duration::from_secs(2));
                                    break;
                                } else {
                                    continue
                                }
                            }
                            KeyCode::Char('r') => {
                                player.hand.show_value();
                                println!("Tour terminé\n");
                                self.counter += 1;
                                break;
                            }
                            _ => {}
                        }
                        if player.hand.value() == 21 {
                            println!("BLACKJACK 🤑💰💲");
                            break;
                        }
                        continue;
                    }
                }
            }
        }
    }

    fn dealer_turn(&mut self){
        println!("\n\nMain du dealer:");
        self.dealer.hand.show_value();
        loop {
            // si counter = 0, tout les joueurs ont perdu et le dealer n'a pas besoin de jouer
            if self.counter == 0 {
                println!("Tous les joueurs ont perdu. Le dealer n'a pas besoin de jouer.");
                break;
            }
            // si counter supérieur à 0
            // si la valeur de sa main est inférieur à 17 il doit tirer
            if self.counter > 0 && self.dealer.hand.value() < 17 {
                self.dealer.hand.hit_cards(&mut self.deck);
                self.dealer.hand.show_value();
                thread::sleep(time::Duration::from_secs(1));
            } else if self.dealer.hand.value() > 21 {// si la valeur de sa main dépasse 21, le dealer a perdu
                println!("Le dealer a dépassé 21, il a perdu.");
                break;
            } else if self.dealer.hand.value() == 21 { // si la valeur de sa main est 21, le dealer a gagné, si aucun autre joueur à 21
                println!("Le dealer a 21, il a gagné.");
                break;
            } else if self.dealer.hand.value() >= 17 && self.dealer.hand.value() < 21 { // si la valeur de sa main est sup 16 et inférieur à 21, le dealer reste
                println!("Main final du dealer:");
                self.dealer.hand.show_value();
                thread::sleep(time::Duration::from_secs(1));
                println!("\nRésultats:");
                for (i, player) in (&self.players).iter().enumerate() {
                    if player.hand.value() > self.dealer.hand.value() && player.hand.value() <= 21 {
                        println!("\nLe joueur {} a gagné contre le dealer.", i+1);
                    } else if player.hand.value() < self.dealer.hand.value() && self.dealer.hand.value() <= 21 {
                        println!("\nLe dealer a gagné contre le joueur {}.", i+1);
                        self.counter -= 1;
                    } else if player.hand.value() == self.dealer.hand.value() {
                        println!("\nÉgalité entre le dealer et le joueur {}.", i+1);
                    }
                }
                break;
            } 
        }
        println!("\nFin de la partie.");
        for player in &mut self.players {
            player.hand.clear();
        }
        self.dealer.hand.clear();
    }

    // ajouter fonctionnalité recuperer les cartes utilisés et au bout d'un moment les réutiliser sinon le deck sera vide
    // ajouter fonctionnalité pour relancer une partie des que la partie precedente est fini
    // ajouter fonctionnalité pour afficher les gains et les pertes des joueurs
    // ajouter fonctionnalité pour gérer les mises des joueurs
    // ajoute fonctionnalité de temps avec un timer pour prise de décision des joueurs
    // utiliser ratatui pour avoir une interface utilisateur

}
pub fn test(){
    let mut game = Game::new();
    game.deck.shuffle();
    game.ask_nb_players();
    game.deal_cards();
    game.players_turns();
    game.dealer_turn();
}

fn main() {
    test();
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
    #[test]
    fn test_is_player(){
        let mut player = Player::new();
        player.hand.add_card(Card::new(Value::Ace, Color::Hearts));
        player.hand.add_card(Card::new(Value::King, Color::Spades));
        assert_eq!(player.hand.show_value(), 21);
    }
}