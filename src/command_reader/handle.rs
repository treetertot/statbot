extern crate serenity;

use super::{Gateway, Command::*};
use serenity::prelude::*;
use serenity::model::{channel::Message, gateway::Ready};

impl EventHandler for Gateway {
    fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
        println!("ready");
    }

    fn message(&self, _ctx: Context, message: Message) {
        if &message.content[0..1] != "!" {
            return;
        }
        let resliced = reslice(&message.content);
        match resliced[0].as_str() {
            "!n" => {
                if resliced.len() > 1 {
                    self.send(New(resliced[1].clone()), message);
                } else {
                    self.send(Invalid("too short".to_string()), message)
                }
            },
            "!a" => {
                if resliced.len() > 3 {
                    match resliced[1].parse() {
                        Ok(value_a) => match resliced[3].parse() {
                            Ok(value_b) => self.send(Add(value_a, resliced[2].clone(), value_b), message),
                            Err(_) => self.send(Invalid("quantity not a number".to_string()), message),
                        },
                        Err(_) => self.send(Invalid("playerID not a number".to_string()), message),
                    }
                } else {
                    self.send(Invalid("too short".to_string()), message)
                }
            },
            "!sp" => {
                if resliced.len() > 3 {
                    match resliced[1].parse() {
                        Ok(value_a) => match resliced[3].parse() {
                            Ok(value_b) => self.send(Spend(value_a, resliced[2].clone(), value_b), message),
                            Err(_) => self.send(Invalid("quantity not a number".to_string()), message),
                        },
                        Err(_) => self.send(Invalid("playerID not a number".to_string()), message),
                    }
                } else {
                    self.send(Invalid("too short".to_string()), message)
                }
            },
            "!dr" => {
                if resliced.len() > 3 {
                    match resliced[1].parse() {
                        Ok(value_a) => match resliced[3].parse() {
                            Ok(value_b) => self.send(Drain(value_a, resliced[2].clone(), value_b), message),
                            Err(_) => self.send(Invalid("quantity not a number".to_string()), message),
                        },
                        Err(_) => self.send(Invalid("playerID not a number".to_string()), message),
                    }
                } else {
                    self.send(Invalid("too short".to_string()), message)
                }
            },
            "!d" => {
                if resliced.len() > 2 {
                    match resliced[1].parse() {
                        Ok(value_a) => match resliced[2].parse() {
                            Ok(value_b) => self.send(Damage(value_a, value_b), message),
                            Err(_) => self.send(Invalid("quantity not a number".to_string()), message),
                        },
                        Err(_) => self.send(Invalid("playerID not a number".to_string()), message),
                    }
                } else {
                    self.send(Invalid("too short".to_string()), message)
                }
            },
            "!sl" => {
                if resliced.len() > 1 {
                    match resliced[1].parse() {
                        Ok(value_a) => self.send(Sleep(value_a), message),
                        Err(_) => self.send(Invalid("playerID not a number".to_string()), message),
                    }
                } else {
                    self.send(Invalid("too short".to_string()), message)
                }
            },
            "!m" => {
                if resliced.len() > 1 {
                    match resliced[1].parse() {
                        Ok(value_a) => self.send(Meal(value_a), message),
                        Err(_) => self.send(Invalid("playerID not a number".to_string()), message),
                    }
                } else {
                    self.send(Invalid("too short".to_string()), message)
                }
            },
            "!sn" => {
                if resliced.len() > 1 {
                    match resliced[1].parse() {
                        Ok(value_a) => self.send(Snack(value_a), message),
                        Err(_) => self.send(Invalid("playerID not a number".to_string()), message),
                    }
                } else {
                    self.send(Invalid("too short".to_string()), message)
                }
            },
            "!e" => {
                if resliced.len() > 1 {
                    match resliced[1].parse() {
                        Ok(value_a) => self.send(End(value_a), message),
                        Err(_) => self.send(Invalid("playerID not a number".to_string()), message),
                    }
                } else {
                    self.send(Invalid("too short".to_string()), message)
                }
            },
            "!p" => {
                if resliced.len() > 1 {
                    match resliced[1].parse() {
                        Ok(value_a) => self.send(Print(value_a), message),
                        Err(_) => self.send(Invalid("playerID not a number".to_string()), message),
                    }
                } else {
                    self.send(Invalid("too short".to_string()), message)
                }
            },
            "!r" => {
                if resliced.len() > 3 {
                    match resliced[1].parse() {
                        Ok(value_a) => match resliced[3].parse() {
                            Ok(value_b) => self.send(Restore(value_a, resliced[2].clone(), value_b), message),
                            Err(_) => self.send(Invalid("quantity not a number".to_string()), message),
                        },
                        Err(_) => self.send(Invalid("playerID not a number".to_string()), message),
                    }
                } else {
                    self.send(Invalid("too short".to_string()), message)
                }
            },
            "!c" => {
                if resliced.len() > 2 {
                    match resliced[1].parse() {
                        Ok(value_a) => self.send(Duplicate(value_a, resliced[2].clone()), message),
                        Err(_) => self.send(Invalid("playerID not a number".to_string()), message),
                    }
                } else {
                    self.send(Invalid("too short".to_string()), message)
                }
            },
            "!save" => self.send(Save, message),
            "!load" => self.send(Load, message),
            "!h" => self.send(Invalid("n -> new player\na -> add stat\nsp -> spend\ndr -> drain\nd -> damage\nsl -> sleep\nm -> meal\nsn -> snack\ne -> end\np -> print stats\nr -> restore lost points".to_string()), message),
            _ => self.send(Invalid("Invalid command".to_string()), message),
        }
    }
}

fn reslice(s: &str) -> Vec<String> {
    let mut word = String::new();
    let mut sentence = Vec::new();
    for chr in s.chars() {
        match chr {
            ' ' => {
                sentence.push(word);
                word = String::new();
            },
            _ => word.push(chr),
        }
    }
    sentence.push(word);
    sentence
}