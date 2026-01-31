//! Graph Algorithms and Maze Solver
//!
//! Implementation of Dijkstra's algorithm for pathfinding in graphs and mazes.
//! Includes a maze solver that finds the shortest path between start and end points.
//!
//! Author: Vincent Espitalier
//! Date: June 2024

#![warn(dead_code)]

use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Add;

use crate::files;

/// Trait for graph vertices that can list their neighbors and distances.
///
/// # Type Parameters
/// * `S` - Vertex type
/// * `A` - Distance type (must support ordering and addition)
pub trait Neighbors<S, A>
where
    A: PartialOrd + Add,
    S: PartialEq,
{
    /// Returns a list of neighboring vertices and their respective distances.
    ///
    /// # Arguments
    /// * `vertex` - The vertex to find neighbors for
    ///
    /// # Returns
    /// A vector of tuples containing (neighbor_vertex, distance)
    fn list_neighbors_and_distances(&self, vertex: &S) -> Vec<(S, A)>;
}

/// Finds the key with the minimum value in a HashMap.
///
/// # Type Parameters
/// * `S` - Key type (must be Eq, Hash, and Clone)
/// * `A` - Value type (must be PartialOrd, Add, TryFrom<i8>, Clone, and Debug)
///
/// # Arguments
/// * `map` - The HashMap to search
///
/// # Returns
/// A tuple containing the key with the minimum value and its value
fn find_min_key_value_pair<S, A>(map: &HashMap<S, A>) -> (S, A)
where
    S: Eq + Hash + Clone,
    A: PartialOrd + Add + TryFrom<i8> + Clone + Debug,
{
    let mut min_key: Option<&S> = None;
    let mut min_value: Option<&A> = None;

    for (key, value) in map.iter() {
        if let Some(current_min) = min_value {
            if value < current_min {
                min_key = Some(key);
                min_value = Some(value);
            }
        } else {
            min_key = Some(key);
            min_value = Some(value);
        }
    }

    (min_key.unwrap().clone(), min_value.unwrap().clone())
}

/// Solves the shortest path problem using Dijkstra's algorithm.
///
/// # Type Parameters
/// * `G` - Graph type implementing Neighbors trait
/// * `S` - Vertex type (must be Eq, Hash, and Clone)
/// * `A` - Distance type (must be PartialOrd, Add, TryFrom<i8>, Clone, Debug, and Add with Output=A)
///
/// # Arguments
/// * `graph` - The graph to search
/// * `start_vertices` - Vector of starting vertices
/// * `end_vertices` - Vector of target vertices
///
/// # Returns
/// A tuple containing:
/// 1. HashMap of all visited vertices and their distances from start
/// 2. HashMap of predecessors for path reconstruction
/// 3. Option containing the first reached end vertex (if any)
///
/// # Example
/// ```
/// let (distances, predecessors, end_vertex) = solve_dijkstra(&graph, start_vertices, end_vertices);
/// ```
///
/// # Reference
/// [Dijkstra's algorithm - Wikipedia](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm)
pub fn solve_dijkstra<G, S, A>(
    graph: &G,
    start_vertices: Vec<S>,
    end_vertices: Vec<S>,
) -> (HashMap<S, A>, HashMap<S, S>, Option<S>)
where
    G: Neighbors<S, A>,
    S: Eq + Hash + Clone,
    A: PartialOrd + Add + TryFrom<i8> + Clone + Debug + Add<Output = A>,
    <A as TryFrom<i8>>::Error: Debug,
{
    // HashMap of processed vertices with their distances from start (initially empty)
    let mut processed: HashMap<S, A> = HashMap::new();

    // HashMap of current vertices with their distances from start
    let mut current: HashMap<S, A> = HashMap::new();

    // HashMap to store predecessors for path reconstruction
    let mut predecessors: HashMap<S, S> = HashMap::new();

    // Initialize distances for start vertices to 0
    let zero_distance: A = A::try_from(0i8).expect("Missing zero distance for type A.");
    for vertex in start_vertices.iter() {
        current.insert(vertex.clone(), zero_distance.clone());
    }

    let mut end_vertex: Option<S> = None;

    // While there are vertices to process
    while !current.is_empty() {
        // Get the vertex with the smallest distance
        let (vertex, distance) = find_min_key_value_pair(&current);

        // Remove the vertex from current set
        current.remove(&vertex);

        // Add the vertex to processed set
        processed.insert(vertex.clone(), distance.clone());

        if end_vertices.contains(&vertex) {
            // Found a path to an end vertex
            end_vertex = Some(vertex);
            break;
        }

        // Iterate through neighbors
        let neighbors: Vec<(S, A)> = graph.list_neighbors_and_distances(&vertex);
        for (neighbor, neighbor_distance) in neighbors.iter() {
            // Skip if neighbor already processed
            if processed.contains_key(neighbor) {
                continue;
            }

            let new_distance = distance.clone() + neighbor_distance.clone();
            let current_distance = current.get(neighbor);

            // If neighbor is already in current set, check if we found a better path
            if let Some(current_dist) = current_distance {
                if &new_distance < current_dist {
                    // Update with better distance
                    current.insert(neighbor.clone(), new_distance);
                    predecessors.insert(neighbor.clone(), vertex.clone());
                }
            } else {
                // Add new neighbor to current set
                current.insert(neighbor.clone(), new_distance);
                predecessors.insert(neighbor.clone(), vertex.clone());
            }
        }
    }

    (processed, predecessors, end_vertex)
}

/// Represents a maze with start and end positions.
pub struct Maze {
    layout: Vec<String>,
    height: u32,
    width: u32,
    start_positions: Vec<u64>,
    end_positions: Vec<u64>,
    start_char: char,
    end_char: char,
}

impl Maze {
    /// Converts a u64 position to (height, width) coordinates.
    ///
    /// # Arguments
    /// * `pos` - The position to convert
    ///
    /// # Returns
    /// A tuple containing (height, width) coordinates
    pub fn position_to_coordinates(pos: u64) -> (u32, u32) {
        let separator_power: u32 = 32;
        let two_pow: u64 = u64::pow(2, separator_power);
        let two_pow_minus_1: u64 = two_pow - 1;
        ((pos / two_pow) as u32, (pos & two_pow_minus_1) as u32)
    }

    /// Converts (height, width) coordinates to a u64 position.
    ///
    /// # Arguments
    /// * `height` - The height coordinate
    /// * `width` - The width coordinate
    ///
    /// # Returns
    /// A u64 position value
    pub fn coordinates_to_position(height: u32, width: u32) -> u64 {
        let separator_power: u32 = 32;
        let two_pow: u64 = u64::pow(2, separator_power);
        (height as u64) * two_pow + (width as u64)
    }

    /// Finds all occurrences of a character in the maze layout.
    ///
    /// # Arguments
    /// * `layout` - The maze layout
    /// * `width` - The maze width
    /// * `height` - The maze height
    /// * `char_to_find` - The character to search for
    ///
    /// # Returns
    /// A vector of positions where the character was found
    fn find_character_in_layout(
        layout: &[String],
        width: u32,
        height: u32,
        char_to_find: char,
    ) -> Vec<u64> {
        let mut positions: Vec<u64> = Vec::new();

        for y in 0..height {
            let current_line: Vec<char> = layout[y as usize].chars().collect();
            for x in 0..width {
                let current_char = current_line[x as usize];
                if char_to_find == current_char {
                    let pos: u64 = Self::coordinates_to_position(y, x);
                    positions.push(pos);
                }
            }
        }

        positions
    }

    /// Creates a new Maze from a layout.
    ///
    /// # Arguments
    /// * `layout` - The maze layout as a vector of strings
    ///
    /// # Returns
    /// A new Maze instance
    ///
    /// # Panics
    /// Panics if the maze has no lines or if lines have inconsistent widths
    pub fn new(layout: &[String]) -> Self {
        let maze_layout = layout.to_owned();
        let maze_height = maze_layout.len() as u32;

        assert!(maze_height >= 1, "Error: Maze must have at least one line");

        let maze_width = maze_layout[0].len() as u32;
        for i in 1..maze_height {
            let current_width: u32 = maze_layout[i as usize].len() as u32;
            assert_eq!(
                current_width, maze_width,
                "Error: All lines must have the same width"
            );
        }

        let start_char = '@';
        let end_char = '$';
        let start_positions =
            Self::find_character_in_layout(&maze_layout, maze_width, maze_height, start_char);
        let end_positions =
            Self::find_character_in_layout(&maze_layout, maze_width, maze_height, end_char);

        Maze {
            layout: maze_layout,
            height: maze_height,
            width: maze_width,
            start_positions,
            end_positions,
            start_char,
            end_char,
        }
    }

    /// Gets the start positions.
    ///
    /// # Returns
    /// A vector of start positions
    pub fn start_positions(&self) -> Vec<u64> {
        self.start_positions.clone()
    }

    /// Gets the end positions.
    ///
    /// # Returns
    /// A vector of end positions
    pub fn end_positions(&self) -> Vec<u64> {
        self.end_positions.clone()
    }

    /// Gets the start character.
    ///
    /// # Returns
    /// The start character
    pub fn start_char(&self) -> char {
        self.start_char
    }

    /// Gets the end character.
    ///
    /// # Returns
    /// The end character
    pub fn end_char(&self) -> char {
        self.end_char
    }
}

impl Neighbors<u64, u64> for Maze {
    fn list_neighbors_and_distances(&self, pos: &u64) -> Vec<(u64, u64)> {
        let mut neighbors: Vec<(u64, u64)> = Vec::new();

        // Possible neighbors are the 4 directions (up, down, left, right) at distance 1
        let (height, width): (u32, u32) = Self::position_to_coordinates(*pos);
        let mut possible_neighbors: Vec<(u64, u64)> = Vec::new();

        if height > 0 {
            possible_neighbors.push((Self::coordinates_to_position(height - 1, width), 1));
        }
        possible_neighbors.push((Self::coordinates_to_position(height + 1, width), 1));
        if width > 0 {
            possible_neighbors.push((Self::coordinates_to_position(height, width - 1), 1));
        }
        possible_neighbors.push((Self::coordinates_to_position(height, width + 1), 1));

        // Can pass through spaces or start/end characters (not walls)
        let passable_chars: Vec<char> = vec![' ', self.start_char, self.end_char];

        for (neighbor, distance) in possible_neighbors {
            let (neighbor_height, neighbor_width): (u32, u32) =
                Self::position_to_coordinates(neighbor);

            // Skip if out of bounds
            if neighbor_height >= self.height {
                continue;
            }
            if neighbor_width >= self.width {
                continue;
            }

            let current_cell: char = self.layout[neighbor_height as usize]
                .chars()
                .collect::<Vec<_>>()[neighbor_width as usize];
            if !passable_chars.contains(&current_cell) {
                continue;
            }
            neighbors.push((neighbor, distance));
        }

        neighbors
    }
}

/// Solves a maze and saves the solution to a file.
///
/// # Arguments
/// * `maze_file` - Path to the maze input file
/// * `solution_file` - Path to save the solution
///
/// # Example
/// ```
/// solve_maze("maze.txt".to_string(), "solution.txt".to_string());
/// ```
pub fn solve_maze(maze_file: String, solution_file: String) {
    let maze_layout: Vec<String> = files::read_text_file_lines(&maze_file, None);
    let maze: Maze = Maze::new(&maze_layout);

    let start_positions: Vec<u64> = maze.start_positions();
    let end_positions: Vec<u64> = maze.end_positions();

    println!("\nStart position(s):");
    for pos in &start_positions {
        let (height, width) = Maze::position_to_coordinates(*pos);
        println!("(x,y) = ({},{})", width, height);
    }

    println!("\nEnd position(s):");
    for pos in &end_positions {
        let (height, width) = Maze::position_to_coordinates(*pos);
        println!("(x,y) = ({},{})", width, height);
    }

    let (distances, predecessors, end_vertex) =
        solve_dijkstra(&maze, start_positions, end_positions);

    if let Some(final_vertex) = end_vertex {
        let final_distance = distances[&final_vertex];
        let (height, width) = Maze::position_to_coordinates(final_vertex);
        println!(
            "End vertex ({}, {}) has a distance of: {}",
            width, height, final_distance
        );

        // Store all visited vertices
        let mut visited_vertices: Vec<(u32, u32)> = Vec::new();
        let start_positions: Vec<u64> = maze.start_positions();
        let end_positions: Vec<u64> = maze.end_positions();

        for pos in distances.keys() {
            if start_positions.contains(pos) || end_positions.contains(pos) {
                continue;
            }
            let (height, width) = Maze::position_to_coordinates(*pos);
            visited_vertices.push((width, height));
        }

        // Build the solution path
        let mut solution_path: Vec<(u32, u32)> = Vec::new();
        let mut current_vertex = final_vertex;
        let start_positions: Vec<u64> = maze.start_positions();

        while predecessors.contains_key(&current_vertex) {
            current_vertex = predecessors[&current_vertex];
            if start_positions.contains(&current_vertex) {
                continue;
            }
            let (height, width) = Maze::position_to_coordinates(current_vertex);
            solution_path.push((width, height));
        }

        // Save solution to file
        let mut solution_layout = maze_layout.clone();

        let visited_char = 'o';
        for (width, height) in visited_vertices {
            let mut current_line: Vec<char> = solution_layout[height as usize].chars().collect();
            current_line[width as usize] = visited_char;
            let line_string = current_line.iter().collect::<String>();
            solution_layout[height as usize] = line_string;
        }

        let path_char = 'x';
        for (width, height) in solution_path {
            let mut current_line: Vec<char> = solution_layout[height as usize].chars().collect();
            current_line[width as usize] = path_char;
            let line_string = current_line.iter().collect::<String>();
            solution_layout[height as usize] = line_string;
        }

        files::write_text_file_lines(&solution_file, &solution_layout);

        // Create colored output for display
        let visited_char_colored = "\x1b[90mo\x1b[0m";
        let path_char_colored = "\x1b[93mx\x1b[0m";
        let start_char_colored = "\x1b[94m@\x1b[0m";
        let end_char_colored = "\x1b[92m$\x1b[0m";

        let mut colored_solution: Vec<String> = Vec::new();
        for line in solution_layout {
            let colored_line = line.replace(visited_char, visited_char_colored);
            let colored_line = colored_line.replace(path_char, path_char_colored);
            let colored_line = colored_line.replace(maze.start_char(), start_char_colored);
            let colored_line = colored_line.replace(maze.end_char(), end_char_colored);
            colored_solution.push(colored_line);
        }

        // Display colored solution
        println!("Solution (via Dijkstra's algorithm)");
        println!("{}", colored_solution.join("\n"));
    }
}
