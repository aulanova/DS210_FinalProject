use rand::Rng;
use std::fs::File;
use std::io::BufRead;
use assert_approx_eq::assert_approx_eq;

// Final Project Part 2:
// The goal is to find the degree distribution in my graph that is devised from the Facebook social circle data set.
// I need to compute the degree of a random selection of 1000 nodes in the graph and then visualize the distribution of these nodes to
// see if there is a particular distribution they follow. From previous reading, I expect to see a power law distribution. I will not be running any statistical analyses. 
// Since that data set I have is quite large (over 4000 nodes), I will choose a random subset of 1000 vertices and
// look at their degrees.

struct Graph {
    adj_list: Vec<Vec<usize>>,
   
}

impl Graph {
    fn new(file_path: &str) -> Self {
        let file = File::open(file_path).expect("Unable to open file");
        let buf_reader = std::io::BufReader::new(file).lines();
        let mut n: usize = 0;
        let mut adj_list: Vec<Vec<usize>> = vec![vec![]; n];
        for (_i, line) in buf_reader.enumerate() {
            let line_str = line.expect("Error reading");
            if _i == 0 {
                n = line_str.parse::<usize>().unwrap();
                adj_list = vec![vec![];n];
            }
            else {
                let v: Vec<&str> = line_str.trim().split_whitespace().collect();
                let x = v[0].parse::<usize>().unwrap();
                let y = v[1].parse::<usize>().unwrap();
                adj_list[x].push(y);
            }
        }
        return Graph {
            adj_list,
        };
    }

    // doing a simple calculation by calculating the degree of a node or vertex
    // the function will return the degree of the node
    fn calculate_degree_of_node(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let random_node: usize = rng.gen_range(0..self.adj_list.len());
        let mut counter = 0;
        for _ in self.adj_list[random_node as usize].iter() {
            counter +=1
    }
        // println!("The degree of the node is {}", counter);
        return counter;
} 
   
    // in this function, I was curious to investigate the degree of nodes at distance 2; I will once again average out the degree
    // node at distance 2 since there won't be 1 singular value since a particular node can have multiple nodes at distance 2 that
    // they are connected to
    fn calculate_degreee_of_node_at_distance_2(&self) -> f32 {
        let mut rng = rand::thread_rng();
        let mut vector_of_distances:Vec<usize> = Vec::new();
        let random_node: usize = rng.gen_range(0..self.adj_list.len());
        for neighbors_distance_1 in self.adj_list[random_node].iter() {
            for neighbors_distance_2 in self.adj_list[*neighbors_distance_1].iter() {
                vector_of_distances.push(self.adj_list[*neighbors_distance_2].len());
            }
        } 
        let avg_distance: f32 = (vector_of_distances.iter().sum::<usize>() as f32)/ (vector_of_distances.len() as f32);
        return avg_distance;

    }

}

// in this function, I will output a vector containing the degrees of the 1000 nodes and the average degree of a node
pub fn main() {
    let data_file = Graph::new("facebook_combined.txt");
    let mut vector_of_nodes:Vec<i32> = vec![];
    for _ in 1..= 1000 {
        let degree_of_node:i32 = Graph::calculate_degree_of_node(&data_file); 
        // println!("{}", degree_of_node);
        vector_of_nodes.push(degree_of_node);
    }
    println!();
    println!("Below is the vector containing the degree of 1000 nodes that were randomly chosen from my graph. In the report you can see the visualized distribution.");
    println!();
    println!("{:?}", vector_of_nodes);
    let sum: i32 =  vector_of_nodes.iter().sum();
    println!();
    let average_degree_of_vertex: f32 = sum as f32/ vector_of_nodes.len() as f32;
    println!("The average degree of a node in this Facebook social circle graph is {}", average_degree_of_vertex);


    // below I will print the vector containing the degree of nodes at distance 2
    let mut vector_of_nodes_distance_2:Vec<f32> = vec![];
    for _ in 1..= 1000 {
        let degree_of_node_distance_2:f32 = Graph::calculate_degreee_of_node_at_distance_2(&data_file); 
        vector_of_nodes_distance_2.push(degree_of_node_distance_2);
        //println!("{}", degree_of_node_distance_2);
    }
    println!();
    println!("Below is the vector containing the degree of 1000 nodes at distance 2 that were randomly chosen from my graph. In the report you can see the visualized distribution.");
    println!();
    println!("{:?}", vector_of_nodes_distance_2);
    calculate_degree_node_test();
    //println!("{:?}", vector_of_nodes_distance_2);
    // let sum: f32 =  vector_of_nodes_distance_2.iter().sum();
    //let average_degree_of_vertex_at_distance_2: f32 = sum as f32/ vector_of_nodes_distance_2.len() as f32;
    //println!("The average degree of a node at distance 2 in this Facebook social circle graph is {}", average_degree_of_vertex_at_distance_2)

    
}


// test case for my calculate degree node function
fn calculate_degree_node_test() {
    // the expected degree of node for my test file calculated by hand
    let expected_degree = 3;
    // below computation is calculated using my function
    let data_file_test = Graph::new("test_file.txt");
    let calculated_degree: i32 = data_file_test.calculate_degree_of_node();
    assert_approx_eq!(expected_degree, calculated_degree, 2i32);
}