//read.rs
use std::collections::HashMap; //importing library modules and crates
use csv;
use std::error::Error;

// Creating the function to read edges from a CSV file and return the HashMap 
pub fn read_edges_from_file(file_path: &str) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
    // Creating a new CSV reader for the file at the given path
    let mut rdr = csv::ReaderBuilder::new()
        //Showing whether the CSV file has a header row 
        // '?' is used to return the error if any
        .has_headers(false) 
        .from_path(file_path)?;

    // Creating an empty HashMap to store comics and heroes relations
    let mut comics_to_heroes = HashMap::new();

    // Iterate over each line in the CSV file
    for result in rdr.records() {
        let record = result?;
        if record.len() != 2 {  // Skipping the record if it does not contain exactly two inputs (hero and comic)
            eprintln!("Warning: Line does not contain exactly two entries, skipping");
            continue;
        }
        // Get the hero and comic names from the dataset
        let hero = &record[0];
        let comic = &record[1];
        // Insert the hero into the HashMap under their corresponding comic, but if the comic key doesn't exist, it's put into an empty Vec
        comics_to_heroes.entry(comic.to_string()).or_insert_with(Vec::new).push(hero.to_string());
    }
    //returning the hashmap filled with the hero and comic titles 
    Ok(comics_to_heroes)
}