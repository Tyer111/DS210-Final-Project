//main.rs
mod graph; //importing all my modules and the struct/functions within them 
mod read;
use graph::MarvelGraph;
use read::read_edges_from_file;

fn main() {
    // Path to the CSV file containing the heros, comics called edges.csv
    let file_path = "/Users/tyerobison/Documents/project1/src/edges.csv"; 
    // Reading the file and creating a map of comics to heroes
    let comics_to_heroes = read_edges_from_file(file_path).expect("Failed to read file"); 
    //creating the graph from the module struct Marvel Graph
    let marvel_graph = MarvelGraph::from_comics_data(comics_to_heroes); 
    
    // Compute centrality scores for all heroes from graph
    let centrality = marvel_graph.compute_centrality(); 

    //finding the hero with the most connections
    let most_connected_hero = marvel_graph.most_connected_hero(); 

    //creating the list of the top 5 heros with the most comics they are in based on the centrality scores from above
    let mut top_5: Vec<(&str, f64)> = centrality.iter().map(|(hero, &c)| (hero.as_str(), c)).collect();
    top_5.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
    top_5.truncate(5);
    println!("Top 5 heroes in terms of the percentage of comics they have appeared in:");
    for (hero, centrality) in top_5 {
        println!("{}: {:.2}%", hero, centrality * 100.0);
    }
    println!("The hero who has the most connections is: {}", most_connected_hero.name);
    
    //collecting a list of all the heros in the graph and counting the total number of pairs of heros 
    let heroes: Vec<_> = marvel_graph.graph.node_indices().collect(); 
    let total_pairs = heroes.len() * (heroes.len() - 1) / 2;

    // Computing the number of heroes each hero can reach within 6 degrees of separation using the precomputed data
    let reachable_heroes = marvel_graph.compute_reachable_heroes();
    let connected_pairs = reachable_heroes.values().map(|set| set.len()).sum::<usize>() / 2;
    let percentage = connected_pairs as f64 / total_pairs as f64 * 100.0; //Calculating the percentage of hero pairs that are connected within 6 degrees
    println!("Percentage of heroes connected within 6 degrees: {:.2}%", percentage);

     // Computing the average number of connections per character form the graph
    let average_connections = marvel_graph.average_connections();
    println!("Average connections per character: {:.2}", average_connections);
}


#[cfg(test)]
mod tests { //setting up the tests for different parts of program
    use std::collections::HashMap;
    use super::*;
    
    #[test]
    fn test_graph_creation() { //testing creating the marvel graph 
        let mut sample_data = HashMap::new();
        //setting up some sample data to test comics v heros and creating a garph
        sample_data.insert("Comic1".to_string(), vec!["Hero1".to_string(), "Hero2".to_string()]);
        sample_data.insert("Comic2".to_string(), vec!["Hero2".to_string(), "Hero3".to_string()]);
        let graph = MarvelGraph::from_comics_data(sample_data);
    
        // Check the number of nodes and edges ensuring that the edges match the pairs of heros in the comics
        assert_eq!(graph.graph.node_count(), 3, "Incorrect number of nodes");
        assert_eq!(graph.graph.edge_count(), 2, "Incorrect number of edges");
    }
    
    #[test]
    fn test_most_connected_hero() { //testing how to find the most connected hero in the graph 
    let mut sample_data = HashMap::new();
    
    // Creating some sample data with Hero1 being the most connected
    sample_data.insert("Comic1".to_string(), vec!["Hero1".to_string(), "Hero2".to_string()]);
    sample_data.insert("Comic2".to_string(), vec!["Hero2".to_string(), "Hero3".to_string()]);
    sample_data.insert("Comic3".to_string(), vec!["Hero1".to_string(), "Hero3".to_string()]);
    sample_data.insert("Comic4".to_string(), vec!["Hero1".to_string(), "Hero4".to_string()]); // Adjusted for Hero1 to appear more times

    // creating the graph to find the most connected hero (expectign hero1)
    let graph = MarvelGraph::from_comics_data(sample_data);
    let most_connected = graph.most_connected_hero();

    // Check if the most connected hero is Hero1 and if the comic count is right
    assert_eq!(most_connected.name, "Hero1", "Incorrect most connected hero");
    assert_eq!(most_connected.comics, 3, "Incorrect number of comics for the most connected hero");
}
    
    #[test]
    fn test_degrees_of_separation_with_precomputed_data() { //testing if the 6 degrees of separation method works with two specific heros
        let mut sample_data = HashMap::new();
        // Sample data setup to ensure Hero1 and Hero5 are connected within 6 degrees
        sample_data.insert("Comic1".to_string(), vec!["Hero1".to_string(), "Hero2".to_string()]);
        sample_data.insert("Comic2".to_string(), vec!["Hero2".to_string(), "Hero3".to_string()]);
        sample_data.insert("Comic3".to_string(), vec!["Hero3".to_string(), "Hero4".to_string()]);
        sample_data.insert("Comic4".to_string(), vec!["Hero4".to_string(), "Hero5".to_string()]);

        //creating the graph to collect the degrees of separation and computing reachable heros within 6 degrees
        let graph = MarvelGraph::from_comics_data(sample_data);
        let reachable_heroes = graph.compute_reachable_heroes();
    
        // Retrieving indices for the two heros from the graph
        let hero1_index = graph.get_hero_index("Hero1").expect("Hero1 not found");
        let hero5_index = graph.get_hero_index("Hero5").expect("Hero5 not found");
        
        // Checking if Hero5 is within the reachable set of Hero1 within 6 degrees
        let is_connected_within_six_degrees = reachable_heroes.get(&hero1_index)
            .map_or(false, |reachable| reachable.contains(&hero5_index));
        
        // Ensure that Hero1 and Hero5 are connected within 6 degrees
        assert!(is_connected_within_six_degrees, "Hero1 and Hero5 are not connected within 6 degrees");
    }
}
