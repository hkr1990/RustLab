use rand::{Rng};
use std::{thread, time};

// Your brother can see the entire forest, a 5 x 5 grid, 
// and knows that you are hiding somewhere in there. 
// He decides to fire off explosions at random spots in the forest. Maybe he'll hit you, maybe he won't. 
// Either way, he has a limited amount of energy can only fire off 36 explosions before his turn is over. 
// But, if your brother hits you 3 times then he will win and get to choose what to eat for dinner


const GRID_SIZE: u8 = 5;
const MAX_EXPLOSIONS: u8 = 36;
const MAX_HITS: u8 = 3;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct Coord {
    x: u8,
    y: u8,
}

impl Coord {
    fn new(x: u8, y: u8) -> Coord {
        return Coord { x, y };
    }

    fn get_rand_coord(max_x: u8, max_y: u8) -> Coord {
        return Coord::new(
            rand::thread_rng().gen_range(0..max_x),
            rand::thread_rng().gen_range(0..max_y),
        );
    }
}

/// The Representation of the hide and seek game
struct GameState {
    explosions_left: u8,
    little_brother_hits: u8,
    little_brother_location: Coord,
    sister_location: Coord,
    brother_hit: bool,
    sister_hit: bool,
}

impl GameState {
    /// Initializes the GameState
    pub fn new() -> Self {
        let mut state = GameState { 
            explosions_left: MAX_EXPLOSIONS, 
            little_brother_hits: 0, 
            little_brother_location: Coord::get_rand_coord(GRID_SIZE as u8, GRID_SIZE as u8), 
            sister_location: Coord::get_rand_coord(GRID_SIZE as u8, GRID_SIZE as u8),
            brother_hit: false,
            sister_hit: false
        };

         // brother and sister can't be in the same spot
        while state.little_brother_location == state.sister_location {
            state.sister_location = Coord::get_rand_coord(GRID_SIZE as u8, GRID_SIZE as u8);
        }

        return state;
    }

    /// Move the little brother to a new location
    pub fn move_little_brother(&mut self) {
        self.little_brother_location = Coord::get_rand_coord(GRID_SIZE as u8, GRID_SIZE as u8) 
    }

    pub fn fire_random_explosion(&mut self)-> bool {
        // reset hit flags
        self.brother_hit = false;
        self.sister_hit = false;

        let explosion_location = Coord::get_rand_coord(GRID_SIZE as u8, GRID_SIZE as u8);
        let mut successful_hit = false;
        if explosion_location == self.little_brother_location {
            self.little_brother_hits += 1;
            successful_hit = true;
            self.brother_hit = true;
        } else if explosion_location == self.sister_location {
            if self.explosions_left < 3 {
                self.explosions_left = 0;
            } else {
                self.explosions_left -= 3;
            }
            self.sister_hit = true;
            println!("Sister got hit! Lose 3 explosions!");
        }
        self.display_forest();
        self.explosions_left -= 1;
        return successful_hit;
    }

    pub fn is_game_over(&self) -> bool {
        return self.explosions_left == 0 || self.little_brother_hits == MAX_HITS;
    }

    pub fn display_forest(&self) {
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                let coord = Coord::new(x, y);
                if coord == self.little_brother_location {
                    if self.brother_hit {
                        print!("\u{1F4A5}");
                    } else {
                        print!("\u{1F466}");
                    }
                } else if coord == self.sister_location {
                    if self.sister_hit {
                        print!("\u{1F4A5}");
                    } else {
                        print!("\u{1F467}");
                    }
                } else {
                    print!("\u{1F332}");
                }
            }
            println!();
        }
    }

    pub fn print_score(&self) {
        println!("Explosions left: {}", self.explosions_left);
        println!("Little brother hits: {}", self.little_brother_hits);
    }

    pub fn print_winner(&self) {
        if self.little_brother_hits == MAX_HITS {
            println!("You Won!");
        } else if self.explosions_left == 0 {
            println!("Your brother won!");
        } else {
            println!("Game not yet over!");
        }
    }
}



fn main() {
    let mut game = GameState::new();
    while !game.is_game_over() {
        clearscreen::clear().expect("failed to clear screen");
        let successful_hit = game.fire_random_explosion();
        if successful_hit {
            println!("Brother got hit! Move him!");
            game.move_little_brother();
        } else {
            println!("Brother was not hit!");
        }
        game.print_score();
        let ten_millis = time::Duration::from_millis(1000);
        thread::sleep(ten_millis);

        if game.is_game_over() {
            game.print_winner();
            break;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    /// Tests that a grid is valid. IE it contains trees, you, and your sister.
    #[test]
    fn test_valid_grid() {}
}
