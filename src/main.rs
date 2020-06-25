use std::{thread, time};
use std::time::{Instant};

const WIDTH: usize = 4;
const HEIGHT: usize = 6;
const DO_SLEEP: bool = false;
const DO_PRINT: bool = false;
const MAX_ITERATIONS: usize = 1000 * 1000;

// STATES:
// 0 state --> dead
// 1 state --> alive

fn print_state(state: [[u8; WIDTH]; HEIGHT]) {
    for (_i, row) in state.iter().enumerate() {
        for (_j, col) in row.iter().enumerate() {
            print!("{} ", col);
        }
        println!()
    }
}

fn out_of_bounds(index: i32, max: i32) -> bool {
    index < 0 || index >= max
}

fn is_alive(state: u8) -> bool {
    state == 1
}

fn update_state(state: &mut [[u8; WIDTH]; HEIGHT]){
    let mut new_state = [[0u8; WIDTH]; HEIGHT];  // create array to hold new sate values
    const MAX_X:i32 = HEIGHT as i32;
    const MAX_Y:i32 = WIDTH as i32;

    for x in 0..MAX_X {
        for y in 0..MAX_Y {
            let mut counter: u8 = 0;  // counter per cell
            for dx in -1..2 as i32 { // [-1, 0, 1]
                let xx = x + dx;
                if out_of_bounds(xx, MAX_X) {
                    continue;
                }
                for dy in -1..2 as i32 { // [-1, 0, 1]
                    if dx == 0 && dy == 0 { // don't count square itself, only the eight neighbours
                        continue;
                    }
                    let yy = y + dy;
                    if out_of_bounds(yy, MAX_Y) {
                        continue;
                    }
                    counter += state[xx as usize][yy as usize];
                }
            }

            let alive:bool = is_alive(state[x as usize][y as usize]);
            // RULE 1: live and 2 or 3 neighbours --> stay alive
            let rule1: bool = alive && (counter == 2 || counter == 3);
            // RULE 2: dead and 3 neighbours --> become alive
            let rule2: bool = !alive && counter == 3;

            if rule1 || rule2 {
                new_state[x as usize][y as usize] = 1;
            }
            // RULE 3: other cells --> become/stay dead
            else {
                new_state[x as usize][y as usize] = 0;
            }
        }
    }

    for (i, row) in new_state.iter().enumerate() {
        for (j, _col) in row.iter().enumerate() {
            state[i][j] = new_state[i][j];
        }
    }
    
}


fn main() {
    // initialize state
    let mut state = [[0u8; WIDTH]; HEIGHT];  // array of 0s as a u8 type (all cells dead)
    state[0][0] = 1;
    state[1][0] = 1;
    state[0][1] = 1;
    state[1][1] = 1;

    state[2][2] = 1;
    state[2][3] = 1;
    state[3][2] = 1;
    state[3][3] = 1;

    let sleep_time = time::Duration::from_millis(500);

    let mut iteration: usize = 0;
    let start = Instant::now();
    loop {
        update_state(&mut state);

        if DO_PRINT {
            print_state(state);
        }

        if DO_SLEEP {
            thread::sleep(sleep_time);
        }

        iteration += 1;
        if iteration >= MAX_ITERATIONS {
            break;
        }
    }
    let duration = start.elapsed();
    println!("Done in {:?}! ({:?} per iteration)", duration, duration/MAX_ITERATIONS as u32);
}
