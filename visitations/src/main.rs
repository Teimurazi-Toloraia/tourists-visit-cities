use typedb_client::{
    Connection, DatabaseManager, Session,
    SessionType::{Data, Schema},
    TransactionType::{Read, Write}
  };
  use std::fs::{self};
  use text_io::read;
  
  
  
  
  const DATABASE: &str = "tourists visiting cities database";
  
  fn new_core_connection() -> typedb_client::Result<Connection> {
    Connection::new_plaintext("localhost:1729")
  }
  
  fn get_schema() -> String {
     fs::read_to_string("./src/schema.tql").expect("Failed to read schema file")
  }
  
  
  fn create_country_entity(name: &str) -> String {
    let result = format!("insert $x isa country, has name \"{}\";", name);
      result
  }
  
  fn create_city_entity(name: &str) -> String {
    let result = format!("insert $x isa city, has name \"{}\";", name);
      result
  }
  
  fn create_person_entity(firstname: &str, lastname: &str) -> String {
    let result = format!("insert $x isa person, has firstname \"{}\", has lastname \"{}\";", firstname, lastname);
      result
  }
  
  async fn create_schema(connection: Connection) -> std::io::Result<()> {
      let schema = get_schema();
      let databases = DatabaseManager::new(connection.clone());
      if databases.contains(DATABASE).await.unwrap()==false {
          databases.create(DATABASE).await.unwrap();
          // define schema
          let session = Session::new(databases.get(DATABASE).await.unwrap(), Schema).await.unwrap();
          let transaction = session.transaction(Write).await.unwrap();
          transaction.query().define(schema.as_str()).await.unwrap();
          transaction.commit().await.unwrap();
          session.force_close().unwrap();
      }
      else {
          println!("\nschema is already defined\n");
      }
      Ok(())
  }
  
  
  
  async fn create_data(connection: Connection) -> std::io::Result<()> {
      let databases = DatabaseManager::new(connection.clone());
      // insert data
      let session = Session::new(databases.get(DATABASE).await.unwrap(), Data).await.unwrap();
      let transaction = session.transaction(Write).await.unwrap();
      let cities = ["Tbilisi", "London", "Cambridge", "Zugdidi"];
      for city in cities {
        let _ = transaction.query().insert(&create_city_entity(city));
      }
      let countries = ["Georgia", "United Kingdom", "United States of America"];
      for country in countries {
        let _ = transaction.query().insert(&create_country_entity(country));
      }
      // Georgians
  
      let geo_first_names= ["Teimurazi", "Natalia", "Luka", "Irakli", "Giorgi", "Robiko"];
      let geo_last_names = ["Toloraia", "Skhulukhia", "Tsintsadze"];
      let mut duration = 0;
      let mut start = 9;
      for first_name in geo_first_names {
        start += 1;
        for last_name in geo_last_names {
            if start <= 12 {
              duration += 1;
            }
            let _ = transaction.query().insert(&&create_person_entity(first_name, last_name));
            // Insert visits
            let end = start + duration;
            let _ = transaction.query().insert(format!(r#"match $person isa person, has firstname "{first_name}", has lastname "{last_name}";
            $city1 isa city, has name "Cambridge";
            insert (person: $person, destination: $city1) isa visited, has start_date 2023-01-{start}, has end_date 2023-01-{end};"#).as_str());
            
  
            let _ = transaction.query().insert(format!(r#"match $person isa person, has firstname "{first_name}", has lastname "{last_name}";
            $city2 isa city, has name "London";
            insert (person: $person, destination: $city2) isa visited, has start_date 2023-03-{start}, has end_date 2023-03-{end};"#).as_str());
        }
      }
  
  
      // UK
  
      duration = 3;
      start = 9;
      let uk_first_names= ["James", "Olivia", "William", "Amelia", "Jack"];
      let uk_last_names = ["Davies", "Patel", "Mitchell"];
      for first_name in uk_first_names {
        start += 1;
        for last_name in uk_last_names {
            if start <= 12 {
              duration += 1;
            }
            let _ = transaction.query().insert(&&create_person_entity(first_name, last_name));
            // Insert visits
            let end = start + duration;
            let _ = transaction.query().insert(format!(r#"match $person isa person, has firstname "{first_name}", has lastname "{last_name}";
            $city1 isa city, has name "Tbilisi";
            insert (person: $person, destination: $city1) isa visited, has start_date 2023-01-{start}, has end_date 2023-01-{end};"#).as_str());
            
  
            let _ = transaction.query().insert(format!(r#"match $person isa person, has firstname "{first_name}", has lastname "{last_name}";
            $city2 isa city, has name "Zugdidi";
            insert (person: $person, destination: $city2) isa visited, has start_date 2023-03-{start}, has end_date 2023-03-{end};"#).as_str());
        }
      }
  
  
      // US
  
      duration = 2;
      start = 9;
      let us_first_names= ["John", "Emma", "Michael", "Sophia", "David"];
      let us_last_names = ["Smith", "Johnson", "Williams"];
      for first_name in us_first_names {
        start += 1;
        for last_name in us_last_names {
            if start <= 12 {
              duration += 1;
            }
            let _ = transaction.query().insert(&&create_person_entity(first_name, last_name));
            // Insert visits
            let end = start + duration;
            let _ = transaction.query().insert(format!(r#"match $person isa person, has firstname "{first_name}", has lastname "{last_name}";
            $city1 isa city, has name "Tbilisi";
            insert (person: $person, destination: $city1) isa visited, has start_date 2023-01-{start}, has end_date 2023-01-{end};"#).as_str());
            
  
            let _ = transaction.query().insert(format!(r#"match $person isa person, has firstname "{first_name}", has lastname "{last_name}";
            $city2 isa city, has name "Zugdidi";
            insert (person: $person, destination: $city2) isa visited, has start_date 2023-03-{start}, has end_date 2023-03-{end};"#).as_str());
        }
      }
  
  
      let _ = transaction.query().insert(r#"match
                    $p isa person;
                    {$p has lastname "Toloraia";} or {$p has lastname "Skhulukhia";} or {$p has lastname "Tsintsadze";};
                    $c isa country, has name "Georgia";
                    insert (person: $p, homeland: $c) isa nationality;"#);
  
      let _ = transaction.query().insert(r#"match
                    $p isa person;
                    {$p has lastname "Smith";} or {$p has lastname "Johnson";} or {$p has lastname "Williams";};
                    $c isa country, has name "United States of America";
                    insert (person: $p, homeland: $c) isa nationality;"#);
  
      let _ = transaction.query().insert(r#"match
                    $p isa person;
                    {$p has lastname "Davies";} or {$p has lastname "Patel";} or {$p has lastname "Mitchell";};
                    $c isa country, has name "United Kingdom";
                    insert (person: $p, homeland: $c) isa nationality;"#);
  
  
      let _ = transaction.query().insert(format!(r#"match
                    (person: $p1, homeland: $c) isa nationality;
                    (person: $p2, homeland: $c) isa nationality;
                    insert (person1: $p1, person2: $p2) isa friendship;"#).as_str());
      
  
  
  
      transaction.commit().await.unwrap();
    
      Ok(())
  }
  
  fn get_the_list_of_cities(_connection: Connection) -> Vec<&'static str> {
    let strings: Vec<&str> = vec!["London", "Tbilisi", "Zugdidi", "Cambridge"];
    strings
    // TODO: do this with query
  }
  
  async fn get_most_visited_city(connection: Connection) -> String {
    let cities = get_the_list_of_cities(connection.clone());
    let mut best_city = "";
    let mut answer = -1;
    for city in cities {
        let num = number_of_times_visited(connection.clone(), city).await;
        if num > answer {
            best_city = city;
            answer = num;
        }
    }
    best_city.to_string()
  }
  
  async fn number_of_times_visited(connection: Connection, city: &str) -> i64 {
  
      let databases = DatabaseManager::new(connection.clone());
      let session = Session::new(databases.get(DATABASE).await.unwrap(), Data).await.unwrap();
      let transaction = session.transaction(Read).await.unwrap();
      let query = format!(r#"match
                          $city isa city, has name "{city}";
                          $v (person: $p, destination: $city) isa visited;
                          get $p;
                          count;"#);
      let answer = transaction.query().match_aggregate(&query.as_str()).await.unwrap();
      answer.into_i64()
  }
  
  
  async fn friends_met(connection: Connection, firstname: &str, lastname: &str) -> i64 {
      let databases = DatabaseManager::new(connection.clone());
      let session = Session::new(databases.get(DATABASE).await.unwrap(), Data).await.unwrap();
      let transaction = session.transaction(Read).await.unwrap();
      let query = format!(r#"match
                                      $p isa person, has firstname "{firstname}", has lastname "{lastname}";
                                      $friendship(person1: $p, person2: $friend) isa friendship;
                                      $first_visit(person: $p, destination: $t) isa visited, has start_date $s1, has end_date $e1;
                                      $second_visit(person: $friend, destination: $t) isa visited, has start_date $s2, has end_date $e2;
                                      $s1 <= $e2;
                                      $s2 <= $e1;
                                    get $friend, $t; count;"#);
  
      let answer = transaction.query().match_aggregate(&query.as_str()).await.unwrap();
      answer.into_i64()
  }
  
  
  
  #[tokio::main]
  async fn main()->std::io::Result<()>{
    let connection= new_core_connection().expect("creating connection");
  
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
              let city:String = read!();
              println!("{} people visited city {city}", number_of_times_visited(connection.clone(), city.as_str()).await);
          }
          2 => println!("most visited city is {}", get_most_visited_city(connection.clone()).await.as_str()),
          3 => {
              println!("Enter the first name of the person");
              let firstname:String = read!();
              println!("Enter the last name of the person");
              let lastname:String = read!();
  
              println!("{firstname} {lastname} met a friend {} times", 
              friends_met(connection.clone(), firstname.as_str(), lastname.as_str()).await);
          }
          _ => println!("Query entered is not 1,2,3 or 0\n")
      };
    }
  
    Ok(())
  
  }