use std::env;
use std::error::Error;
use clap::{Parser, Subcommand};
use sqlx::PgPool;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    List,
    Create {
        description: String,
    },
    Done {
        id: i64
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    let cli = Cli::parse();
    let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;

    match &cli.command {
        Some(Commands::List) => {
            get_items(&pool).await?
        },
        Some(Commands::Create {description}) => {
            println!("Creating a new TODO item with description: {description}");
            let id = create_todo(&pool, description.to_string()).await?;
            println!("Created TODO item: {id}")
        },
        Some(Commands::Done {id}) => {
            println!("Marking todo {id} as done");
            complete_todo(&pool, *id).await?;
        },
        None => {
            println!("That is not a real command")
        }
    }

    return Ok(());
}

async fn get_items(pool: &PgPool) -> Result<(), sqlx::Error> {
    let recs = sqlx::query!(
        r#"
        SELECT *
        FROM todo_items
        ORDER BY id
        "#,
    )
        .fetch_all(pool)
        .await?;

    for rec in recs {
        println!(
            "- [{}] {}: {}",
            if rec.complete.unwrap() { "x" } else { " " },
            rec.id,
            &rec.description,
        );
    }

    Ok(())
}

async fn create_todo(pool: &PgPool, description: String) -> Result<i64, sqlx::Error> {
    let rec = sqlx::query!(r#"
    INSERT INTO todo_items ( description )
    VALUES ( $1 )
    RETURNING id
    "#,
    description
    )
        .fetch_one(pool)
        .await?;

    Ok(rec.id)
}

async fn complete_todo(pool: &PgPool, id: i64) -> Result<bool, sqlx::Error> {
    let affected = sqlx::query!(
        r#"
        UPDATE todo_items
        SET complete = TRUE
        WHERE id = $1
        "#,
        id
    )
        .execute(pool)
        .await?
        .rows_affected();

    Ok(affected > 0)
}