use std::path::PathBuf;

use tui_setup_wizard::{SetupWizard, SetupWizardAnswer, SetupWizardError, SetupWizardStep};

#[derive(Debug)]
pub enum DatabaseConfig {
    Postgres { url: String },
    Sqlite { path: PathBuf },
}

#[derive(Debug)]
pub struct Config {
    pub logging_file_path: Option<PathBuf>,
    pub logging_stdio: bool,

    pub database: DatabaseConfig,

    pub port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            logging_file_path: None,
            logging_stdio: true,

            database: DatabaseConfig::Sqlite {
                path: PathBuf::from("./database.db"),
            },

            port: 3000,
        }
    }
}

fn main() -> Result<(), SetupWizardError> {
    let setup_wizard_steps = &[
        SetupWizardStep::info("Logging").build(),
        SetupWizardStep::enable("Use file logging?")
            .with_id("enable_logging")
            .build(),
        SetupWizardStep::text("Log file location:")
            .with_default_value("./output.log")
            .with_id("log_file_location")
            .apply_using(|config: &mut Config, file_path| {
                if !file_path.is_empty() {
                    config.logging_file_path = Some(PathBuf::from(file_path.clone()));
                }
            })
            .only_if("enable_logging", |answer| match answer {
                SetupWizardAnswer::Enable(enable_logging) => *enable_logging,
                _ => false,
            })
            .build(),
        SetupWizardStep::warning("No log file path specified, disabled file logging")
            .only_if("enable_logging", |answer| match answer {
                SetupWizardAnswer::Enable(enable_logging) => *enable_logging,
                _ => false,
            })
            .only_if("log_file_location", |answer| match answer {
                SetupWizardAnswer::Text(log_file_location) => log_file_location.is_empty(),
                _ => false,
            })
            .build(),
        SetupWizardStep::enable("Use console logging?")
            .with_default_value(true)
            .apply_using(|config: &mut Config, enable_logging| {
                config.logging_stdio = *enable_logging;
            })
            .build(),
        SetupWizardStep::info("Database").build(),
        SetupWizardStep::select("Backend", &["Postgres", "SQLite"])
            .with_id("database_backend")
            .build(),
        SetupWizardStep::text("Postgres connection URL:")
            .apply_using(|config: &mut Config, url| {
                config.database = DatabaseConfig::Postgres { url: url.clone() }
            })
            .validate_using(
                |url| !url.is_empty(),
                "You must input a connection URL for Postgres",
            )
            .validate_using(
                |url| url.starts_with("postgresql://"),
                "Connection URL must use the \"postgresql://\" format",
            )
            .only_if("database_backend", |answer| {
                matches!(answer, SetupWizardAnswer::Select(0))
            })
            .build(),
        SetupWizardStep::text("SQLite database location:")
            .with_default_value("./database.db")
            .apply_using(|config: &mut Config, file_path| {
                config.database = DatabaseConfig::Sqlite {
                    path: PathBuf::from(file_path.clone()),
                }
            })
            .validate_using(
                |file_path| !file_path.is_empty(),
                "You must input a file path for SQLite",
            )
            .only_if("database_backend", |answer| {
                matches!(answer, SetupWizardAnswer::Select(1))
            })
            .build(),
        SetupWizardStep::info("Web Server").build(),
        SetupWizardStep::unsigned_number("Port:")
            .with_default_value(3000)
            .validate_using(|number| *number >= 1, "Ports must be between 1 and 65535")
            .validate_using(
                |number| *number <= 65535,
                "Ports must be between 1 and 65535",
            )
            .apply_using(|config: &mut Config, port| {
                config.port = u16::try_from(*port).unwrap_or(3000);
            })
            .build(),
    ];

    let setup_wizard =
        SetupWizard::<Config>::new(setup_wizard_steps).with_title("Example Server Setup");

    let final_config = setup_wizard.run(Config::default())?;

    println!();
    println!("{final_config:#?}");

    Ok(())
}
