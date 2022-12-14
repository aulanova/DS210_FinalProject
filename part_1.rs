use rand::Rng;
use std::fs::File;
use std::io::BufRead;
use std::collections::VecDeque;
use std::vec;
use assert_approx_eq::assert_approx_eq;


// Final Project Part 1: Finding the "average" or "usual" distance between a pair of vertices
// The goal is to calculate the average distance between a pair of vertices in the graph generated by the Facebook social circle data set obtained
// from the Stanford Large Network Dataset Collection.

// I want to investigate the idea of "6 degrees of separation" and see if various people on the app can be connected
// to one another through no more than 6 other people on the app.

// I will run a breadth-first-search algorithm from a random start node to a random end node; subsequently, I will find the distance between those 2 vertices.
// Since there are over 4000 nodes in this text file, I will randomly run the algorithm 1000 x and take the average of those 1000 distances.

// The code:

// creating a struct Graph
struct Graph {
    adj_list: Vec<Vec<usize>>,
}

// creating a struct Queue for my breadth-first-search algorithm
struct Queue<T> {
    components: VecDeque<T>,
}
impl<T> Queue<T> {
    // generating a new Queue
    fn generate_new() -> Queue<T> {
        Queue {
            components: VecDeque::new(),
        }
    }
    // insterting into back of the queue
    fn add_to_back_of_queue(&mut self, v: T) {
        self.components.push_back(v)
    }

    // returning and removing item from the front of the queue
    fn remove_from_front_queue(&mut self) -> T {
        self.components.pop_front().expect("Cannot dequeue from empty queue.")
    }

    // checking if the queue is empty
    fn is_queue_empty(&self) -> bool {
        self.components.len() == 0
    }
}

// creating an implementation on the struct Graph
impl Graph {
    // creating an adjacency list
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
            } else {
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

    // running a breadth first search algorithm where the function returns the path from the origin node to the final node
    fn breadth_first_search(&self) -> Option<Vec<Option<usize>>> {
        let mut rng = rand::thread_rng();
        let origin_node: usize = rng.gen_range(0..self.adj_list.len());
        let final_node: usize = rng.gen_range(0..self.adj_list.len());
        let mut queue = Queue::generate_new();
        queue.add_to_back_of_queue(origin_node);
        let mut visited_vertices = vec![false; self.adj_list.len()];
        visited_vertices[0] = true;

        //"order" is used to record the order in which we visit the vertices
        let mut order: Vec<Option<usize>> = vec![None; self.adj_list.len()];
        'outer: while !queue.is_queue_empty() {
            let current_node = queue.remove_from_front_queue();
            for i in self.adj_list[current_node as usize].iter() {
                if *i == final_node {
                    order[*i as usize] = Some(current_node);
                    break 'outer;
                }
                if !visited_vertices[*i as usize] {
                    queue.add_to_back_of_queue(*i);
                    visited_vertices[*i as usize] = true;
                    order[*i as usize] = Some(current_node);
                }
            }
        }
        let mut trail = Vec::new();
        let mut end_node_check = Some(final_node);
        while end_node_check!= None {
            trail.push(end_node_check);
            end_node_check = order[end_node_check.unwrap_or(0) as usize];
        }
        trail.reverse();
        return match trail[0] {
            Some(x) if x == origin_node => Some(trail),
            _ => None,
        };
    }

    fn average_distance_between_vertices(&self) -> f32 {
        let data_file = Graph::new("facebook_combined.txt");
        let mut vec_of_distances: Vec<i32> = vec![];
        //let data_file = Graph::new("facebook_combined.txt");
        // In order to obtain 1000 paths from a random start-node to a random end-node, 
        // I will loop through the bfs algorithm 1000x and collect
        // the resulting vector path which will give me the distance that I will then average out. 
        for _i in 1..=1000 {
            let resulting_vector_path = Graph::breadth_first_search(&data_file);
            //println!("the initial vector path is {:?}", resulting_vector_path);
            for _j in resulting_vector_path {
                //println!("the vector of the path is {:?}", _j);
                let mut distance: i32 = 0;
                for _i in _j {
                    // println!(" the value in the vector is {:?}", _i);
                    distance += 1;
                }
                //println!("the distance is {}", distance);
                vec_of_distances.push(distance);
            }
        }
        //println!("vector of the distances {:?}", vec_of_distances);
        let sum: i32 = vec_of_distances.iter().sum();
        let average_distance: f32 = (sum as f32) / (vec_of_distances.len() as f32);
        println!("The average distance between a pair of vertices in this graph is {}.", average_distance);
        println!();
        return average_distance;
        
    }


}

// note: use cargo run release for faster computations
pub fn main() {
    println!();
    println!( "The following computation is for the graph generated from the Faceboook social circle data set obtained from the Stanford Large Network Dataset Collection.");
    println!();
    // running a test case
    bfs_test()   
}
// However many times I ran it, the average distance between a pair of vertices in the Facebook social circle graph was never above 6. It was most of the times in the
// range of 5 - 6, mainly hovering around 5.5.


// The following below is a test case for my breadth-first search algorithm using the test_file.txt. The test case allows for some room in error (around 0.5) because
// the algorithm randomly picks out certain start and end nodes, so a certain range should be expected. An exact distance will not be computed since the algorithm also
// returns float numbers and not integers.

fn bfs_test() {
    // the expected breadth first search distance between a pair of vertices calculated by using a Python algorithm
    let expected_bfs = 5.5;
    // below computation is calculated using my function
    let data_file_test = Graph::new("test_file.txt");
    let calculated_bfs: f32 = data_file_test.average_distance_between_vertices();
    //assert_approx_eq!(expected_bfs,calculated_bfs);
    assert_approx_eq!(expected_bfs, calculated_bfs, 0.5f32);
}