mod apa;
mod set_match;

use simul::Simulation;
use std::collections::HashMap;
use std::fs::File;
use std::io::Error;
use std::io::Write;

fn find_winner(simulation: &Simulation) -> String {
    simulation
        .agents
        .iter()
        .find(|a| {
            a.state()
                .produced
                .last()
                .is_some_and(|m| m.interrupt.is_some())
        })
        .map(|a| a.state().id.clone())
        .unwrap()
}

fn main() -> Result<(), Error> {
    // To vary who "breaks" first, we pass in a starting player, 0 or 1.
    let mut starting_player: usize = 0;

    let mut w = File::create("apa-data.txt")?;
    writeln!(&mut w, "luck_chance\tbetter_player_win_percent")?;

    for pct in [
        0.00, 0.05, 0.10, 0.15, 0.20, 0.25, 0.30, 0.35, 0.40, 0.45, 0.50,
    ]
    .into_iter()
    {
        let mut innings = 0u64;
        let mut count: HashMap<String, u32> = HashMap::new();
        for _ in 0..131072 {
            let finished_simulation =
                apa::nine_ball_apa_rules_simulation_alice_vs_john(pct, starting_player);
            let winner = find_winner(&finished_simulation);

            innings += finished_simulation.time;

            *count.entry(winner).or_default() += 1;
            starting_player ^= 1;
        }

        eprintln!("Luck={}", pct);
        eprintln!("Average Innings: {}", innings as f32 / 131072.0);

        writeln!(
            &mut w,
            "{}\t{}",
            pct * 100.0,
            (count["alice"] as f32 / (count["alice"] + count["john"]) as f32) * 100.0
        )?;
    }

    let mut w = File::create("set-match-data.txt")?;
    writeln!(&mut w, "luck_chance\tbetter_player_win_percent")?;

    for pct in [
        0.00, 0.05, 0.10, 0.15, 0.20, 0.25, 0.30, 0.35, 0.40, 0.45, 0.50,
    ]
    .into_iter()
    {
        let mut count: HashMap<String, u32> = HashMap::new();
        let mut starting_player = 0;

        for _ in 0..131072 {
            let finished_simulation =
                set_match::normal_nine_ball_simulation_alice_vs_john(pct, starting_player);
            let winner = find_winner(&finished_simulation);

            *count.entry(winner).or_default() += 1;
            starting_player ^= 1;
        }

        writeln!(
            &mut w,
            "{}\t{}",
            pct * 100.0,
            (count["alice"] as f32 / (count["alice"] + count["john"]) as f32) * 100.0
        )?;
    }

    Ok(())
}
