mod blackjack;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Layout, Alignment},
    widgets::{Block, Paragraph, Borders, List, ListItem},
    Frame,
    prelude::Margin,
    style::{Style, Color, Modifier},
    text::{Span, Line},
};
use blackjack::{Game, GameState};
use std::io;

struct App {
    game: Game,
    game_state: GameState,
    current_player: usize,
    message: String,
    show_dealer_cards: bool,
}

impl App{
    fn new() -> Self {
        let mut game = Game::new();
        game.deck.shuffle();
        Self {
            game,
            game_state: GameState::Setup,
            current_player: 0,
            message: "Appuyez sur 'n' pour commencer".to_string(),
            show_dealer_cards: false,
        }
    }
}

fn main() {
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(draw).expect("failed to draw frame");
        if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
            break;
        }
    }
    ratatui::restore();
    blackjack::test();
}


fn draw(frame: &mut Frame) {
    use Constraint::Percentage;

    let area = frame.area();
    // Bloc principal avec titre
    frame.render_widget(Block::bordered().title("Blackjack"), area);

    // Zone intérieure pour ne pas recouvrir le titre/bordure
    let inner_area = area.inner(Margin {
        vertical: 1,
        horizontal: 2,
    });

    // Découpe verticale en 3 parties
    let vertical = Layout::vertical([Percentage(30), Percentage(65), Percentage(5)]);
    let [dealer_area, players_area, status_area] = vertical.areas(inner_area);

    frame.render_widget(Block::bordered().title("Dealer"), dealer_area);
    frame.render_widget(Block::bordered().title("Players"), players_area);
    frame.render_widget(Block::bordered().title("Status Bar"), status_area);
}