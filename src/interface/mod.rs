mod asset;

use crate::geomath::Point;
use crate::{asset, chessboard::Board, types::*};
use sdl3::EventPump;
use sdl3::event::Event;
use sdl3::pixels::Color;
use sdl3::render::{Texture, TextureCreator, WindowCanvas};
use sdl3::video::WindowContext;
use sdl3::surface::Surface;
use std::any::TypeId;
use std::collections::HashMap;
use std::{error::Error, path::PathBuf};
use sdl3::image::LoadSurface;

pub struct Interface {
    canvas: WindowCanvas,
    event_pump: EventPump,
    /// Board of the game
    pub board: Board,
    /// Current turn
    pub player: PieceColor,
    /// Is the player dragging a piece?
    dragging: bool,
    /// Offset from (0, 0) of the rect to where the player's cursor is
    drag_offset: Point<f32>,
}

impl Interface {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let sdl = sdl3::init()?;
        let video = sdl.video()?;

        let window = video
            .window("Chess", 800, 600)
            .position_centered()
            .build()?;
        let canvas = window.into_canvas();
        let event_pump = sdl.event_pump()?;

        Ok(Self {
            canvas,
            event_pump,
            board: Board::default(),
            player: PieceColor::default(),
            dragging: false,
            drag_offset: Point::default(),
        })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
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
        let texture_creator = self.canvas.texture_creator();
        let mut assets = HashMap::new();
        
        for (key, path) in paths {
            let surface = Surface::from_file(path)?;
            let texture = texture_creator.create_texture_from_surface(&surface)?;
            assets.insert(key, texture);
        }

        'running: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running,
                    _ => {} // TODO take input (handle drag & drop)
                }
            }

            // TODO let the user move pieces freely before while MouseDown
            //  place definitely the piece only when MouseUp
            //  take input as Movement when MouseUp
            //  handle input in another func (.logic() probably) IF MouseUp

            self.canvas.set_draw_color(Color::BLACK);
            self.canvas.clear();

            self.draw()?;
            self.canvas.present();
        }

        Ok(())
    }
    /// Draws the board and then the pieces
    fn draw(&mut self) -> Result<(), Box<dyn Error>> {
        todo!();
    }

    fn logic(&mut self, mov: Movement) -> bool {
        todo!();
    }
}
