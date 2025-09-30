use clap::Parser;
use config::{Config, File};
use serde::{Deserialize, Serialize};

#[derive(Debug, Parser)]
#[clap(version, about = "Prints its configuration to STDOUT.")]
struct Cli {
    /// enables debug mode
    #[clap(short, long)]
    debug: bool,
    /// path to configuration file
    #[clap(short, long, default_value = "config.toml", env = "CONF_FILE")]
    conf: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Database {
    username: String,
    password: String,
    database_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AppConfig {
    debug: bool,
    database: Database,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            debug: false,
            database: Database {
                username: "postgres".to_string(),
                password: "password".to_string(),
                database_name: "mydb".to_string(),
            },
        }
    }
}

impl std::fmt::Display for AppConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Application Configuration:")?;
        writeln!(f, "  Debug: {}", self.debug)?;
        writeln!(f, "  Database:")?;
        writeln!(f, "    Username: {}", self.database.username)?;
        writeln!(
            f,
            "    Password: {}",
            if self.debug {
                &self.database.password
            } else {
                "***"
            }
        )?;
        writeln!(
            f,
            "    Database name: {}",
            if self.debug {
                &self.database.database_name
            } else {
                "***"
            }
        )?;
        Ok(())
    }
}

impl AppConfig {
    fn load(cli: &Cli) -> Result<Self, config::ConfigError> {
        let default_config = Self::default();
        let config_builder = Config::builder()
            // default values
            .set_default("debug", default_config.debug)?
            .set_default("database.username", default_config.database.username)?
            .set_default("database.password", default_config.database.password)?
            .set_default("database.database_name", default_config.database.database_name)?
            // toml file with config data
            .add_source(File::with_name(&cli.conf).required(false))
            // env vars with CONF prefix
            .add_source(config::Environment::with_prefix("CONF").separator("_"))
            // CLI flags - highest priority
            .set_override("debug", cli.debug)?;


        let config = config_builder.build()?;
        config.try_deserialize()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match AppConfig::load(&cli) {
        Ok(config) => {
            println!("{}", config);

            // also print as JSON for debugging if requested
            if cli.debug {
                println!("\nRaw JSON representation: ");
                println!("{}", serde_json::to_string_pretty(&config)?);
            }
        }
        Err(err) => {
            eprintln!("error loading configuration {}", err);
            std::process::exit(1);
        }
    }

    Ok(())
}
