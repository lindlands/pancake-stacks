use std::{io, thread, time};
use std::io::Write;
use std::io::prelude::*;
use std::fmt;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState}; 
use crossterm::{event, terminal};

const WIDTH_OF_SECTION: usize = 20;
const NUM_PLATES: i8 = 3;
const NUM_PANCAKES: i8 = 3;
const INIT_PLATE: usize = 0;

struct Cleanup;

impl Drop for Cleanup {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode")
    }
}

#[derive(Debug)]
enum State {
    Standard,
    Select,
}

#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
enum Pancake {
    Sm,
    M,
    Lg,
    None,
}
impl fmt::Display for Pancake {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self {
            Pancake::Lg => write!(f, "   (========)     "),
            Pancake::M => write!(f, "    (======)      "),
            Pancake::Sm => write!(f, "     (====)       "),
            Pancake::None => write!(f, "                  "),
       }
    }
}

fn is_smaller_than(p1: Pancake, p2: Pancake) -> bool {
    match p1 {
        Pancake::Sm => true,
        Pancake::M => {
            if p2 == Pancake::Sm {
                return false;
            }
            true
        },
        Pancake::Lg => {
            if p2 == Pancake::Sm || p2 == Pancake::M {
                return false;
            }
            true
        },
        Pancake::None => true,
    }
}

fn draw_selected() {
    print!("->")
}

fn draw_deselected() {
    print!("  ")
}

fn process_standard_keypresses(event: KeyEvent, state: &mut State, 
    state_array: &mut [[Pancake; (NUM_PANCAKES + 1) as usize]; NUM_PLATES as usize], 
    player_coord: &mut [i8; 2]) {
    match event {
            KeyEvent {
                code: KeyCode::Up,
                modifiers: event::KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE
            } => update_coord(player_coord, [1, 0]),
            KeyEvent {
                code: KeyCode::Left,
                modifiers: event::KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE
            } => update_coord(player_coord,[0, -1]),
            KeyEvent {
                code: KeyCode::Down,
                modifiers: event::KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE
            } => update_coord(player_coord,[-1, 0]),
            KeyEvent {
                code: KeyCode::Right,
                modifiers: event::KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE
            } => update_coord(player_coord,[0, 1]),
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: event::KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE
            } => {
                select(state_array, *player_coord); 
                update_coord(player_coord,[1, 0]); 
                *state = State::Select; 
            },
            _ => (),
    }
}

fn process_select_keypresses(event: KeyEvent, state: &mut State, 
    state_array: &mut [[Pancake; (NUM_PANCAKES + 1) as usize]; NUM_PLATES as usize], 
    player_coord: &mut [i8; 2]) {
        match event {
            KeyEvent {
                code: KeyCode::Left,
                modifiers: event::KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE
            } => {
                let old_coord = *player_coord;
                update_coord(player_coord, [0, -1]); 
                update_pancake_location(state_array, *player_coord, old_coord);
            },
            KeyEvent {
                code: KeyCode::Right,
                modifiers: event::KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE
            } => {
                let old_coord = *player_coord; 
                update_coord(player_coord, [0, 1]);
                update_pancake_location(state_array, *player_coord, old_coord);
            },
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: event::KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE
            } =>  (),
            _ => (),
        }
}

fn main() {
    // clear_screen();
    // print!("Hello!");
    // if let Err(error) = io::stdout().flush() {
    //     panic!("{}", error);
    // }
    // thread::sleep(time::Duration::from_millis(2000));
    let _cleanup = Cleanup;
    terminal::enable_raw_mode().expect("Could not turn on Raw mode");
    let mut state = State::Standard;
    let mut state_array = [[Pancake::None; (NUM_PANCAKES + 1) as usize]; NUM_PLATES as usize];
    let mut player_coord: [i8; 2] = [0, 0];
    state_array[INIT_PLATE][0] = Pancake::Lg;
    state_array[INIT_PLATE][1] = Pancake::M;
    state_array[INIT_PLATE][2] = Pancake::Sm;
    println!("{:?}", state);
    print_state(state_array, player_coord);
    draw_background();
    loop {
        if let Event::Key(event) = event::read().expect("Failed to read line") {
            if event.code == KeyCode::Esc {
                println!("\n[Exiting...]");
                break;
            }
            match state {
                State::Standard => {
                    process_standard_keypresses(event, &mut state, &mut state_array, &mut player_coord);
                }, 
                State::Select => {
                    process_select_keypresses(event, &mut state, &mut state_array, &mut player_coord);

                }
            };
            print_state(state_array, player_coord);
            draw_background();     
            // clear_screen();
        }
    }
}

fn select(state: &mut [[Pancake; (NUM_PANCAKES + 1) as usize]; NUM_PLATES as usize], player_coord: [i8; 2]) {
    match state[player_coord[1] as usize][player_coord[0] as usize] {
        Pancake::None => (println!("this is none")),
        pancake => {
            if player_coord[0] + 1 > NUM_PLATES {
                return;
            }
            if is_smaller_than(pancake, state[(player_coord[1]) as usize][(player_coord[0] + 1) as usize]) {
                state[player_coord[1] as usize][player_coord[0] as usize] = Pancake::None;
                state[player_coord[1] as usize][player_coord[0] as usize + 1] = pancake;
            } else {
                println!("failure");
            }
        }
    }
}

fn update_pancake_location(state: &mut [[Pancake; (NUM_PANCAKES + 1) as usize]; NUM_PLATES as usize], 
new_coord: [i8; 2], old_coord: [i8; 2]) {
    let pankcake = state[old_coord[1] as usize][old_coord[0] as usize];
    state[new_coord[1] as usize][new_coord[0] as usize] = pankcake;
    state[old_coord[1] as usize][old_coord[0] as usize] = Pancake::None;
}

fn update_coord(player_coord: &mut [i8; 2], new_coord: [i8; 2]) {
    let mut temp = *player_coord;
    temp[0] = temp[0] + new_coord[0];
    temp[1] = temp[1] + new_coord[1];
    if temp[0] < 0 || temp[0] > (NUM_PANCAKES + 1) {
        temp[0] = player_coord[0];
    }
    if temp[1] < 0 || temp[1] > (NUM_PLATES) {
        temp[1] = player_coord[1];
    }
    player_coord[0] = temp[0];
    player_coord[1] = temp[1];
}

fn clear_screen() {
    print!("{}[2J", 27 as char);
}

fn print_state(state: [[Pancake; 4]; 3], player_coord: [i8; 2]) {
    for i in (0..(NUM_PANCAKES + 1)).rev() {
        for j in 0..NUM_PLATES {
            if player_coord[0] == i && player_coord[1] == j {
                draw_selected();
            } else {
                draw_deselected();
            }
            match state[j as usize][i as usize] {
                Pancake::Lg => print!("{}", Pancake::Lg),
                Pancake::M => print!("{}", Pancake::M),
                Pancake::Sm => print!("{}", Pancake::Sm),
                Pancake::None => print!("{}", Pancake::None),

            }
        }
        println!();
    }
}

fn draw_background() {
    draw_plates();
    println!();
    println!();
    print!("|");
    print!("{}", tablecloth());
    print!("|");
    println!();
}

fn draw_plates() {
    let mut plate_area: [char; WIDTH_OF_SECTION] = [' '; WIDTH_OF_SECTION];
    let plate = plate();
    let letters: Vec<char> = plate.chars().collect();
    let start = plate_area.len() / 2  - (plate.len() / 2);
    for i in 0..plate.len() {
        plate_area[start + i] = letters[i];
    }
    for _i in 0..NUM_PLATES {
        print!("{}", String::from_iter(plate_area));
    }
}

fn plate() -> String {
    "\\____________/".to_string()
}

fn tablecloth() -> String {
    let mut tablecloth: String = "".to_string();
    for _i in 0..(WIDTH_OF_SECTION as i8 * NUM_PLATES / 3) {
        tablecloth.push_str("▄▀ ")
    }
    tablecloth
}