use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct BlogPost {
    id: i32,
    date: String,
    title: String,
    body: String,
    author: String,
}

#[derive(Parser, Debug)]
#[clap(name = "blog_client", version = "1.0", author = "Your Name")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    #[clap(name = "list")]
    List,
    #[clap(name = "show")]
    Show { id: i32 },
    #[clap(name = "create")]
    Create {
        title: String,
        body: String,
        author: String,
    },
    #[clap(name = "delete")]
    Delete { id: i32 },
}

/// cargo audit
#[tokio::main]
async fn main() {
    let args = Opts::parse();
    match args.subcmd {
        SubCommand::List => {
            let posts = reqwest::get("http://localhost:3001/")
                .await
                .unwrap()
                .json::<Vec<BlogPost>>()
                .await
                .unwrap();

            for post in posts {
                println!("{}: {}", post.id, post.title)
            }
        }
        _ => println!("Run with --help for help"),
    }
}
