// graph.rs
use petgraph::graph::{Graph, NodeIndex}; //importing module from petgraph and other libraries 
use std::collections::HashMap;
use std::collections::HashSet;
use itertools::Itertools;
use petgraph::visit::Visitable;
use petgraph::visit::VisitMap;

// Creating a struct to represent a Hero, # of comics they appear in, and centrality
pub struct Hero { 
    pub name: String,
    pub comics: usize,
    pub centrality: f64,
}
// Implementing for new Heros
impl Hero {
    // New Hero constructor setup 
    fn new(name: String) -> Hero {
        Hero {
            name,
            comics: 0, //start the comic count as 0
            centrality: 0.0, //start the centrality as 0
        }
    }
}

//defining a struct for the graph 
pub struct MarvelGraph {
    // Graph where nodes are heros and edges represent interactions in comics
    pub graph: Graph<Hero, ()>,
    // HashMap to keep track of NodeIndex for each hero by their name
    pub hero_indices: HashMap<String, NodeIndex>,
}
// Implementation block for MarvelGraph containing all available comics
impl MarvelGraph {
    // Creating a function to create a MarvelGraph from the map of comics data
    pub fn from_comics_data(data: HashMap<String, Vec<String>>) -> MarvelGraph {
        let mut graph = MarvelGraph {
            graph: Graph::new(),
            hero_indices: HashMap::new(),
        };
        //iterating through each comic and heros that met within it 
        for (_comic, heroes) in data {
             
            // Track heroes already counted in this comic to avoid repeats
            let mut counted_heroes = HashSet::new();
            for hero in heroes.iter() {  // Adding heroes to the graph and counting their comic appearances
    
                let hero_index = *graph.hero_indices.entry(hero.clone()).or_insert_with(|| {
                    graph.graph.add_node(Hero::new(hero.clone()))
                });
    
                // augmenting the comics count for the hero, if its not already counted for the comic
                if let Some(hero_node) = graph.graph.node_weight_mut(hero_index) {
                    if counted_heroes.insert(hero.clone()) {
                        hero_node.comics += 1;
                    }
                }
            }
            // Add edges between all pairs of heroes in the same comic, if not already 
            for hero_pairs in heroes.iter().combinations(2) {
                let hero1_index = graph.hero_indices[hero_pairs[0]];
                let hero2_index = graph.hero_indices[hero_pairs[1]];
    
                // Check if the edge already exists before adding
                if !graph.graph.contains_edge(hero1_index, hero2_index) {
                    graph.graph.add_edge(hero1_index, hero2_index, ());
                }
            }
        }
        graph
    }

    pub fn average_connections(&self) -> f64 { //function to caluclate average number of hero connections
        let total_connections: usize = self.graph.node_indices()
            .map(|node_idx| self.graph.neighbors(node_idx).count())
            .sum();

        let num_heroes = self.graph.node_count();
        
        // Return the average if there are heroes, otherwise return 0
        if num_heroes > 0 {
            total_connections as f64 / num_heroes as f64
        } else {
            0.0
        }
    }
    //function to compute centrality for each hero
    pub fn compute_centrality(&self) -> HashMap<String, f64> {
        let mut centrality = HashMap::new();
        let n = self.graph.node_count() as f64;
        
        //actually calculating centrality
        for node_idx in self.graph.node_indices() {
            let hero = &self.graph[node_idx];
            let degree = self.graph.neighbors(node_idx).count() as f64;
            // Normalizing the centrality
            let normalized_centrality = degree / (n - 1.0);
            centrality.insert(hero.name.clone(), normalized_centrality);
        }
        centrality
    }
    pub fn most_connected_hero(&self) -> Hero {  // Function to find the hero with the most comic appearances

        let mut max_comics = 0;
        let mut most_connected_hero = Hero::new("".to_string());

        for hero_index in self.graph.node_indices() {
            let hero = &self.graph[hero_index];
            if hero.comics > max_comics {
                max_comics = hero.comics;
                most_connected_hero = Hero::new(hero.name.clone());
                most_connected_hero.comics = hero.comics;
            }
        }

        most_connected_hero
    }

    //Creating the function to compute heroes reachable within 6 degrees
    pub fn compute_reachable_heroes(&self) -> HashMap<NodeIndex, HashSet<NodeIndex>> {
        let mut reachable_heroes: HashMap<NodeIndex, HashSet<NodeIndex>> = HashMap::new();

        // Iterating through each hero and compute their reachable heroes in 6 degrees
        for node_idx in self.graph.node_indices() {
            let mut visited = self.graph.visit_map(); // Keeping track of visited nodes
            let mut queue = std::collections::VecDeque::new();
            queue.push_back((node_idx, 0)); // Starting with the current node and degree 0

            // issuing set of reachable nodes
            let mut reachable: HashSet<NodeIndex> = HashSet::new();

            while let Some((current_idx, degree)) = queue.pop_front() {
                if degree > 6 { break; } //Stopping if degree exceeds 6 (6 degrees of separation)

                for neighbor_idx in self.graph.neighbors(current_idx) {
                    // Visiting unvisited neighbors and add them to reachable set
                    if !visited.is_visited(&neighbor_idx) {
                        visited.visit(neighbor_idx);
                        reachable.insert(neighbor_idx);
                        queue.push_back((neighbor_idx, degree + 1)); // Incrementing degree for next level
                    }
                }
            }
            //Storing the set of reachable heroes for each hero
            reachable_heroes.insert(node_idx, reachable);
        }

        reachable_heroes
    }
    
    #[allow(dead_code)]
    // Function to get the index of a hero by their name (this is only used in the test)
    pub fn get_hero_index(&self, hero_name: &str) -> Option<NodeIndex> {
        self.hero_indices.get(hero_name).cloned() // returning the node index if the hero exists in the graph already
    }
}
