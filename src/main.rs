use rand::Rng;

#[derive(Debug, PartialEq)]
enum Door {
    Car,
    Goat,
}

fn main() {
    let mut wins = 0;
    let mut losses = 0;
    for i in 0..1_000_000 {
        if run_monty_hall(i % 1000 == 0) {
            wins += 1;
        } else {
            losses += 1;
        }
    }
    println!("wins: {}", wins);
    println!("losses: {}", losses);
}

fn run_monty_hall(should_print: bool) -> bool {
    let prize_door = rand::thread_rng().gen_range(0..3);
    let doors: Vec<Door> = (0..3)
        .map(|i| {
            if i == prize_door {
                Door::Car
            } else {
                Door::Goat
            }
        })
        .collect();
    let chosen_door = rand::thread_rng().gen_range(0..3);
    let revealed_door = loop {
        let revealed_door = rand::thread_rng().gen_range(0..3);
        if revealed_door != chosen_door && doors.get(revealed_door).unwrap() == &Door::Goat {
            break revealed_door;
        }
    };
    let switched_door = (0..3)
        .find(|n| *n != chosen_door && *n != revealed_door)
        .unwrap();
    if should_print {
        println!("doors: {:?}", doors);
        println!(
            "chosen door: {:?} ({})",
            doors.get(chosen_door).unwrap(),
            chosen_door + 1
        );
        println!(
            "switched door: {:?} ({})",
            doors.get(switched_door).unwrap(),
            switched_door + 1
        );
        println!("---");
    }
    doors.get(switched_door) == Some(&Door::Car)
}
