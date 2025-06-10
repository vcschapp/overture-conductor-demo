use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "conductor", about = "Overture schema CLI", version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(aliases = ["schem", "s"])]
    Schema {
        #[command(subcommand)]
        subcommand: SchemaCommand,
    },
    Data {
        #[command(subcommand)]
        subcommand: DataCommand,
    },
}

#[derive(Subcommand)]
enum SchemaCommand {
    Init,
    Publish,
    Transpile,
    Validate,
}

#[derive(Subcommand)]
enum DataCommand {
    Convert,
    Validate,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Command::Schema { subcommand } => schema(subcommand),
        Command::Data { subcommand } => data(subcommand),
    }
}

fn schema(command: &SchemaCommand) {

}

fn data(command: &DataCommand) {

}
