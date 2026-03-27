use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use sqlx::migrate::Migrator;
use sqlx::postgres::PgPoolOptions;
use sqlx::{FromRow, PgPool};

use crate::error::{Error, Result};

const MIGRATIONS_DIR: &str = "template/axum-forge-template/migrations";

pub fn add(name: &str) -> Result<()> {
    validate_name(name)?;

    let migrations_dir = migrations_dir();
    fs::create_dir_all(&migrations_dir)?;

    let next_number = next_migration_number(&migrations_dir)?;
    let prefix = format!("{next_number:04}_{name}");

    let up_path = migrations_dir.join(format!("{prefix}.up.sql"));
    let down_path = migrations_dir.join(format!("{prefix}.down.sql"));

    fs::write(&up_path, up_template(name))?;
    fs::write(&down_path, down_template(name))?;

    println!("Created migration files:");
    println!("  {}", up_path.display());
    println!("  {}", down_path.display());

    Ok(())
}

pub async fn up(database_url: Option<String>) -> Result<()> {
    let pool = connect_pool(database_url).await?;
    let migrator = load_migrator().await?;

    migrator.run(&pool).await?;

    println!("Migrations are up to date.");
    print_current_version(&pool).await?;

    Ok(())
}

pub async fn down(database_url: Option<String>) -> Result<()> {
    let pool = connect_pool(database_url).await?;
    let migrator = load_migrator().await?;

    let current = current_version(&pool).await?;

    let Some(current) = current else {
        println!("No applied migrations.");
        return Ok(());
    };

    let target = current - 1;
    migrator.undo(&pool, target).await?;

    println!("Last migration reverted successfully.");
    print_current_version(&pool).await?;

    Ok(())
}

pub async fn status(database_url: Option<String>) -> Result<()> {
    let pool = connect_pool(database_url).await?;
    let applied = applied_migrations(&pool).await?;

    if applied.is_empty() {
        println!("No applied migrations.");
    } else {
        println!("Applied migrations:");
        for migration in applied {
            println!(
                "  {:04} | success={} | {}",
                migration.version,
                migration.success,
                migration.description
            );
        }
    }

    match current_version(&pool).await? {
        Some(version) => println!("Current version: {version}"),
        None => println!("Current version: none"),
    }

    Ok(())
}

async fn connect_pool(database_url: Option<String>) -> Result<PgPool> {
    let database_url = resolve_database_url(database_url)?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    Ok(pool)
}

fn resolve_database_url(database_url: Option<String>) -> Result<String> {
    match database_url {
        Some(url) => Ok(url),
        None => env::var("DATABASE_URL").map_err(|_| Error::MissingDatabaseUrl),
    }
}

async fn load_migrator() -> Result<Migrator> {
    let migrator = Migrator::new(Path::new(MIGRATIONS_DIR)).await?;
    Ok(migrator)
}

fn migrations_dir() -> PathBuf {
    PathBuf::from(MIGRATIONS_DIR)
}

fn next_migration_number(dir: &Path) -> Result<u32> {
    let mut max_number = 0u32;

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();

        if let Some(number) = parse_migration_prefix(&file_name) {
            max_number = max_number.max(number);
        }
    }

    Ok(max_number + 1)
}

fn parse_migration_prefix(file_name: &str) -> Option<u32> {
    let prefix = file_name.split('_').next()?;

    if prefix.len() != 4 || !prefix.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }

    prefix.parse().ok()
}

fn validate_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(Error::InvalidMigrationName(
            "name cannot be empty".to_string(),
        ));
    }

    let valid = name
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_');

    if !valid {
        return Err(Error::InvalidMigrationName(
            "use only lowercase letters, digits, and underscores".to_string(),
        ));
    }

    Ok(())
}

fn up_template(name: &str) -> String {
    format!("-- Migration: {name}\n-- Write your UP migration here.\n")
}

fn down_template(name: &str) -> String {
    format!("-- Migration: {name}\n-- Write your DOWN migration here.\n")
}

#[derive(Debug, FromRow)]
struct AppliedMigration {
    version: i64,
    description: String,
    success: bool,
}

async fn applied_migrations(pool: &PgPool) -> Result<Vec<AppliedMigration>> {
    let rows = sqlx::query_as::<_, AppliedMigration>(
        r#"
        SELECT version, description, success
        FROM _sqlx_migrations
        ORDER BY version
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

async fn current_version(pool: &PgPool) -> Result<Option<i64>> {
    let version = sqlx::query_scalar::<_, Option<i64>>(
        r#"
        SELECT MAX(version)
        FROM _sqlx_migrations
        WHERE success = TRUE
        "#,
    )
    .fetch_one(pool)
    .await?;

    Ok(version)
}

async fn print_current_version(pool: &PgPool) -> Result<()> {
    match current_version(pool).await? {
        Some(version) => println!("Current migration version: {version}"),
        None => println!("Current migration version: none"),
    }

    Ok(())
}