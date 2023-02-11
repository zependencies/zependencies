use directories::ProjectDirs;
use error_stack::{IntoReport, Report, ResultExt};
use serde_with::SerializeDisplay;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use thiserror::Error;
use tracing::Instrument;

#[derive(Error, SerializeDisplay, Debug)]
#[error("while initializing database")]
pub struct InitError;

/// Initialize the database.
///
/// This includes creating files and setting up its tables.
#[tracing::instrument(skip_all)]
pub async fn init(project_dirs: &ProjectDirs) -> Result<SqlitePool, Report<InitError>> {
    let db_url = create_database(project_dirs)
        .await
        .attach_printable("failed to create database file")?;
    let db = SqlitePool::connect(&db_url)
        .await
        .into_report()
        .change_context(InitError)
        .attach_printable_lazy(|| format!("failed to connect pool to {db_url:?}"))?;
    sqlx::migrate!()
        .run(&db)
        .await
        .into_report()
        .change_context(InitError)
        .attach_printable("failed to run migrations")?;
    Ok(db)
}

/// Create a database if it does not exist and return its URL.
async fn create_database(project_dirs: &ProjectDirs) -> Result<String, Report<InitError>> {
    let path = project_dirs.data_dir();
    std::fs::create_dir_all(path)
        .into_report()
        .change_context(InitError)
        .attach_printable_lazy(|| format!("failed to create directory {path:?}"))?;
    let file = path.join("sqlite.db");
    let db_url = file
        .to_str()
        .ok_or(InitError)
        .into_report()
        .attach_printable_lazy(|| format!("file path ({file:?}) is not printable as UTF-8"))?;
    if !Sqlite::database_exists(db_url)
        .await
        .into_report()
        .change_context(InitError)
        .attach_printable_lazy(|| {
            format!("failed to check whether database at {db_url:?} exists")
        })?
    {
        Sqlite::create_database(db_url)
            .instrument(tracing::info_span!("creating database file"))
            .await
            .into_report()
            .change_context(InitError)
            .attach_printable_lazy(|| format!("could not create database at {db_url:?}"))?;
    }
    Ok(db_url.to_string())
}
