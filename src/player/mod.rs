use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Player {
    pub name: String,
    statsmax: HashMap<String, u8>,
    stats: HashMap<String, u8>,
    exp: u16,
    fight_or_flight: u8,
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mut output = String::new();
        output += "name: ";
        output += &self.name;
        output += "\n";
        for key in self.stats.keys() {
            let value = self.stats[key];
            let next = format!("{}: {} out of {}\n", &key, value, self.statsmax[key]);
            output += &next;
        }
        output += "exp: ";
        output += &self.exp.to_string();
        write!(f, "{}", output)
    }
}

impl Player {
    pub fn new(name: String) -> Player {
        let mut start = Player{
            name: name,
            statsmax: HashMap::new(),
            stats: HashMap::new(),
            exp: 0,
            fight_or_flight: 0,
        };
        start.add_stat("str".to_string(), 100);
        start.add_stat("soul".to_string(), 100);
        start.add_stat("know".to_string(), 100);
        start.add_stat("wis".to_string(), 100);
        start.add_stat("hp".to_string(), 100);
        start.add_stat("dex".to_string(), 100);
        start.add_stat("energy".to_string(), 10);
        start
    }
    pub fn add_stat(&mut self, cat: String, value: u8) {
        self.stats.insert(cat.clone(), value);
        self.statsmax.insert(cat.clone(), value);
    }
    pub fn spend_single(&mut self, cat: String, amount: u8) -> Result<PlayerState, String> {
        if self.stats.contains_key(&cat) {
            if self.stats[&cat] < amount {
                return Err(String::from("not enough"));
            }
            if self.fight_or_flight == 0 {
                *self.stats.get_mut(&cat).unwrap() -= amount;
            } else {
                if cat != "energy".to_string() {
                    self.fight_or_flight -= 1;
                }
            }
            if self.stats[&cat] == 0 {
                return Ok(Exhausted(cat));
            }
            return Ok(Alive);
        } else {
            return Err(String::from("not a valid stat, check your spelling"));
        }
    }
    pub fn spend(&mut self, cat: String, amount: u8) -> Result<PlayerState, String> {
        match self.spend_single("energy".to_string(), amount) {
            Ok(_) => if "energy" != &cat {
                self.spend_single(cat, amount)
            } else {
                Ok(Alive)
            },
            Err(message) => Err(message),
        }
    }
    pub fn damage(&mut self, amount: u8) -> PlayerState {
        let hp = self.stats.get_mut("hp").unwrap();
        if *hp > 10 {
            if amount > *hp {
                *hp = 1;
                self.fight_or_flight = 3;
                return Fight;
            }
            if amount + 10 > *hp {
                *hp -= amount;
                self.fight_or_flight = 3;
                return Fight;
            }
            *hp -= amount;
            return Alive;
        }
        if self.fight_or_flight > 0 {
            if amount > *hp {
                *hp = 1;
                return Alive;
            }
            *hp -= amount;
            return Alive;
        }
        if amount > *hp {
            *hp = 0;
            return Dead;
        }
        *hp -= amount;
        Alive
    }
    pub fn sleep(&mut self) -> PlayerState {
        for key in self.statsmax.keys() {
            if key == "hp" {
                continue;
            }
            let value = self.statsmax[key];
            let diff = value - self.stats[key];
            *self.stats.get_mut(key).unwrap() = value;
            self.exp += diff as u16;
            if self.exp >= 1000 {
                self.exp = self.exp % 1000;
                return LevelUp;
            }
        }
        return Alive;
    }
    pub fn eat_meal(&mut self) -> PlayerState {
        let value = self.statsmax["hp"];
        let diff = value - self.stats["hp"];
        self.exp += diff as u16;
        *self.stats.get_mut("hp").unwrap() = value;
        if self.exp >= 1000 {
            self.exp = self.exp % 1000;
            return LevelUp;
        }
        return Alive;
    }
    pub fn eat_snack(&mut self) -> PlayerState {
        let value = self.statsmax["hp"];
        let diff = value - self.stats["hp"];
        self.exp += diff as u16;
        *self.stats.get_mut("hp").unwrap() = value;
        if self.exp >= 1000 {
            self.exp = self.exp % 1000;
            return LevelUp;
        }
        return Alive;
    }
    pub fn restore(&mut self, cat: String, amount: u8) {
        if self.stats.contains_key(&cat) {
            if amount + self.stats[&cat] > self.statsmax[&cat] {
                *self.stats.get_mut(&cat).unwrap() += amount;
            } else {
                *self.stats.get_mut(&cat).unwrap() = self.statsmax[&cat];
            }
        }
    }
}

pub enum PlayerState {
    Alive,
    Fight,
    Exhausted(String),
    LevelUp,
    Dead,
}
use PlayerState::*;