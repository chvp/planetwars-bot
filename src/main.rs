use serde::{Deserialize, Serialize};
use std::io::{self, BufRead, Write};
use std::iter::zip;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    pub planets: Vec<Planet>,
    pub expeditions: Vec<Expedition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Planet {
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub owner: Option<usize>,
    pub ship_count: usize,
}

impl Planet {
    fn distance(&self, other: &Self) -> usize {
        ((self.x - other.x).powf(2.0) + (self.y - other.y).powf(2.0))
            .sqrt()
            .ceil() as usize
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expedition {
    pub id: usize,
    pub origin: String,
    pub destination: String,
    pub owner: usize,
    pub ship_count: usize,
    pub turns_remaining: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dispatch {
    pub origin: String,
    pub destination: String,
    pub ship_count: usize,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Command {
    pub moves: Vec<Dispatch>,
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let state: State = serde_json::from_str(&line.expect("error reading line"))?;
        let mut command = Command { moves: vec![] };
        let (mut my_planets, mut other_planets): (Vec<Planet>, Vec<Planet>) =
            state.planets.into_iter().partition(|p| p.owner == Some(1));
        my_planets.sort_by_key(|p| p.ship_count);
        my_planets.reverse();
        other_planets.sort_by_key(|p| p.ship_count);
        for (mine, theirs) in zip(my_planets, other_planets).take(1) {
            let attackers: usize = state
                .expeditions
                .iter()
                .filter(|e| e.owner != 1 && e.destination == mine.name)
                .map(|e| e.ship_count)
                .sum();
            let already_sent: usize = state
                .expeditions
                .iter()
                .filter(|e| e.owner == 1 && e.destination == mine.name)
                .map(|e| e.ship_count)
                .sum();
            let max_send = mine.ship_count - attackers;
            let to_beat = theirs.ship_count
                + if let Some(_) = theirs.owner {
                    mine.distance(&theirs)
                } else {
                    0
                };
            if max_send > to_beat && already_sent <= to_beat {
                command.moves.push(Dispatch {
                    origin: mine.name.clone(),
                    destination: theirs.name.clone(),
                    ship_count: to_beat + 1 - already_sent,
                });
            }
        }

        println!("{}", serde_json::to_string(&command)?);
        io::stdout().flush()?;
    }
    Ok(())
}
