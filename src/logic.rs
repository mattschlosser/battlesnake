use rand::seq::SliceRandom;
use rocket_contrib::json::JsonValue;
use std::collections::HashMap;

use log::info;

use crate::{Battlesnake, Board, Game};

pub fn get_info() -> JsonValue {
    info!("INFO");

    // Personalize the look of your snake per https://docs.battlesnake.com/references/personalization
    return json!({
        "apiversion": "1",
        "author": "",
        "color": "#888888",
        "head": "default",
        "tail": "default",
    });
}

pub fn start(game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("{} START", game.id);
}

pub fn end(game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("{} END", game.id);
}

pub fn possible_move_count(possible_moves: &HashMap<&str, bool>) -> usize {
    return possible_moves
        .into_iter()
        .filter(|&(_, v)| *v == true)
        .count();
}

pub fn get_move(game: &Game, _turn: &u32, _board: &Board, you: &Battlesnake) -> &'static str {
    let mut possible_moves: HashMap<_, _> = vec![
        ("up", true),
        ("down", true),
        ("left", true),
        ("right", true),
    ]
    .into_iter()
    .collect();

    // Step 0: Don't let your Battlesnake move back in on its own neck
    let my_head = &you.head;
    let my_neck = &you.body[1];
    if my_neck.x < my_head.x {
        // my neck is left of my head
        possible_moves.insert("left", false);
    } else if my_neck.x > my_head.x {
        // my neck is right of my head
        possible_moves.insert("right", false);
    } else if my_neck.y < my_head.y {
        // my neck is below my head
        possible_moves.insert("down", false);
    } else if my_neck.y > my_head.y {
        // my neck is above my head
        possible_moves.insert("up", false);
    }

    // TODO: Step 1 - Don't hit walls.
    
    // Use board information to prevent your Battlesnake from moving beyond the boundaries of the board.
    let board_width = _board.width;
    let board_height = _board.height;
    if my_head.x >= board_width - 1 {
        possible_moves.insert("right", false);
    }
    if my_head.y >= board_height - 1 {
        possible_moves.insert("up", false);
    }
    if my_head.x <= 0 {
        possible_moves.insert("left", false);
    }
    if my_head.y <= 0 {
       possible_moves.insert("down", false);
    }

    // TODO: Step 2 - Don't hit yourself.
    // Use body information to prevent your Battlesnake from colliding with itself.
    // body = move_req.body
    you.body.iter().for_each(|coord| {
        if (coord.x == my_head.x + 1 && coord.y == my_head.y) {
            possible_moves.insert("right", false);
        }
        else if (coord.x == my_head.x - 1 && coord.y == my_head.y) {
            possible_moves.insert("left", false);
        } 
        else if (coord.x == my_head.x && coord.y == my_head.y + 1) {
            possible_moves.insert("up", false);
        }
        else if (coord.x == my_head.x && coord.y == my_head.y - 1) {
            possible_moves.insert("down", false);
        }
    });

    // TODO: Step 3 - Don't collide with others.
    _board.snakes.iter().for_each(|snake| {
        snake.body.iter().for_each(|coord| {   
            if (coord.x == my_head.x + 1 && coord.y == my_head.y) {
                possible_moves.insert("right", false);
            }
            else if (coord.x == my_head.x - 1 && coord.y == my_head.y) {
                possible_moves.insert("left", false);
            } 
            else if (coord.x == my_head.x && coord.y == my_head.y + 1) {
                possible_moves.insert("up", false);
            }
            else if (coord.x == my_head.x && coord.y == my_head.y - 1) {
                possible_moves.insert("down", false);
            }
        });
    });
            
           
    // Use snake vector to prevent your Battlesnake from colliding with others.
    // snakes = move_req.board.snakes

    // TODO: Step 4 - Find food.
    // Use board information to seek out and find food.
    // food = move_req.board.food
    if _board.food.len() > 0 {
        // find the first piece of food
        let food = &_board.food[0];
        // we dont want to go in the opposite direction 
        if food.x < my_head.x && possible_move_count(&possible_moves) > 1 {
            possible_moves.insert("right", false);
        }
        if food.x > my_head.x && possible_move_count(&possible_moves) > 1 {
            possible_moves.insert("left", false);
        }
        if food.y < my_head.y && possible_move_count(&possible_moves) > 1 {
            possible_moves.insert("up", false);
        }
        if food.y > my_head.y && possible_move_count(&possible_moves) > 1 {
            possible_moves.insert("down", false);
        }
    };
       

    // Finally, choose a move from the available safe moves.
    // TODO: Step 5 - Select a move to make based on strategy, rather than random.
    let moves = possible_moves
        .into_iter()
        .filter(|&(_, v)| v == true)
        .map(|(k, _)| k)
        .collect::<Vec<_>>();
    let chosen = moves.choose(&mut rand::thread_rng()).unwrap();

    info!("{} MOVE {}", game.id, chosen);

    return chosen;
}
