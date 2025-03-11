use std::fmt;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState}; 
use crossterm::{event, terminal};
mod view;

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
    Menu,
    Standard,
    Select,
    Complete,
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

fn is_solved(state_array: [[Pancake; (NUM_PANCAKES + 1) as usize]; NUM_PLATES as usize]) -> bool {
    let mut final_state = [[Pancake::None; (NUM_PANCAKES + 1) as usize]; NUM_PLATES as usize];
    final_state[(NUM_PLATES - 1) as usize][0] = Pancake::Lg;
    final_state[(NUM_PLATES - 1) as usize][1] = Pancake::M;
    final_state[(NUM_PLATES - 1) as usize][2] = Pancake::Sm;
    state_array == final_state
}

fn drop_pancake(state: &mut State, state_array: &mut [[Pancake; (NUM_PANCAKES + 1) as usize]; NUM_PLATES as usize], player_coord: &mut [i8; 2]) {
    let selected_pancake =  state_array[player_coord[1] as usize][(player_coord[0]) as usize];
    let mut place = 1;
    for i in 1..(player_coord[0] + 1) {
        match state_array[player_coord[1] as usize][(player_coord[0] - i) as usize] {
            Pancake::None => place = i,
            p => {
                if is_smaller_than(selected_pancake, p) {
                    place = i - 1;
                    break;
                } else {
                    return;
                }
            },
        }
    }
    update_pancake_location(state_array, [player_coord[0] - place, player_coord[1]], *player_coord);
    update_coord(player_coord,[-1, 0]);
    *state = State::Standard;
}

fn is_at_pancake(state_array: [[Pancake; (NUM_PANCAKES + 1) as usize]; NUM_PLATES as usize], player_coord: [i8; 2]) -> bool {
    state_array[player_coord[1] as usize][(player_coord[0]) as usize] != Pancake::None
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
            } => _ = update_coord(player_coord, [1, 0]),
            KeyEvent {
                code: KeyCode::Left,
                modifiers: event::KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE
            } => {
                if update_coord(player_coord, [0, -1]) {
                    snap_to_pancake(*state_array, player_coord,[-1, 0])
                }
            },
            KeyEvent {
                code: KeyCode::Down,
                modifiers: event::KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE
            } => _ = update_coord(player_coord,[-1, 0]),
            KeyEvent {
                code: KeyCode::Right,
                modifiers: event::KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE
            } => {
                if update_coord(player_coord, [0, 1]) {
                    snap_to_pancake(*state_array, player_coord,[-1, 0])
                }
            },
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: event::KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE
            } => {
                if is_at_pancake(*state_array, *player_coord) && select(state_array, player_coord){
                    *state = State::Select; 
                }
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
                kind: KeyEventKind::Release,
                state: KeyEventState::NONE
            } => {
                let old_coord = *player_coord;
                if update_coord(player_coord, [0, -1]) {
                    update_pancake_location(state_array, *player_coord, old_coord);
                }
            },
            KeyEvent {
                code: KeyCode::Right,
                modifiers: event::KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE
            } => {
                let old_coord = *player_coord; 
                if update_coord(player_coord, [0, 1]) {
                    update_pancake_location(state_array, *player_coord, old_coord);

                }
            },
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: event::KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE
            } => {
                drop_pancake(state, state_array, player_coord);
                snap_to_pancake(*state_array, player_coord, *player_coord);
            },
            _ => (),
        }
}

fn initialize(state_array: &mut [[Pancake; (NUM_PANCAKES + 1) as usize]; NUM_PLATES as usize], player_coord: &mut [i8; 2]) {
    *state_array = [[Pancake::None; (NUM_PANCAKES + 1) as usize]; NUM_PLATES as usize];
    state_array[INIT_PLATE][0] = Pancake::Lg;
    state_array[INIT_PLATE][1] = Pancake::M;
    state_array[INIT_PLATE][2] = Pancake::Sm;
    *player_coord = [0,0];
}

fn select(state_array: &mut [[Pancake; (NUM_PANCAKES + 1) as usize]; NUM_PLATES as usize], player_coord: &mut [i8; 2]) -> bool{
    match state_array[player_coord[1] as usize][player_coord[0] as usize] {
        Pancake::None => false,
        pancake => {
            if player_coord[0] + 1 > NUM_PLATES {
                return false;
            }
            if is_smaller_than(pancake, state_array[(player_coord[1]) as usize][(player_coord[0] + 1) as usize]) {
                let destination = [NUM_PANCAKES, player_coord[1]];
                update_pancake_location(state_array, destination, *player_coord);
                set_coord(player_coord, [NUM_PANCAKES, player_coord[1]]);
                true
            } else {
                false
            }
        }
    }
}

fn update_pancake_location(state_array: &mut [[Pancake; (NUM_PANCAKES + 1) as usize]; NUM_PLATES as usize], 
new_coord: [i8; 2], old_coord: [i8; 2]) {
    let pankcake = state_array[old_coord[1] as usize][old_coord[0] as usize];
    state_array[new_coord[1] as usize][new_coord[0] as usize] = pankcake;
    state_array[old_coord[1] as usize][old_coord[0] as usize] = Pancake::None;
}

fn set_coord(player_coord: &mut [i8; 2], new_coord: [i8; 2]) {
    let mut temp = *player_coord;
    temp[0] = new_coord[0];
    temp[1] = new_coord[1];
    if temp[0] < 0 || temp[0] > (NUM_PANCAKES + 1) {
        temp[0] = player_coord[0];
    }
    if temp[1] < 0 || temp[1] > (NUM_PLATES) {
        temp[1] = player_coord[1];
    }
    player_coord[0] = temp[0];
    player_coord[1] = temp[1];
}

fn snap_to_pancake(state_array: [[Pancake; (NUM_PANCAKES + 1) as usize]; NUM_PLATES as usize], player_coord: &mut [i8; 2], new_coord: [i8; 2]) {
    while update_coord(player_coord, new_coord) && !is_at_pancake(state_array, *player_coord) { () }
}

fn update_coord(player_coord: &mut [i8; 2], new_coord: [i8; 2]) -> bool {
    let mut temp = *player_coord;
    temp[0] = temp[0] + new_coord[0];
    temp[1] = temp[1] + new_coord[1];
    if temp[0] < 0 || temp[0] >= (NUM_PANCAKES + 1) {
        return false;
    }
    if temp[1] < 0 || temp[1] >= NUM_PLATES {
        return false;
    }
    player_coord[0] = temp[0];
    player_coord[1] = temp[1];
    true
}

fn main() {
    let _cleanup = Cleanup;
    terminal::enable_raw_mode().expect("Could not turn on Raw mode");
    let mut state = State::Menu;
    let mut state_array = [[Pancake::None; (NUM_PANCAKES + 1) as usize]; NUM_PLATES as usize];
    let mut player_coord: [i8; 2] = [0, 0];
    loop {
        if let Event::Key(event) = event::read().expect("Failed to read line") {
            match state {
                State::Menu => {
                    view::print_welcome();
                    match event {
                        KeyEvent {
                            code: KeyCode::Esc,
                            modifiers: event::KeyModifiers::NONE,
                            kind: KeyEventKind::Press,
                            state: KeyEventState::NONE
                        } => {
                            view::print_exit();
                            break;
                        },
                        KeyEvent {
                            code: KeyCode::Enter,
                            modifiers: event::KeyModifiers::NONE,
                            kind: KeyEventKind::Press,
                            state: KeyEventState::NONE
                        } => {
                            initialize(&mut state_array, &mut player_coord);
                            state = State::Standard;
                        },
                        _ => {}
                    }
                    view::clear_screen();
                }
                State::Standard => {
                    if event.code == KeyCode::Esc {
                        state = State::Menu;
                    } else {
                        process_standard_keypresses(event, &mut state, &mut state_array, &mut player_coord);
                        view::print_screen(state_array, player_coord);
                    }
                }, 
                State::Select => {
                    if event.code == KeyCode::Esc {
                        state = State::Menu;
                    } else {
                        process_select_keypresses(event, &mut state, &mut state_array, &mut player_coord);
                        view::print_screen(state_array, player_coord);
                        if is_solved(state_array) {
                            state = State::Complete;
                        }
                    }
                },
                State::Complete => {
                    view::print_complete();
                    match event {
                        KeyEvent {
                            code: KeyCode::Enter,
                            modifiers: event::KeyModifiers::NONE,
                            kind: KeyEventKind::Press,
                            state: KeyEventState::NONE
                        } => {
                            state = State::Menu;
                        },
                        _ => {}
                    }
                }
            }
        }
    }
}