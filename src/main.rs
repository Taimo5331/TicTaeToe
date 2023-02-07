use phf::phf_map;
use std;
use std::{thread, time};

pub mod server;

static NUMPAD_CONVERTER: phf::Map<&str, &'static [usize; 2]> = phf_map! {
    "1" => &[2, 0],
    "2" => &[2, 1],
    "3" => &[2, 2],
    "4" => &[1, 0],
    "5" => &[1, 1],
    "6" => &[1, 2],
    "7" => &[0, 0],
    "8" => &[0, 1],
    "9" => &[0, 2],
};

fn main() {
    let mut whose_turn: &str = "X";

    let mut bringiton: bool = true;

    let mut table: Vec<Vec<&str>> = vec![
        vec!["@", "@", "@"],
        vec!["@", "@", "@"],
        vec!["@", "@", "@"],
    ];

    println!("{:?}", table);
    loop {
        while bringiton {
            let mut line = String::new();

            println!(
                "     
              {} | {} | {}\n
              ──────────\n
              {} | {} | {}\n
              ──────────\n
              {} | {} | {}\n",
                &table[0][0],
                &table[0][1],
                &table[0][2],
                &table[1][0],
                &table[1][1],
                &table[1][2],
                &table[2][0],
                &table[2][1],
                &table[2][2]
            );

            println!("type your move : ");
            std::io::stdin().read_line(&mut line).unwrap();

            let chosen_to_change: &&[usize; 2] =
                NUMPAD_CONVERTER.get(line.as_str().trim()).unwrap();

            println!("{:?}", chosen_to_change);

            match whose_turn {
                "X" => {
                    table[chosen_to_change[0]][chosen_to_change[1]] = "X";
                    whose_turn = "O";
                }
                "O" => {
                    table[chosen_to_change[0]][chosen_to_change[1]] = "O";
                    whose_turn = "X";
                }
                _ => println!("WTF!!!"),
            }

            bringiton = !won_and_who(table.clone());
        }
        whose_turn = "X";

        bringiton = true;

        table = vec![
            vec!["@", "@", "@"],
            vec!["@", "@", "@"],
            vec!["@", "@", "@"],
        ];

        thread::sleep(time::Duration::from_secs(3));
        print!("{}[2J", 27 as char);
    }
}

fn won_and_who(table: Vec<Vec<&str>>) -> bool {
    let mut who = "";
    // col check
    for i in 0..3 {
        if &table[0][i] == &table[1][i] && &table[1][i] == &table[2][i] {
            who = &table[0][i];
        }
    }
    // row check
    for i in 0..3 {
        if &table[i][0] == &table[i][1] && &table[i][1] == &table[i][2] {
            who = &table[i][0];
        }
    }

    if &table[0][0] == &table[1][1] && &table[1][1] == &table[2][2] {
        who = &table[0][0];
    }

    if &table[0][2] == &table[1][1] && &table[1][1] == &table[2][0] {
        who = &table[0][2];
    }

    if who != "" && who != "@" {
        println!("{} won...", who);
        return true;
    } else {
        return false;
    }
}
