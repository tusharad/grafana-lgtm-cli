use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name="grafana-lgtm-cli", version, about = "Natural language querying for Prometheus")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Ask {
        prompt: String
    },
    Serve {
        #[arg(short, long, default_value_t = 8080)]
        port: u16
    }
}
