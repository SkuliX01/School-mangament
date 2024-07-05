use sqlx::postgres::PgPoolOptions;
use sqlx::{QueryBuilder, Postgres};

mod utils;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("")
        .await?;
    
    let mut create_table_students_query: QueryBuilder<Postgres> = QueryBuilder::new(
        "CREATE TABLE IF NOT EXISTS students(id SERIAL PRIMARY KEY, first_name TEXT NOT NULL, last_name TEXT NOT NULL, student_age INT NOT NULL)"
    );

    let mut create_table_teachers_query: QueryBuilder<Postgres> = QueryBuilder::new(
        "CREATE TABLE IF NOT EXISTS teachers(id SERIAL PRIMARY KEY, first_name TEXT NOT NULL, last_name TEXT NOT NULL, teacher_age INT NOT NULL)"
    );

    create_table_students_query.build().execute(&pool).await?;
    create_table_teachers_query.build().execute(&pool).await?;

    println!("Welcome to School Management System!");

    loop {

        println!("Choose an Operation:");
        println!("1. Add new student");
        println!("2. Remove student");
        println!("3. See All Students");
        println!("4. Add Teacher");
        println!("5. Remove Teacher");
        println!("6. See All Teachers");
        println!("7. Add Teacher Absence");
        println!("8. Exit");

        let option = utils::get_input("Enter a Choice: ");

        if option.trim() == "8"  {
            println!("Exiting...");
            break;
        }

        match option.trim() {
            "1" => utils::register_student(&pool).await?,
            "2" => utils::remove_student_by_id(&pool).await?,
            "3" => utils::get_all_students(&pool).await?,
            "4" => utils::register_teacher(&pool).await?,
            "5" => utils::remove_teacher_by_id(&pool).await?,
            "6" => utils::see_all_teachers(&pool).await?,
            _ => println!("Invalid option. Please choose a valid operation."),
        }
    }

    Ok(())

}
