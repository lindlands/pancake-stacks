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
    Menu,
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

fn print_welcome() {
    println!("Welcome to Pancake Stacks!");
    println!("Your task is to take that big stack of pancakes on the left plate and move them to the plate on the far right.");
    println!("In doing so, however, you need to follow these rules:");
    println!("   1) You can move only one pancake at a time.");
    println!("   2) A pancake can never rest on another pancake smaller than itself.");
    println!();
    println!("To play, use the arrow keys to move and ENTER to pick up a pancake.");
    println!("To exit, press ESC.");
    println!("To start, press ENTER.");
}

fn print_exit() {
    println!();
    println!("[Exiting...]");
}

fn draw_selected() {
    print!("->")
}

fn draw_deselected() {
    print!("  ")
}

fn print_screen(state_array: [[Pancake; (NUM_PANCAKES + 1) as usize]; NUM_PLATES as usize], player_coord: [i8; 2]) {
    print_state(state_array, player_coord);
    draw_background();
    clear_screen();
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
                if is_at_pancake(*state_array, *player_coord) {
                    select(state_array, player_coord);
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

fn main() {
    // clear_screen();
    // print!("Hello!");
    // if let Err(error) = io::stdout().flush() {
    //     panic!("{}", error);
    // }
    // thread::sleep(time::Duration::from_millis(2000));
    let _cleanup = Cleanup;
    terminal::enable_raw_mode().expect("Could not turn on Raw mode");
    let mut state = State::Menu;
    let mut state_array = [[Pancake::None; (NUM_PANCAKES + 1) as usize]; NUM_PLATES as usize];
    let mut player_coord: [i8; 2] = [0, 0];
    loop {
        if let Event::Key(event) = event::read().expect("Failed to read line") {
            match state {
                State::Menu => {
                    print_welcome();
                    match event {
                        KeyEvent {
                            code: KeyCode::Esc,
                            modifiers: event::KeyModifiers::NONE,
                            kind: KeyEventKind::Press,
                            state: KeyEventState::NONE
                        } => {
                            print_exit();
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
                    clear_screen();
                }
                State::Standard => {
                    if event.code == KeyCode::Esc {
                        state = State::Menu;
                    } else {
                        process_standard_keypresses(event, &mut state, &mut state_array, &mut player_coord);
                        print_screen(state_array, player_coord);
                    }
                }, 
                State::Select => {
                    if event.code == KeyCode::Esc {
                        state = State::Menu;
                    } else {
                        process_select_keypresses(event, &mut state, &mut state_array, &mut player_coord);
                        print_screen(state_array, player_coord);
                    }
                }
            };
        }
    }
}

fn select(state_array: &mut [[Pancake; (NUM_PANCAKES + 1) as usize]; NUM_PLATES as usize], player_coord: &mut [i8; 2]) {
    match state_array[player_coord[1] as usize][player_coord[0] as usize] {
        Pancake::None => (),
        pancake => {
            if player_coord[0] + 1 > NUM_PLATES {
                return;
            }
            if is_smaller_than(pancake, state_array[(player_coord[1]) as usize][(player_coord[0] + 1) as usize]) {
                let destination = [NUM_PANCAKES, player_coord[1]];
                update_pancake_location(state_array, destination, *player_coord);
                set_coord(player_coord, [NUM_PANCAKES, player_coord[1]]);
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