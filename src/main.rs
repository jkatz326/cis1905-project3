use clap::{Parser, Subcommand};
use ngram::client::Client;
use ngram::server::Server;

// Fill out the `Args` struct to parse the command line arguments. You may find clap "subcommands"
// helpful.
/// An archive service allowing publishing and searching of books
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    mode: Mode,
}

// First need to determine if command is client or server
#[derive(Subcommand, Debug)]
enum Mode {
    Client(ClientArgs),
    Server(ServerArgs),
}

// If client need an address, port, and one of the three requests below
#[derive(Parser, Debug)]
struct ClientArgs {
    address: String,
    port: u16,
    #[command(subcommand)]
    request: Request,
}

#[derive(Subcommand, Debug)]
enum Request {
    Publish {
        path: String,
    },
    Search {
        word: String,
    },
    Retrieve {
        doc_id: usize,
    },
}

// Else, just need port, only one server command
#[derive(Parser, Debug)]
struct ServerArgs {
    port: u16,
}


// Inspect the contents of the `args` struct that has been created from the command line arguments
// the user passed. Depending on the arguments, either start a server or make a client and send the
// appropriate request. You may find it helpful to print the request response.
fn main() {
    let args = Args::parse();
    match args.mode {
        // Client mode
        Mode::Client(client_args) => { 
            println!("Connecting to server at {}:{}...", client_args.address, client_args.port);
            let client = Client::new(&client_args.address, client_args.port);
            match client_args.request {
                Request::Publish { path }=> {
                    println!("Sending PUBLISH request for: {}", path);
                    match client.publish_from_path(&path) {
                        Some(response) => println!("Server response: {:?}", response),
                        None => eprintln!("Error: Failed to get response from server."),
                    }
                }
                Request::Search { word }=> {
                    println!("Sending SEARCH request for: {}", word);
                    match client.search(&word) {
                        Some(response) => println!("Server response: {:?}", response),
                        None => eprintln!("Error: Failed to get response from server."),
                    }
                }
                Request::Retrieve { doc_id }=> {
                    println!("Sending RETRIEVE request for: {}", doc_id);
                    match client.retrieve(doc_id) {
                        Some(response) => println!("Server response: {:?}", response),
                        None => eprintln!("Error: Failed to get response from server."),
                    }
                }
            }
        }
        // Server mode
        Mode::Server(server_args) => {
            println!("Starting server on port {}...", server_args.port);
            let server = Server::new();
            server.run(server_args.port);
        }

    }
}
