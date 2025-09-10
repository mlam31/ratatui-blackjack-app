use rand::seq::SliceRandom;
use rand::thread_rng;



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
    pub fn to_int(&self) -> u8 {
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
    pub discarded: Vec<Card>,
}

impl Deck {
    pub fn initialize_deck() -> Self {
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
        let mut cards = Vec::with_capacity(6 * 52);
        for _ in 0..6 { // 6 jeux de 52 cartes
            for color in &colors {
                for value in &values {
                    cards.push(Card { value: value.clone(), color: color.clone() });
                }
            }
        }
        Deck {cards, discarded: Vec::new()}
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    /// Tire une carte (si le paquet est vide, on replace les cartes défaussées)
    fn draw(&mut self) -> Option<Card> {
        if self.cards.is_empty() {
            if self.discarded.is_empty(){
                return None
            }
            // récupère les cartes défaussées, mélange et continue
            self.cards.append(&mut self.discarded);
            self.shuffle();
        }
        self.cards.pop()
    }

    /// déplace les cartes d'une main vers discarded et vide la main
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
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }

    pub fn cards(&self) -> &Vec<Card> {
        &self.cards
    }


    pub fn add_card(&mut self, card: Card) {
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

    pub fn show_value(&self) -> u8 {
        let value = self.value();
        //println!("{}: [{}]", value, self.cards.iter().map(|c| format!("{:?} of {:?}", c.value, c.color)).collect::<Vec<String>>().join(", "));
        value
    }

    pub fn show_value_dealer(&self) -> u8 {
        if let Some(card) = self.cards.first() {
            card.value.to_int()
        } else {
            0
        }
    }

    pub fn clear(&mut self) {
        self.cards.clear();
    }

    pub fn is_blackjack(&self) -> bool {
        self.cards.len() == 2 && self.value() == 21
    }
    
    pub fn is_bust(&self) -> bool {
        self.value() > 21
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
    pub finished: bool,
}

pub enum PlayerActionResult {
    Continue(u8),    // valeur de la main
    Bust(u8),        // dépassé 21
    Blackjack,       // exactement 21
    InvalidAction,   // ne peut pas jouer
}

impl Player {
    pub fn new() -> Self {
        Self {
            hand: Hand::new(),
            bank: 1000,
            bet: 10,
            who: Who::Player,
            win: 0,
            finished: false,
        }
    }

    pub fn reset(&mut self) {
        self.hand.clear();
        self.win = 0;
        self.finished = false
    }

    pub fn set_bet(&mut self, amount: u32) {
        if amount <= self.bank {
            self.bet = amount;
        }
    }

    pub fn show_updated_player_bank(&self){
        println!("{}", self.bank)
    }

    pub fn reset_for_new_round(&mut self) {
        self.reset();
    }
}
#[derive(Debug)]
pub struct Dealer {
    pub hand: Hand,
    pub bank: u32,
    pub who: Who,
}

pub enum DealerActionResult {
        Continue(u8),
        Stand(u8),
        Bust(u8),
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
        let mut deck = Deck::initialize_deck();
        deck.shuffle();
        let players = Vec::new();
        let dealer = Dealer::new();
        Self { deck, players, dealer}
    }

    pub fn create_players(&mut self, count: usize) -> Result<(), String> {
        self.players.clear();
        if count < 1 || count > 7 {
            return Err("Nombre de joueurs invalide".to_string());
        }
        for _ in 0..count{
            self.players.push(Player::new())
        }
        Ok(())
    }
    
    pub fn set_player_bet(&mut self, player_index: usize, amount: u32) -> Result<(), String> {
        if player_index >= self.players.len() {
            return Err("Invalid player index".to_string());
        }
        if amount > self.players[player_index].bank {
            Err("Bet amount superior to player's bank".to_string())
        } else {
            self.players[player_index].bet = amount;
            Ok(())
        }
    }

    pub fn deal_cards(&mut self) {
        // Nettoyer les mains précédentes
        for player in &mut self.players {
            player.reset()
        }
        self.dealer.hand.clear();

        // Distribution des cartes
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
    
    pub fn can_player_play(&self, player_index: usize) -> bool {
        if player_index >= self.players.len() {
            return false;
        }
        let p = &self.players[player_index];
        (!p.finished) && !p.hand.is_bust() && !p.hand.is_blackjack()
    }
    
    pub fn player_hit(&mut self, player_index: usize) -> PlayerActionResult {
        self.players[player_index].hand.hit_cards(&mut self.deck);
        let player_hand_value = self.players[player_index].hand.value();

        if player_hand_value > 21 {
            self.players[player_index].win = -1;
            PlayerActionResult::Bust(player_hand_value)
        } else if  player_hand_value == 21{
            self.players[player_index].win = 2;
            PlayerActionResult::Blackjack
        } else {
            PlayerActionResult::Continue(player_hand_value)
        }
    }
    
    pub fn player_stand(&mut self, player_index: usize) {
         if let Some(p) = self.players.get_mut(player_index) {
            p.finished = true;
        }
    }

    
    pub fn dealer_turn(&mut self) {
        while self.should_dealer_hit() {
            self.dealer.hand.hit_cards(&mut self.deck);
        }
    }

    pub fn reveal_dealer_cards(&mut self) {
        self.dealer.hand.show_value();
    }
    
    pub fn should_dealer_hit(&self) -> bool {
        self.dealer.hand.value() < 17
    }
    
    pub fn dealer_hit(&mut self) -> DealerActionResult {
        self.dealer.hand.hit_cards(&mut self.deck);
        let dealer_hand_value = self.dealer.hand.value();

        if dealer_hand_value < 17{
            DealerActionResult::Continue(dealer_hand_value)
        } else if dealer_hand_value >= 17 || dealer_hand_value < 22 {
            DealerActionResult::Stand(dealer_hand_value)
        } else {
            DealerActionResult::Bust(dealer_hand_value)
        }
    }
    
    pub fn calculate_final_results(&mut self) {
        let dealer_val = self.dealer.hand.value();
        let dealer_bust = dealer_val > 21;

        for player in self.players.iter_mut() {
            let player_val = player.hand.value();

            if player.hand.is_blackjack() && self.dealer.hand.is_blackjack() {
                player.win = 0; // égalité
            } else if player.hand.is_blackjack() {
                player.win = 2; // blackjack
            } else if dealer_bust && !player.hand.is_bust() {
                player.win = 1; // dealer bust
            } else if player.hand.is_bust() {
                player.win = -1; // joueur bust
            } else if player_val > dealer_val {
                player.win = 1;
            } else if player_val < dealer_val {
                player.win = -1;
            } else {
                player.win = 0; // égalité
            }
        }
    }
    
    pub fn get_round_results(&self) -> Vec<(usize, i8, u32)> {
        self.players
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let gain = match p.win {
                    -1 => -(p.bet as i32),
                    0 => 0,
                    1 => p.bet as i32,
                    2 => (p.bet as i32 * 3) / 2,
                    _ => 0,
                };
                (i, p.win, gain as u32)
            })
            .collect()
    }
    pub fn discard_all_hands(&mut self) {
        for player in &mut self.players {
            self.deck.discard_hand(&mut player.hand);
            player.reset();
        }
        self.deck.discard_hand(&mut self.dealer.hand);
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

    // --- GETTERS utiles pour l'UI ---
    pub fn get_player_hand(&self, index: usize) -> Option<&Hand> {
        Some(&self.players.get(index).unwrap().hand)
    }
    
    pub fn get_dealer_hand(&self) -> &Hand {
        &self.dealer.hand
    }
    
    pub fn get_dealer_visible_value(&self) -> u8 {
        if let Some(card) = self.dealer.hand.cards.first() {
            let val = card.value.clone().to_int();
            val
        } else {
            0
        }
    }
    
    pub fn get_player_count(&self) -> usize {
        self.players.len()
    }
    
    pub fn is_all_players_done(&self) -> bool {
        self.players.iter().all(|p| p.win != 0 || p.hand.is_bust() || p.hand.is_blackjack())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_deck_initialization() {
        let deck = Deck::initialize_deck();
        assert_eq!(deck.cards.len(), 312);
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