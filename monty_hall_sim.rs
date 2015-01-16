#![feature(slicing_syntax)]

use std::rand;
use std::rand::Rng;

fn rand_door() -> int {
    let mut rng = rand::thread_rng();
    rand::sample(&mut rng, range(1, 4i), 1)[0]
}

fn pick_open_door( prize_door: int, pick_door: int ) -> int {
    let mut open_door_set = vec!();
    
    if prize_door != 1 && pick_door != 1 {
        open_door_set.push( 1i );
    }

    if prize_door != 2 && pick_door != 2 {
        open_door_set.push( 2i );
    }

    if prize_door != 3 && pick_door != 3 {
        open_door_set.push( 3i );
    }

    let mut rng = rand::thread_rng();
    rand::sample(&mut rng, open_door_set.into_iter(), 1)[0]
}

fn play_round( switch_choice:bool ) -> bool {
    let prize_door = rand_door();
    let pick_door  = rand_door();
    let open_door  = pick_open_door( prize_door, pick_door );

    let mut final_door = pick_door;

    if switch_choice {
        if pick_door != 1 && open_door != 1 {
            final_door = 1;
        }

        if pick_door != 2 && open_door != 2 {
            final_door = 2;
        }

        if pick_door != 3 && open_door != 3 {
            final_door = 3;
        }
    }

    prize_door == final_door
}

fn main() {
    let mut stay_wins   = 0;
    let mut switch_wins = 0;
    let mut total       = 0;
    
    while total <= 1000 {
        if play_round( false ) {
            stay_wins += 1;
        }

        if play_round( true ) {
            switch_wins += 1;
        }

        total += 1;
    }

    println!( "Stay   Wins: {}%", stay_wins as f32 / total as f32 * 100.0 );
    println!( "Switch Wins: {}%", switch_wins as f32 / total as f32 * 100.0 );
}
