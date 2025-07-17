mod asset;

use crate::types::{Movement, piece_from_char};
use crate::{asset, chessboard::Board, geomath::Point, types::*};
use std::any::TypeId;
use std::collections::HashMap;
use std::collections::HashSet;
use std::{
    any::Any,
    error::Error,
    path::PathBuf,
};
use sdl3::event::Event;
use sdl3::pixels::Color;
use sdl3::render::WindowCanvas;

pub struct Interface {
    /// Map of assets for the pieces.
    ///
    /// The key is a tuple of [`TypeId`] and [`PieceColor`].
    /// The [`TypeId`] can be obtained with something like
    /// `Pawn::id()` or `Queen::id()` (from trait [`Any`]),
    /// and the [`PieceColor`] is the color of the piece.
    ///
    /// The value is a [`PathBuf`], the path of the asset.
    assets: HashMap<(TypeId, PieceColor), PathBuf>,
    /// Board of the game
    pub board: Board,
    /// Current turn
    pub player: PieceColor,
}

impl Default for Interface {
    fn default() -> Self {
        let paths = [
            asset!(Rook, White),
            asset!(Knight, White),
            asset!(Bishop, White),
            asset!(Queen, White),
            asset!(King, White),
            asset!(Pawn, White),
            asset!(Rook, Black),
            asset!(Knight, Black),
            asset!(Bishop, Black),
            asset!(Queen, Black),
            asset!(King, Black),
            asset!(Pawn, Black),
        ];
        let mut assets = HashMap::new();
        assets.extend(paths);

        Self {
            assets,
            board: Board::default(),
            player: PieceColor::default(),
        }
    }
}

impl Interface {
    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let sdl = sdl3::init()?;
        
        let video = sdl.video()?;
        
        let window = video
            .window("Chess", 800, 800)
            .position_centered()
            .build()?;
        
        let mut canvas = window.into_canvas();
        
        let mut event_pump = sdl.event_pump()?;
        
        // global state to draw
        
        // is the player dragging a piece?
        let mut dragging = false;
        // offset from (0, 0) of the piece rect
        // to where the player cursor is
        let mut drag_offset = (0f32, 0f32);
        
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} => break 'running,
                    _ => {}
                    // TODO take input (handle drag & drop)
                }
            }
            
            // TODO let the user move pieces freely before while MouseDown
            //  place definitely the piece only when MouseUp
            //  take input as Movement when MouseUp
            //  handle input in another func (.logic() probably) IF MouseUp
            
            canvas.set_draw_color(Color::BLACK);
            canvas.clear();
            
            self.draw(&mut canvas)?;
            canvas.present();
        }
        
        Ok(())
    }
    /// Draws the board and then the pieces
    fn draw(&mut self, canvas: &mut WindowCanvas) -> Result<(), Box<dyn Error>> {
        
        
        Ok(())
    }
}
