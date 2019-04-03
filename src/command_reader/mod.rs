extern crate serenity;
extern crate ron;
extern crate crossbeam_channel;

use std::thread;
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;
use crate::player::{Player, PlayerState::*};
use serenity::model::channel::Message;
use ron::ser::{to_string_pretty, PrettyConfig};
use ron::de::from_reader;
use crossbeam_channel::{unbounded, Sender};

pub mod handle;

fn say<D: Display>(mess: Message, out: D) {
    mess.channel_id.say(out).unwrap();
}

pub struct Gateway (
    Sender<(Command, Message)>,
);

impl Gateway {
    pub fn new() -> Gateway {
        let (sender, reciever) = unbounded();
        thread::spawn(move || {
            let pretty = PrettyConfig {
                depth_limit: 5,
                separate_tuple_members: true,
                enumerate_arrays: true,
                ..PrettyConfig::default()
            };
            let mut players = Vec::new();
            loop {
                let (command, message) = reciever.recv().unwrap();
                match command {
                    New(name) => {players.push(Player::new(name)); say(message, "added player")},
                    Add(name, category, amount) => if has(&players, name) {players[name].add_stat(category, amount); say(message, "stat added")} else {say(message, "not a valid player")},
                    Spend(name, category, amount) => if has(&players, name) {match players[name].spend(category.clone(), amount) {
                        Ok(state) => match state {
                            Exhausted(category) => say(message, format!("exhausted of all {}", category)),
                            _ => say(message, format!("{} spent successfully", category)),
                        },
                        Err(m) => say(message, m),
                    }} else {say(message, "not a valid player")},
                    Drain(name, category, amount) => if has(&players, name) {match players[name].spend_single(category.clone(), amount) {
                        Ok(state) => match state {
                            Exhausted(category) => say(message, format!("exhausted of all {}", category)),
                            _ => say(message, format!("{} spent successfully", category)),
                        },
                        Err(m) => say(message, m),
                    }} else {say(message, "not a valid player")},
                    Damage(name, amount) => if has(&players, name) {match players[name].damage(amount) {
                        Dead => {players.remove(name); say(message, format!("{} just straight up died", players[name].name))},
                        Fight => say(message, format!("{} has entered fight or flight", players[name].name)),
                        _ => say(message, "ouch")
                    }} else {say(message, "not a valid player")},
                    Sleep(name) => if has(&players, name) {match players[name].sleep() {
                        LevelUp => say(message, format!("{} has leveled up", players[name].name)),
                        _ => say(message, format!("{} slept", players[name].name)),
                    }} else {say(message, "not a valid player")},
                    Meal(name) => if has(&players, name) {match players[name].eat_meal() {
                        LevelUp => say(message, format!("{} has leveled up", players[name].name)),
                        _ => say(message, format!("{} ate a full meal", players[name].name)),
                    }} else {say(message, "not a valid player")},
                    Snack(name) => if has(&players, name) {match players[name].eat_snack() {
                        LevelUp => say(message, format!("{} has leveled up.\nFrom a snack.\nHOW!?", players[name].name)),
                        _ => say(message, format!("{} ate a snack", players[name].name)),
                    }} else {say(message, "not a valid player")},
                    Restore(name, category, amount) => if has(&players, name) {
                        players[name].restore(category.clone(), amount);
                        say(message, format!("restored {} {} to {}", amount, category, players[name].name));
                    } else {say(message, "not a valid player")},
                    End(name) => if has(&players, name) {
                        players[name].restore("energy".to_string(), 3);
                        say(message, format!("restored 3 energy to {}", players[name].name));
                    } else {say(message, "not a valid player")},
                    Print(name) => if has(&players, name) {say(message, &players[name])} else {say(message, "not a valid player")},
                    Duplicate(first, new_name) => if has(&players, first) {
                        let mut new_player = players[first].clone();
                        new_player.name = new_name;
                        players.push(new_player);
                    } else {say(message, "not a valid player")},
                    Save => File::create("characters.ron").unwrap().write_all(to_string_pretty(&players, pretty.clone()).unwrap().as_bytes()).unwrap(),
                    Load => players = from_reader(File::open("characters.ron").unwrap()).unwrap(),
                    Invalid(mess) => say(message, mess),
                }
            }
        });
        Gateway(sender)
    }
    fn send(&self, command: Command, message: Message) {
        self.0.send((command, message)).unwrap();
    }
}

fn has<T>(vec: &Vec<T>, name: usize) -> bool {
        return name < vec.len()
    }

pub enum Command {
    New(String),
    Add(usize, String, u8),
    Spend(usize, String, u8),
    Drain(usize, String, u8),
    Damage(usize, u8),
    Sleep(usize),
    Meal(usize),
    Snack(usize),
    Restore(usize, String, u8),
    End(usize),
    Print(usize),
    Duplicate(usize, String),
    Save,
    Load,
    Invalid(String),
}
use Command::*;