use dotenvy::dotenv;
use remote_core::Command as RemoteCommand;
use remote_core::{Ping, Pong};
use reqwest::Client;
use std::env;
use std::process::Command as SystemCommand;

async fn ping() -> Result<Vec<RemoteCommand>, Box<dyn std::error::Error>> {
    let server_url = env::var("SERVER").expect("Server must be specified");

    let client = Client::new();
    let model = Ping {
        token: env::var("TOKEN").expect("Token must be specified"),
    };

    let res = client
        .post(format!("{}/ping", server_url))
        .json(&model)
        .send()
        .await?;

    let commands = res.json::<Vec<RemoteCommand>>().await?;

    Ok(commands)
}

async fn pong(commands: Vec<RemoteCommand>) -> Result<(), Box<dyn std::error::Error>> {
    let server_url = env::var("SERVER").expect("Server must be specified");

    let client = Client::new();
    let model = Pong {
        token: env::var("TOKEN").expect("Token must be specified"),
        commands,
    };

    client
        .post(format!("{}/pong", server_url))
        .json(&model)
        .send()
        .await?;

    Ok(())
}

fn run_commands(commands: &mut Vec<RemoteCommand>) {
    for command in commands.iter_mut() {
        let output = if cfg!(target_os = "windows") {
            SystemCommand::new("cmd")
                .args(&["/C", &command.command])
                .output()
        } else {
            SystemCommand::new("sh")
                .arg("-c")
                .arg(&command.command)
                .output()
        };

        match output {
            Ok(output) => {
                if output.status.success() {
                    let output_str = String::from_utf8_lossy(&output.stdout).to_string();
                    command.output = Some(output_str);
                } else {
                    let error_str = String::from_utf8_lossy(&output.stderr).to_string();
                    command.output = Some(error_str);
                }
            }
            Err(e) => {
                command.output = Some(format!("Failed to execute command: {}", e));
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    println!("Remote Central Client running");

    match ping().await {
        Ok(mut commands) => {
            println!("Received commands: {:?}", commands);

            if commands.is_empty() {
                return;
            }

            run_commands(&mut commands);

            if let Err(e) = pong(commands).await {
                eprintln!("Error sending pong: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Error pinging server: {}", e);
        }
    }
}
