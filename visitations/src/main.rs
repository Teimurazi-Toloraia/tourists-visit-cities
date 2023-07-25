use std::fs::{self};
use text_io::read;

use typedb_client::{
    Connection, DatabaseManager, Options, Session,
    SessionType::{Data, Schema},
    Transaction,
    TransactionType::{Read, Write},
};

const DATABASE: &str = "database describing tourists visiting cities from countries";

fn new_core_connection() -> typedb_client::Result<Connection> {
    Connection::new_plaintext("localhost:1729")
}

fn get_schema() -> String {
    fs::read_to_string("./src/schema.tql").expect("Failed to read schema file")
}

fn create_country_entity(name: &str) -> String {
    format!("insert $x isa country, has name \"{name}\";")
}

fn create_city_entity(name: &str) -> String {
    format!("insert $x isa city, has name \"{name}\";")
}

fn create_person_entity(firstname: &str, lastname: &str) -> String {
    format!("insert $x isa person, has firstname \"{firstname}\", has lastname \"{lastname}\";")
}

async fn create_schema(connection: Connection) -> typedb_client::Result {
    let schema = get_schema();
    let databases = DatabaseManager::new(connection.clone());
    if databases.contains(DATABASE).await? {
        println!("\nSchema Already Defined\n");
    } else {
        databases.create(DATABASE).await?;
        let session = Session::new(databases.get(DATABASE).await?, Schema).await?;
        let transaction = session.transaction(Write).await?;
        transaction.query().define(schema.as_str()).await?;
        transaction.commit().await?;
        println!("\nSchema Defined Successfully\n");
    }

    Ok(())
}

fn create_visited_relationship(
    start: i32,
    end: i32,
    first_name: &str,
    last_name: &str,
    city: &str,
    month_id: i32,
) -> String {
    format!(
        r#"match $person isa person, has firstname "{first_name}", has lastname "{last_name}";
    $city1 isa city, has name "{city}";
    insert (person: $person, destination: $city1) isa visited, has start_date 2023-0{month_id}-{start}, has end_date 2023-0{month_id}-{end};"#
    )
}

fn insert_data(
    transaction: &Transaction<'_>,
    first_name: &str,
    last_name: &str,
    cities: Vec<&str>,
    end: i32,
    start: i32,
    months: Vec<i32>,
) {
    for (city, month_id) in cities.iter().zip(months.iter()) {
        let _ = transaction.query().insert(
            &create_visited_relationship(start, end, first_name, last_name, city, *month_id)
                .as_str(),
        );
    }
}

async fn create_data(connection: Connection) -> typedb_client::Result {
    let databases = DatabaseManager::new(connection.clone());
    // insert data
    let session = Session::new(databases.get(DATABASE).await?, Data).await?;
    let transaction = session.transaction(Write).await?;
    let cities = ["Tbilisi", "London", "Cambridge", "Zugdidi"];
    cities
        .iter()
        .map(|city| transaction.query().insert(&create_city_entity(city)))
        .count();
    let countries = ["Georgia", "United Kingdom", "United States of America"];
    countries
        .iter()
        .map(|country| transaction.query().insert(&create_country_entity(&country)))
        .count();
    // Georgians

    let geo_first_names = ["Teimurazi", "Natalia", "Luka", "Irakli", "Giorgi", "Robiko"];
    let geo_last_names = ["Toloraia", "Skhulukhia", "Tsintsadze"];
    let mut duration = 0;
    let mut start = 9;
    for first_name in geo_first_names {
        start += 1;
        for last_name in geo_last_names {
            if start <= 12 {
                duration += 1;
            }
            let _ = transaction
                .query()
                .insert(&create_person_entity(first_name, last_name));
            // Insert visits
            let end = start + duration;
            let cities: Vec<&str> = ["Cambridge", "London"].to_vec();
            let months: Vec<i32> = [1, 3].to_vec();
            insert_data(
                &transaction,
                first_name,
                last_name,
                cities,
                end,
                start,
                months,
            );
        }
    }

    // UK

    duration = 3;
    start = 9;
    let uk_first_names = ["James", "Olivia", "William", "Amelia", "Jack"];
    let uk_last_names = ["Davies", "Patel", "Mitchell"];
    for first_name in uk_first_names {
        start += 1;
        for last_name in uk_last_names {
            if start <= 12 {
                duration += 1;
            }
            let _ = transaction
                .query()
                .insert(&&create_person_entity(first_name, last_name));
            // Insert visits
            let end = start + duration;
            let cities: Vec<&str> = ["Zugdidi", "Tbilisi"].to_vec();
            let months: Vec<i32> = [1, 3].to_vec();
            insert_data(
                &transaction,
                first_name,
                last_name,
                cities,
                end,
                start,
                months,
            );
        }
    }

    // US

    duration = 2;
    start = 9;
    let us_first_names = ["John", "Emma", "Michael", "Sophia", "David"];
    let us_last_names = ["Smith", "Johnson", "Williams"];
    for first_name in us_first_names {
        start += 1;
        for last_name in us_last_names {
            if start <= 12 {
                duration += 1;
            }
            let _ = transaction
                .query()
                .insert(&&create_person_entity(first_name, last_name));
            // Insert visits
            let end = start + duration;
            let cities: Vec<&str> = ["Zugdidi", "Tbilisi"].to_vec();
            let months: Vec<i32> = [1, 3].to_vec();
            insert_data(
                &transaction,
                first_name,
                last_name,
                cities,
                end,
                start,
                months,
            );
        }
    }

    transaction.commit().await?;

    Ok(())
}

fn get_the_list_of_cities(_connection: Connection) -> Vec<&'static str> {
    vec!["London", "Tbilisi", "Zugdidi", "Cambridge"]
}

async fn get_most_visited_city(connection: Connection) -> typedb_client::Result<String> {
    let cities = get_the_list_of_cities(connection.clone());
    let mut best_city = "";
    let mut answer = -1;
    for city in cities {
        let num = number_of_times_visited(connection.clone(), city).await?;
        if num > answer {
            best_city = city;
            answer = num;
        }
    }
    Ok(best_city.to_string())
}

async fn number_of_times_visited(connection: Connection, city: &str) -> typedb_client::Result<i64> {
    let databases = DatabaseManager::new(connection.clone());
    let session = Session::new(databases.get(DATABASE).await?, Data).await?;
    let with_inference = Options::new().infer(true);
    let transaction = session
        .transaction_with_options(Read, with_inference)
        .await?;
    let query = format!(
        r#"match
                          $city isa city, has name "{city}";
                          $v (person: $p, destination: $city) isa visited;
                          get $p;
                          count;"#
    );
    let answer = transaction.query().match_aggregate(&query.as_str()).await?;
    Ok(answer.into_i64())
}

async fn friends_met(
    connection: Connection,
    firstname: &str,
    lastname: &str,
) -> typedb_client::Result<i64> {
    let databases = DatabaseManager::new(connection.clone());
    let session = Session::new(databases.get(DATABASE).await?, Data).await?;
    let with_inference = Options::new().infer(true);
    let transaction = session
        .transaction_with_options(Read, with_inference)
        .await?;
    let query = format!(
        r#"match
                                      $p isa person, has firstname "{firstname}", has lastname "{lastname}";
                                      $friendship(person1: $p, person2: $friend) isa friendship;
                                      $first_visit(person: $p, destination: $t) isa visited, has start_date $s1, has end_date $e1;
                                      $second_visit(person: $friend, destination: $t) isa visited, has start_date $s2, has end_date $e2;
                                      $s1 <= $e2;
                                      $s2 <= $e1;
                                    get $friend, $t; count;"#
    );

    let answer = transaction.query().match_aggregate(&query.as_str()).await?;
    Ok(answer.into_i64())
}

#[tokio::main]
async fn main() -> typedb_client::Result {
    let connection = new_core_connection().expect("creating connection");

    create_schema(connection.clone()).await?;

    create_data(connection.clone()).await?;

    loop {
        println!("Choose which query you are interested in:");
        println!("1: How many people visited city x");
        println!("2: Find a city that was visited the most");
        println!("3: Find how many times person x met a friend");
        println!("0: Exit");
        let query: i32 = read!();
        match query {
            0 => break,
            1 => {
                println!("Enter the name of the city");
                let city: String = read!();
                println!(
                    "{} people visited city {city}",
                    number_of_times_visited(connection.clone(), city.as_str()).await?
                );
            }
            2 => println!(
                "most visited city is {}",
                get_most_visited_city(connection.clone()).await?.as_str()
            ),
            3 => {
                println!("Enter the first name of the person");
                let firstname: String = read!();
                println!("Enter the last name of the person");
                let lastname: String = read!();

                println!(
                    "{firstname} {lastname} met a friend {} times",
                    friends_met(connection.clone(), firstname.as_str(), lastname.as_str()).await?
                );
            }
            _ => println!("Query entered is not 1,2,3 or 0\n"),
        };
    }

    Ok(())
}
