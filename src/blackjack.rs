use rand::seq::SliceRandom;
use rand::thread_rng;
use std::{io::{self, Write}, ops::Index};
use crossterm::event::{read, Event, KeyCode, KeyEventKind};
use std::{thread, time};


#[derive(Debug, PartialEq)]
pub enum GameState {
    Setup, // Number of players and the bet amount for each
    DealingCards, // Cards distribution with animation on TUI
    PlayersTurn, // Players choose to stay or hit a card
    DealerTurn, // Show the 2nd card of the dealer and has to hit if less than 16
    Result, // Result who lost / win and restart the game with same bet amount
}

#[derive(Debug, Clone)]
pub struct Card {
    pub value: Value,
    pub color: Color,
}

impl Card {
    pub fn new(value: Value, color: Color) -> Self {
        Self { value, color }
    }

    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn color(&self) -> &Color {
        &self.color
    }
}

#[derive(Debug, Clone)]
pub enum Color {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl Color {
    pub fn to_symbol(&self) -> &str {
        match self {
            Color::Hearts => "♥",
            Color::Diamonds => "♦", 
            Color::Clubs => "♣",
            Color::Spades => "♠",
        }
    }
    
    pub fn to_color(&self) -> ratatui::style::Color {
        match self {
            Color::Hearts | Color::Diamonds => ratatui::style::Color::Red,
            Color::Clubs | Color::Spades => ratatui::style::Color::White,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
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
    pub fn to_int(self) -> u8 {
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
            _other => 10
        }
    }
    
    pub fn to_string(&self) -> &str {
        match self {
            Value::Ace => "A",
            Value::Two => "2",
            Value::Three => "3",
            Value::Four => "4", 
            Value::Five => "5",
            Value::Six => "6",
            Value::Seven => "7",
            Value::Eight => "8",
            Value::Nine => "9",
            Value::Ten => "10",
            Value::Jack => "J",
            Value::Queen => "Q",
            Value::King => "K",
        }
    }
}
#[derive(Debug)]
pub struct Deck {
    pub cards: Vec<Card>,
    pub length: usize,
    pub discarded: Vec<Card>,
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
            discarded: Vec::new()
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

    pub fn shuffle(&mut self) {
        println!("Mélange du deck...");
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }


    fn draw(&mut self) -> Option<Card> {
        if self.length == 0 {
            println!("Le deck est vide, on réutilise les cartes précédemment jouées !");
            self.cards.append(&mut self.discarded);
            self.shuffle();
            self.length = self.cards.len();
        }
        self.length -= 1;
        self.cards.pop()
    }

    pub fn discard_hand(&mut self, hand: &mut Hand) {
        self.discarded.append(&mut hand.cards);
        hand.clear();
    }
}

#[derive(Debug, Clone)]
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    pub fn cards(&self) -> &Vec<Card> {
        &self.cards
    }

    fn new() -> Self {
        Self { cards: Vec::new() }
    }

    fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn hit_cards(&mut self, deck: &mut Deck) {
        if let Some(card) = deck.draw() {
            self.add_card(card);
        }
    }

    pub fn value(&self) -> u8 {
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
    pub bank: u32,
    pub bet: u32,
    pub who: Who,
    pub win: i8, // -1 = perdu, 0 = égalité, 1 = gagné, 2 = blackjack
}

impl Player {
    pub fn new() -> Self {
        Self {
            hand: Hand::new(),
            bank: 1000,
            bet: 10,
            who: Who::Player,
            win: 0,
        }
    }

    pub fn reset(&mut self) {
        self.hand.clear();
        self.win = 0;
    }

    pub fn set_bet(&mut self, amount: u32) {
        self.bet = amount
    }

    pub fn show_updated_player_bank(&self){
        println!("{}", self.bank)
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
}

impl Game {
    pub fn new() -> Self {
        let deck = Deck::initialize_deck();
        let players = Vec::new();
        let dealer = Dealer::new();
        Self { deck, players, dealer}
    }

    pub fn ask_initial_bets(&mut self) {
        println!("\n--- Mise initiale des joueurs ---");
        for (i, player) in self.players.iter_mut().enumerate() {
            loop {
                print!("Entrez la mise pour le joueur {} (entre 10 et 100): ", i + 1);
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                if let Ok(bet) = input.trim().parse::<u32>() {
                    if bet >= 10 && bet <= 100 && bet <= self.dealer.bank {
                        player.bet = bet;
                        self.dealer.bank -= bet; // retirer la mise de la banque du croupier
                        break;
                    }
                }
                println!("Mise invalide. Veuillez entrer un montant valide et disponible dans la banque du dealer.");
            }
        }
        println!("Toutes les mises ont été placées !");
    }

    pub fn deal_cards(&mut self) {
        // Nettoyer les mains précédentes
        for player in &mut self.players {
            player.hand.clear();
            player.win = 0; // reset état du joueur
        }
        self.dealer.hand.clear();
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
                            
                                break;
                            }
                            _ => {}
                        }
                        if player.hand.value() == 21 {
                            println!("BLACKJACK 🤑💰💲");
                            player.win = 2;
                            break;
                        }
                        continue;
                    }
                }
            }
        }
    }

    pub fn dealer_turn(&mut self) {
        println!("\n\nMain du dealer:");
        self.dealer.hand.show_value();

        
        // 1) Marquer immédiatement les joueurs qui ont déjà busté
        for player in &mut self.players {
            if player.hand.value() > 21 && player.win == 0 {
                player.win = -1;
                // Appliquer le résultat directement ici
                match player.win {
                    -1 => { /* déjà perdu, rien à ajouter */ }
                    0 => { player.bank += player.bet; }
                    1 => { player.bank += 2 * player.bet; }
                    2 => { player.bank += (player.bet * 5) / 2; }
                    _ => {}
                }
    }
}

        // Si tout le monde a perdu -> on s'arrête
        if self.players.iter().all(|p| p.win == -1) {
            println!("Tous les joueurs ont perdu. Le dealer n'a pas besoin de jouer.");
            return;
        }

        // Tour du croupier
        loop {
            let dealer_value = self.dealer.hand.value();

            if dealer_value < 17 {
                self.dealer.hand.hit_cards(&mut self.deck);
                self.dealer.hand.show_value();
                thread::sleep(time::Duration::from_secs(1));
            } else if dealer_value > 21 {
                println!("Le dealer a dépassé 21, il a perdu.");
                for player in self.players.iter_mut() {
                    if player.win == 0 { // joueur encore "en lice"
                        player.win = 1;
                    }
                }
                break;
            } else if dealer_value == 21 {
                println!("Le dealer a 21 !");
                for player in self.players.iter_mut() {
                    if player.win == 0 { // joueur encore "en lice"
                        if player.hand.value() == 21 {
                            player.win = 0; // égalité
                        } else {
                            player.win = -1; // perdu
                        }
                    }
                }
                break;
            } else { // dealer_value >= 17 && dealer_value < 21
                println!("Main finale du dealer:");
                self.dealer.hand.show_value();
                thread::sleep(time::Duration::from_secs(1));

                println!("\nRésultats:");
                for (i, player) in self.players.iter_mut().enumerate() {
                    if player.win != 0 {
                        continue; // déjà fixé (blackjack, bust, etc.)
                    }
                    if player.hand.value() > dealer_value && player.hand.value() <= 21 {
                        println!("Le joueur {} a gagné contre le dealer.", i + 1);
                        player.win = 1;
                    } else if player.hand.value() < dealer_value {
                        println!("Le dealer a gagné contre le joueur {}.", i + 1);
                        player.win = -1;
                    } else {
                        println!("Égalité entre le dealer et le joueur {}.", i + 1);
                        player.win = 0;
                    }
                }
                break;
            }
        }

        println!("\nFin de la partie.");
        self.apply_results();
        for (i, player) in &mut self.players.iter_mut().enumerate() {
            println!("Bank du joueur {}", i  + 1);
            player.show_updated_player_bank();
        }
    }
    
    pub fn add_players(&mut self, count: usize) {
        self.players.clear();
        for _ in 0..count {
            let player = Player::new();
            self.players.push(player);
        }
    }

    pub fn apply_results(&mut self) {
        for player in &mut self.players {
            player.bank -= player.bet;
            match player.win {
                -1 => { /* perdu : mise déjà retirée dans initial_bet() */ }
                0 => { player.bank += player.bet; }        // égalité : récupère sa mise
                1 => { player.bank += 2 * player.bet; }    // gagné : double la mise
                2 => { player.bank += (player.bet * 5) / 2; } // blackjack : 2.5x la mise
                _ => {}
            }
        }
    }

    pub fn discard_all_hands(&mut self) {
        for player in &mut self.players {
            self.deck.discard_hand(&mut player.hand);
        }
        self.deck.discard_hand(&mut self.dealer.hand);
    }

    pub fn play_round(&mut self) {
        self.deck.shuffle();
        self.ask_initial_bets();
        self.deal_cards();
        self.players_turns();
        self.dealer_turn();
        self.discard_all_hands(); // remet les cartes jouées dans discarded

        loop {
            print!("Voulez-vous relancer une autre partie ? (o/n) : ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            match input.trim().to_lowercase().as_str() {
                "o" => break self.play_round(),
                "n" => {
                    println!("Fin de la session de jeu !");
                    return;
                },
                _ => println!("Entrée invalide"),
            }
        }
    }

    pub fn play_next_round(&mut self) {
        println!("\n\n\n\n\n\n\n\n\n\n\n\n--- Nouveau tour ---");

        // Remettre les cartes jouées dans le deck
        self.discard_all_hands();

        // Réinitialiser les joueurs (mais conserver la mise)
        for player in &mut self.players {
            player.win = 0;
        }

        // Rejouer le tour
        self.deck.shuffle();
        self.deal_cards();
        self.players_turns();
        self.dealer_turn();
    }
}

    // ajoute fonctionnalité de temps avec un timer pour prise de décision des joueurs
    // utiliser ratatui pour avoir une interface utilisateur
    // compteur de joueur avec blackjack, si le croupier a blackjack aussi il y a égalité

pub fn test(){
    let mut game = Game::new();
    
    game.ask_nb_players();  // nombre de joueurs
    game.ask_initial_bets(); // mises initiales

    loop {
        thread::sleep(time::Duration::from_secs(1));
        game.play_next_round();

        // Vérifie si tous les joueurs ou le dealer sont à sec
        if game.players.iter().all(|p| p.bank == 0) {
            println!("Tous les joueurs sont à court d'argent !");
            break;
        }
        if game.dealer.bank == 0 {
            println!("Le dealer est à court d'argent !");
            break;
        }

        println!("Recommence automatiquement le prochain tour avec les mêmes mises...");
        thread::sleep(time::Duration::from_secs(2));
    }

    println!("Fin de la session !");
}

pub fn main() {
    let mut game = Game::new();
    
    game.ask_nb_players();  // nombre de joueurs
    game.play_round();       // lance le premier tour et propose de relancer
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