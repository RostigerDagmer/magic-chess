use std::sync::Arc;

use spell::Kind;
use spell::{JihadiWarrior, Spell};
use yew::prelude::*;
use chess::{Board, Color, Piece, Square};
use crate::game::moves::make_move;
use crate::game::spell;
use crate::game::Game;
use log::info;

fn render_square(sq: &UISquare) -> Html {

    let color = match sq.color {
        Color::White => "white",
        Color::Black => "black",
    };

    let inner = match sq.piece.clone() {
        Some(piece) => {
            let piece_color = match piece.color {
                Color::White => "white",
                Color::Black => "black",
            };
            html! {
                <img id={sq.str_id()} src={format!("/assets/pieces/{}/{}.svg", piece_color, piece.to_string().to_lowercase())} alt={format!("{:?}", piece.to_string())} />
            }
        },
        None => html! {},
    };

    html! {
        <div class={"square square--".to_owned() + color} style={format!{"width: {}%; height: {}%;", sq.width, sq.height}}>
            <div class="piece">
                {inner}
            </div>
            {spell::render(&sq.spell)}
        </div>
    }
}

#[derive(Clone)]
pub struct UIBoard {
    dims: (usize, usize),
    board: Board,
    spells: Arc<Vec<spell::Kind<dyn Spell>>>,
}

#[derive(Properties, PartialEq, Clone)]
struct UIPiece {
    piece: Piece,
    color: Color,
}

impl ToString for UIPiece {
    fn to_string(&self) -> String {
        self.piece.to_string(self.color)
    }
}

#[derive(Properties)]
struct UISquare {
    piece: Option<UIPiece>,
    color: Color,
    spell: spell::Kind<dyn Spell>,
    width: f64,
    height: f64,
    id: [u8; 2],
}

impl PartialEq for UISquare {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl UISquare {
    fn gen_square(file: usize, rank: usize, piece: Option<Piece>, color: Option<Color>, spell: spell::Kind<dyn Spell>, size: (f64, f64)) -> UISquare {
        let piece = match piece {
            Some(piece) => Some(UIPiece{ piece, color: color.unwrap() }),
            None => None,
        };
        match (file + rank) % 2 {
            0 => Self{ piece, color: Color::White, spell, width: size.0, height: size.1, id: [file as u8, rank as u8]},
            _ => Self{ piece, color: Color::Black, spell, width: size.0, height: size.1, id: [file as u8, rank as u8]},
        }
    }
    fn str_id(&self) -> String {
        format!("{}{}", self.id[0], self.id[1])
    }
}

impl Default for UIBoard {
    fn default() -> Self {
        let mut spells: Vec<Kind<dyn Spell>> = Vec::with_capacity(64);
        (0..64).into_iter().for_each(|i| {
            if i == 4 {
                spells.push(Kind::Opaque(Arc::new(JihadiWarrior::default())))
            } else if i == 20 {
                spells.push(Kind::Transparent(Arc::new(JihadiWarrior::default())));
            } else {
                spells.push(Kind::None);
            }
        });
        Self {
            dims: (8, 8),
            spells: Arc::new(spells),
            board: Board::default(),
        }
    }
}

impl UIBoard {
    pub fn new(board: Board, dims: (usize, usize), spells: Option<Arc<Vec<spell::Kind<dyn Spell>>>>) -> Self {
        let spells = spells.unwrap_or({
            let mut default = Vec::with_capacity(dims.0 * dims.1);
            (0..dims.0 * dims.1).into_iter().for_each(|i| {
                default.push(Kind::None);
            });
            Arc::new(default)
        });
        Self {
            dims,
            spells,
            board: board.clone(),
        }
    }
    fn gen_squares(&self) -> Vec<UISquare> {
        let width = (1.0 / self.dims.0 as f64) * 100.0;
        let height = (1.0 / self.dims.1 as f64) * 100.0;
        (0..self.dims.1).into_iter().map(|rank| {
            (0..self.dims.0).into_iter().map(move |file| {
                let square = Square::make_square(chess::Rank::from_index(rank), chess::File::from_index(file));
                let mut spell = Kind::None;
                match &self.spells.clone()[square.get_file().to_index() * self.dims().0 + square.get_rank().to_index()] {
                    Kind::Transparent(s) => spell = Kind::Transparent(s.clone()),
                    Kind::Opaque(s) => spell = Kind::Opaque(s.clone()),
                    Kind::None => ()
                }

                let piece = self.board.piece_on(square);
                let color = self.board.color_on(square);
                UISquare::gen_square(file, rank, piece, color, spell, (width, height))
            })
        }).flat_map(|x| x).collect()
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn piece_on(&self, square: chess::Square) -> Option<chess::Piece> {
        self.board().piece_on(square)
    }

    pub fn color_on(&self, square: chess::Square) -> Option<chess::Color> {
        self.board().color_on(square)
    }

    pub fn make_move_new(&self, m: chess::ChessMove) -> UIBoard {
        let b_ = self.board().make_move_new(m);
        Self::new(b_, self.dims, Some(self.spells.clone()))
    }

    pub fn dims(&self) -> (usize, usize) {
        self.dims
    }

    pub fn spell_on(&self, square: chess::Square) -> Option<Arc<dyn Spell>> {
        let s = &self.spells[square.get_file().to_index() * self.dims().0 + square.get_rank().to_index()];
        info!("spell on {:?}:{:?}", square, s);
        match s {
            Kind::None => None,
            Kind::Opaque(s) => Some(s.clone()),
            Kind::Transparent(s) => Some(s.clone())
        }
    }

    pub fn remove_piece(&self, square: Square) -> UIBoard {
        let board = self.board().clone();
        let cleared = board.clear_square(square);
        match cleared {
            Some(board) => Self::new(board, self.dims, Some(self.spells.clone())),
            None => self.clone(),
        }
    }
}

fn wrapped_move(state: UseStateHandle<Game>) -> Callback<DragEvent, ()> {
    let ondrag = Callback::from(
        move |e| {
            let game = state.clone();
            let _board = make_move(&game, e);
            info!("{:?}", game.raw_board().side_to_move());
            state.set(_board)
        });
    ondrag
}

pub fn render(state: UseStateHandle<Game>) -> Html {
    // let whites = game.current_position().color_combined(chess::Color::White);
    // let blacks = game.current_position().color_combined(chess::Color::Black);
    let game = state.clone();
    html! {
        <div id="board" class="board" ondragend={wrapped_move(state)}>
            { game.board().gen_squares().iter().rev().map(|s| render_square(s)).collect::<Html>() }
        </div>
    }
}