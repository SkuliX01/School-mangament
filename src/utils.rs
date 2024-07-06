use sqlx::{Pool, Postgres, Row};
use std::io;

pub fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
}

pub async fn register_student(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    let first_name = get_input("Enter first name:").trim().to_string();
    
    let last_name = get_input("Enter last name:").trim().to_string();

    let student_age: i32;
    loop {
        let age_input = get_input("Enter age:");
        match age_input.trim().parse::<i32>() {
            Ok(age) if age >= 0 => {
                student_age = age;
                break;
            },
            Ok(_) => {
                println!("Invalid age. Please enter a valid non-negative number.");
            },
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
            }
        }
    }

    let insert_query = "INSERT INTO students (first_name, last_name, student_age) VALUES ($1, $2, $3)";

    sqlx::query(insert_query)
        .bind(first_name)
        .bind(last_name)
        .bind(student_age)
        .execute(pool)
        .await?;

    println!("Succesfully added new Student \n");

    Ok(())
}

pub async fn get_all_students(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {

    let rows = sqlx::query("SELECT * FROM students")
        .fetch_all(pool)
        .await?;

    for row in rows {

        let id: i32 = row.get("id");
        let first_name: &str = row.get("first_name");
        let last_name: &str = row.get("last_name");
        let student_age: i32 = row.get("student_age");

        println!("\n Student ID: {}, First Name: {}, Last Name: {}, Student Age: {}", id, first_name, last_name, student_age)
    }

    Ok(())

}

pub async fn remove_student_by_id(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    
    let student_id: i32;

    loop {
        let id_input = get_input("Enter student ID you want to remove:");
        match id_input.trim().parse::<i32>() {
            Ok(id) if id >= 0 => {
                student_id = id;
                break;
            },
            Ok(_) => {
                println!("Invalid age. Please enter a valid non-negative number.");
            },
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
            }
        }
    }

    let delete_query = "DELETE FROM students WHERE id = $1";

    sqlx::query(delete_query)
        .bind(student_id)
        .execute(pool)
        .await?;

    Ok(())

}

pub async fn see_all_teachers(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {

    let rows = sqlx::query("SELECT * FROM teachers")
        .fetch_all(pool)
        .await?;

    for row in rows {

        let id: i32 = row.get("id");
        let first_name: &str = row.get("first_name");
        let last_name: &str = row.get("last_name");
        let student_age: i32 = row.get("teacher_age");

        println!("\n Teacher ID: {}, First Name: {}, Last Name: {}, Student Age: {}", id, first_name, last_name, student_age)
    }

    Ok(())
}


pub async fn register_teacher(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {

    let first_name = get_input("Enter Teacher first name").trim().to_string();

    let last_name = get_input("Enter Teacher last name: ").trim().to_string();

    let teacher_age: i32;
    loop {
        let age_input = get_input("Enter Teacher age:");
        match age_input.trim().parse::<i32>() {
            Ok(age) if age >= 0 => {
                teacher_age = age;
                break;
            },
            Ok(_) => {
                println!("Invalid age. Please enter a valid non-negative number.");
            },
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
            }
        }
    }

    let insert_query = "INSERT INTO teachers (first_name, last_name, teacher_age) VALUES ($1, $2, $3)";

    sqlx::query(insert_query)
        .bind(first_name)
        .bind(last_name)
        .bind(teacher_age)
        .execute(pool)
        .await?;

    println!("Succesfully added new teacher");

    Ok(())

}

pub async fn remove_teacher_by_id(pool: &Pool<Postgres>) -> Result<(), sqlx::Error>{

    let teacher_id: i32;

    loop {
        let id_input = get_input("Enter teacher ID you want to remove:");
        match id_input.trim().parse::<i32>() {
            Ok(id) if id >= 0 => {
                teacher_id = id;
                break;
            },
            Ok(_) => {
                println!("Invalid age. Please enter a valid non-negative number.");
            },
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
            }
        }
    }


    let delete_query = "DELETE FROM teachers WHERE id = $1";

    sqlx::query(delete_query)
        .bind(teacher_id)
        .execute(pool)
        .await?;

    Ok(())

}

pub async fn add_teacher_absence(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {

    let teacher_first_name = get_input("Enter Teacher First Name: ").trim().to_string();
    let teacher_last_name = get_input("Enter Teacher Last Name: ").trim().to_string();
    let absence_begin_date = get_input("Enter Teacher Absence Begin Date (day/month/year) format: ").trim().to_string();
    let absence_end_date = get_input("Enter Teacher Absence End Date (day/month/year) format: ").trim().to_string();

    let begin_clone = absence_begin_date.clone();
    let end_clone = absence_end_date.clone();

    let insert_query = "INSERT INTO teachers_absence (first_name, last_name, begin_date, end_date) VALUES ($1, $2, $3, $4)";

    sqlx::query(insert_query)
        .bind(teacher_first_name)
        .bind(teacher_last_name)
        .bind(absence_begin_date)
        .bind(absence_end_date)
        .execute(pool)
        .await?;

    println!("Succesfully Created new teachers Absence in days {} - {}", begin_clone, end_clone);

    Ok(())
}
