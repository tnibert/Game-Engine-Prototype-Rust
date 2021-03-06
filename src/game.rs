use crate::tile::{Tile, TILE_SIZE};
use crate::player::Player;
use crate::input::Input;
use crate::gameobject::GameObject;

use crate::im::Pixel;

use std::cell::RefCell;

pub const SCREEN_WIDTH: u32 = 640;
pub const SCREEN_HEIGHT: u32 = 480;

pub struct Game <'a> {
    pub input: Input<'a>,
    gameobjects: Vec<Box<RefCell<dyn GameObject>>>
}

impl Game <'_> {
    pub fn new() -> Self {
        let player = Box::new(RefCell::new(Player::new()));
        //let mytilearea = RefCell::new(TileArea::new(create_tile_map()));

        // just for testing
        let mytile = Box::new(RefCell::new(
            // black square
            Tile::new(|| {
                let mut img = im::RgbaImage::new(TILE_SIZE as u32, TILE_SIZE as u32);
        
                for x in 0..TILE_SIZE as u32 {
                    for y in 0..TILE_SIZE as u32 {
                        img.put_pixel(x, y, im::Rgb([0, 0, 0]).to_rgba());
                    }
                }
                img
            })
        ));

        // setup subscriptions
        let mut input = Input::new();
        // TODO
        //let p_ref: &RefCell<Player> = &player;
        //input.subscribe(p_ref, vec!["up", "down", "left", "right"]);

        //drop(p_ref);
        Game {
            input: input,
            gameobjects: vec![mytile, player]
        }
    }
}

impl GameObject for Game <'_> {
    // create the screen image
    fn render(&self) -> Option<im::RgbaImage> {
        let mut screen_img = im::RgbaImage::new(SCREEN_WIDTH, SCREEN_HEIGHT);

        // clear screen
        // todo: find a more efficient call to do this
        for x in 0..SCREEN_WIDTH {
            for y in 0..SCREEN_HEIGHT {
                screen_img.put_pixel(x, y, im::Rgb([255, 255, 255]).to_rgba());
            }
        }

        for g in &self.gameobjects {
            if let Some(img) = g.borrow().render() {
                if let Some(pos) = g.borrow().position() {
                    im::imageops::overlay(&mut screen_img, &img, pos.0 as i64, pos.1 as i64);
                } else {
                    continue;
                }
            } else {
                continue;
            }
        }

        return Some(screen_img);
    }

    fn position(&self) -> Option<(f64, f64)> {
        return Some((0.0, 0.0));
    }

    fn update(&mut self) {
        for u in &self.gameobjects {
            u.borrow_mut().update();
        }
    }
}