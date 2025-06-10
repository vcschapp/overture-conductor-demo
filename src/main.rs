use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "conductor", about = "Overture schema CLI", version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(aliases = ["schem", "s"], about = "Interact with an Overture schema or schema extension")]
    Schema {
        #[command(subcommand)]
        subcommand: SchemaCommand,
    },
    #[command(aliases = ["dat", "dta", "d"], about = "Interact with data in an Overture schema or extension")]
    Data {
        #[command(subcommand)]
        subcommand: DataCommand,
    },
}

#[derive(Subcommand)]
enum SchemaCommand {
    #[command(aliases = ["ch", "ck", "c"], about = "Analyze this schema and report errors")]
    Check,
    #[command(aliases = ["initialize", "ini", "in", "i"], about = "Create a new Overture schema in the current directory")]
    Init,
    #[command(aliases = ["pub", "p"], about = "Package and upload this schema to the registry")]
    Publish,
    #[command(aliases = ["trans", "tr", "t"], about = "Convert this schema to a different representation")]
    Transpile,
}

#[derive(Subcommand)]
enum DataCommand {
    #[command(aliases = ["conv", "con", "c"], about = "Convert from one format to another")]
    Convert,
    #[command(aliases = ["valid", "val", "v"], about = "Verify that the data matches the schema")]
    Validate,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Command::Schema { subcommand } => schema(subcommand),
        Command::Data { subcommand } => data(subcommand),
    }
}

// ====================================================================
// SCHEMA
// ====================================================================

fn schema(command: &SchemaCommand) {

}

// ====================================================================
// DATA
// ====================================================================

fn data(command: &DataCommand) {

}
