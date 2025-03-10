use crate::{Pancake, NUM_PANCAKES, NUM_PLATES};

const WIDTH_OF_SECTION: usize = 20;

pub fn print_welcome() {
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

pub fn print_exit() {
    println!();
    println!("[Exiting...]");
}

pub fn draw_selected() {
    print!("->")
}

pub fn draw_deselected() {
    print!("  ")
}

pub fn print_screen(state_array: [[Pancake; (NUM_PANCAKES + 1) as usize]; NUM_PLATES as usize], player_coord: [i8; 2]) {
    print_state(state_array, player_coord);
    draw_background();
    clear_screen();
}

pub fn clear_screen() {
    print!("{}[2J", 27 as char);
}

pub fn print_state(state: [[Pancake; 4]; 3], player_coord: [i8; 2]) {
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