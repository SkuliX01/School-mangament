use sqlx::postgres::PgPoolOptions;
use sqlx::{QueryBuilder, Postgres};

mod utils;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("")
        .await?;
    
    let mut create_table_query: QueryBuilder<Postgres> = QueryBuilder::new(
        "CREATE TABLE IF NOT EXISTS students(id SERIAL PRIMARY KEY, first_name TEXT NOT NULL, last_name TEXT NOT NULL, student_age INT NOT NULL)"
    );

    create_table_query.build().execute(&pool).await?;

    println!("Welcome to School Management System!");

    loop {

        println!("Choose an Operation:");
        println!("1. Add new student");
        println!("2. Remove student");
        println!("3. See all students");
        println!("4. Exit");

        let option = utils::get_input("Enter a Choice: ");

        if option.trim() == "5"  {
            println!("Exiting...");
            break;
        }

        match option.trim() {
            "1" => utils::register_student(&pool).await?,
            "3" => utils::get_all_students(&pool).await?,
            _ => println!("Invalid option. Please choose a valid operation."),
        }
    }

    Ok(())

}