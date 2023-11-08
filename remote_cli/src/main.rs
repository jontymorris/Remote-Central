use clap::Parser;
use dotenvy::dotenv;
use remote_core::{
    add_command_for_ip, get_commands_done_for_ip, get_commands_queue_for_ip, get_recent_clients,
};

#[derive(Parser, Debug)]
#[command(author = None, version = None, about = "Remote Central CLI", long_about = None)]
struct Args {
    #[arg(long, help = "List the connected clients")]
    clients: bool,

    #[arg(long, help = "List the command history for an IP")]
    history: bool,

    #[arg(long, help = "Specify a client IP")]
    ip: Option<String>,

    #[arg(long, help = "Specify a client command to issue")]
    command: Option<String>,
}

fn main() {
    dotenv().ok();

    let args = Args::parse();

    if args.clients {
        print_clients();
    }

    if args.history {
        print_client_history(args.ip.clone().expect("IP must be specified"));
    }

    if args.command.is_some() {
        add_client_command(
            args.ip.clone().expect("IP must be specified"),
            args.command.unwrap(),
        );
    }
}

fn print_clients() {
    let clients = get_recent_clients();
    let now = chrono::Utc::now();

    if clients.is_empty() {
        println!("No connected clients");
        return;
    }

    println!("Connected clients:");
    for client in clients {
        let difference = now - client.date;
        println!("\t{} (seen {}s ago)", client.ip, difference.num_seconds());
    }

    println!("");
}

fn print_client_history(ip: String) {
    let waiting_commands = get_commands_queue_for_ip(&ip);
    let done_commands = get_commands_done_for_ip(&ip);
    let now = chrono::Utc::now();

    if !waiting_commands.is_empty() {
        println!("Commands waiting:");

        for command in waiting_commands {
            let difference = now - command.date;
            println!("\t{} ({}s ago)", command.command, difference.num_seconds());
        }

        println!("");
    }

    if !done_commands.is_empty() {
        println!("Done commands:");

        for command in done_commands {
            let difference = now - command.date;
            println!(
                "\t{} ({}s ago): {}",
                command.command,
                difference.num_seconds(),
                command.output.unwrap_or_default()
            );
        }

        println!("");
    }
}

fn add_client_command(ip: String, command: String) {
    println!("Adding command to {}:\n\t{}", ip, command);
    add_command_for_ip(ip, command);
}
