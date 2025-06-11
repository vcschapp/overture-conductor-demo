use clap::{Parser, Subcommand, ValueEnum, ValueHint};
use std::path::PathBuf;

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
    Transpile {
        #[command(subcommand)]
        subcommand: TranspileCommand,
    }
}

#[derive(Subcommand)]
enum DataCommand {
    #[command(aliases = ["conv", "con", "c"], about = "Convert from one format to another")]
    Convert,
    #[command(aliases = ["sample", "sam", "exam", "e"], about = "Generate example dataset in a supported data format")]
    Example {
        #[command(subcommand)]
        subcommand: ExampleCommand,
    },
    #[command(aliases = ["valid", "val", "v"], about = "Verify that the data matches the schema")]
    Validate,
}

#[derive(Subcommand)]
enum TranspileCommand {
    #[command(aliases = ["docusaurus", "mdx"], about = "Docusaurus-flavored markdown (`.mdx`)")]
    DocDocusaurus,
    #[command(aliases = ["json", "geojson", "geo-json"], about = "JSON Schema validating Overture-flavored GeoJSON")]
    SchemaGeoJson,
    #[command(aliases = ["spark"], about = "Spark schema in PySpark or Scala dialect")]
    SchemaSpark {
        #[clap(short, long, help = "Spark dialect", value_hint = ValueHint::Other)]
        dialect: SparkDialect,
        #[clap(short, long, help = "Output directory (default: current directory)", value_hint = ValueHint::DirPath, value_name = "DIR")]
        output_dir: Option<PathBuf>,
    },
    #[command(aliases = ["java"], about = "Java language SDK (Scala-compatible) with optional serde support")]
    SdkJava {
        #[clap(short, long, help = "Output directory (default: current directory)", value_hint = ValueHint::DirPath, value_name = "DIR")]
        output_dir: Option<PathBuf>,
        #[clap(short, long, value_delimiter = ',', help = "Serde support", value_hint = ValueHint::Other)]
        serde: Vec<SerdeKind>,
    },
    #[command(aliases = ["python", "py"], about = "Python language SDK with optional serde support")]
    SdkPython,
    #[command(about = "Spark-based distributed validation script in PySpark or Scala dialect")]
    ValidateSpark,
}

#[derive(Subcommand)]
enum ExampleCommand {
    #[command(aliases = ["parquet", "parq", "par", "p"], about = "Sample GeoParquet file")]
    GeoParquet,
    #[command(aliases = ["json", "j"], about = "Sample GeoJSON features")]
    GeoJson,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
#[clap(rename_all = "kebab-case")]
enum SparkDialect {
    PySpark,
    Scala,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
#[clap(rename_all = "kebab-case")]
enum SerdeKind {
    GeoJson,
    GeoParquet,
    SparkDataframe,
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
