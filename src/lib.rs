use std::char::MAX;

use heapless::Vec;
use rand::SeedableRng;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;

fn find_neighbors(index: usize, width: usize, height: usize) -> [Option<usize>;4] {
    let num_cells = width * height;

    let up = if index < num_cells - width {
        Some(index + width)
    } else {
        None
    };

    let down = if index > width - 1 {
        Some(index - width)
    } else {
        None
    };

    let left = if index % width != 0 {
        Some(index - 1)
    } else {
        None
    };

    let right = if (index + 1) % width != 0 {
        Some(index + 1)
    } else {
        None
    };

    return [up, down, left, right];
}

pub fn there_is_no_passage_here<const N: usize>(
    index: usize, 
    neighbor: usize, 
    passages: &Vec<(usize,usize), N>
) -> bool {
    for pass in passages {
        if (index == pass.0 && neighbor == pass.1) || (index == pass.1 && neighbor == pass.0) {
            return false;
        }
    }
    return true;
}

pub fn find_passages<const M: usize, const N: usize>(
    index: usize, 
    width: usize, 
    height: usize, 
    visited: &mut Vec<bool,M>, 
    passages: &mut Vec<(usize,usize),N>,
    rng: &mut SmallRng
) {

    visited[index] = true;

    let neighbors = find_neighbors(index, width, height);
    let mut potential_passages: Vec<usize,4> = neighbors.into_iter()
        .flatten() // Option implements IntoIter
        .filter(|&n| visited[n] == false)
        .filter(|&n| there_is_no_passage_here(index, n, passages))
        .collect();
    potential_passages.shuffle(rng);

    for pass in potential_passages {
        if visited[pass] == false {
            passages.push((index,pass)).unwrap();
            find_passages(pass, width, height, visited, passages, rng);
        }
    }
}

pub fn find_walls<const M: usize, const N: usize, const P: usize>(
    width: usize, 
    height: usize, 
    passages: &mut Vec<(usize,usize),M>, 
    horizontal_walls: &mut Vec<u16,N>,
    vertical_walls: &mut Vec<u16,P>
) {
    // Find horizontal walls
    for h in 0..height+1 {
        // First and last rows contain all walls
        if h == 0 || h == height {
            horizontal_walls[h] = (0..width).fold(0b0 as u16, |acc, val| acc + (0b1 << val));
        } else {
            // For all other rows, check for walls below each cell
            for w in 0..width {
                let ind = h*width + w;
                if there_is_no_passage_here(ind,ind-width,passages) { 
                    horizontal_walls[h] = horizontal_walls[h] + (0b1 << w);
                }
            }
        }
    }

    // Find vertical walls
    for w in 0..width+1 {
        // First and last columns contain all walls
        if w == 0 || w == width {
            vertical_walls[w] = (0..height).fold(0b0 as u16, |acc, val| acc + (0b1 << val));
        } else {
            // For all other columns, check for walls to the left of each cell
            for h in 0..height {
                let ind = h*width + w;
                if there_is_no_passage_here(ind,ind-1,passages) { 
                    vertical_walls[w] = vertical_walls[w] + (0b1 << h);
                }
            }
        }
    }
}

pub fn find_path<const M: usize, const N: usize>(
    start: usize,
    end: usize,
    width: usize, 
    height: usize, 
    passages: &mut Vec<(usize,usize),N>,
    visited: &mut Vec<bool,M>,
    stack: &mut Vec<usize,N>,
    paths: &mut Vec<usize,N>,
    pruned_path: &mut Vec<usize,N>,
    rng: &mut SmallRng
) {

    visited[start] = true;
    stack.push(start).unwrap();
    let mut still_looking = true;
    let mut checkpoint = start;

    while let Some(node) = stack.pop() {
        if still_looking {
            visited[node] = true;
            let neighbors = find_neighbors(node, width, height);
            let mut potential_paths: Vec<usize,4> = neighbors.into_iter()
                .flatten() // Option implements IntoIter
                .filter(|&n| visited[n] == false)
                .filter(|&n| there_is_no_passage_here(node, n, passages) == false)
                .collect();
            potential_paths.shuffle(rng);

            if node == end {
                paths.push(node).unwrap();
                still_looking = false;
            } else if potential_paths.len() > 1 {
                checkpoint = node;
                paths.push(node).unwrap();
            } else if potential_paths.len() == 0 {
                while let Some(p) = paths.pop() {
                    if p == checkpoint {
                        paths.push(checkpoint).unwrap();
                        break;
                    }
                }
            } else {
                paths.push(node).unwrap();
            }

            for pass in potential_paths {
                stack.push(pass).unwrap();
            }
        }
    }

    let mut last_node = paths.pop().unwrap();
    pruned_path.insert(0, last_node).unwrap();
    while let Some(node) = paths.pop() {
        if there_is_no_passage_here(last_node, node, passages) == false {
            pruned_path.insert(0, node).unwrap();
            last_node = node;
        }
    }
}

pub fn next_node_in_path<const M: usize, const N: usize>(
    start: usize,
    end: usize,
    width: usize, 
    height: usize, 
    passages: &mut Vec<(usize,usize),N>,
    visited: &mut Vec<bool,M>,
    stack: &mut Vec<usize,N>,
    paths: &mut Vec<usize,N>,
    pruned_path: &mut Vec<usize,N>,
    rng: &mut SmallRng
) -> usize {
    find_path(start, end, width, height, passages, visited, stack, paths, pruned_path, rng);
    if pruned_path.len() > 1 {
        pruned_path.remove(1)
    } else {
        start
    }
}

fn print_maze<const N: usize>(width: usize, height: usize, passages: &mut Vec<(usize,usize),N>) {
    for _ in 0..width {
        print!(" _");
    }
    print!("\n");
    for ht in (0..height).rev() {
        print!("|");
        for wd in 0..width {
            let ind = ht*width + wd;
            if ht > 0 {
                if there_is_no_passage_here(ind,ind-width,passages) { 
                    print!("_") 
                } else {
                    print!(" ")
                }
            } else {
                print!("_");
            }
            if there_is_no_passage_here(ind,ind+1,passages) { 
                print!("|") 
            } else {
                print!(" ")
            }

        }
        print!("\n");
    }
    print!("\n\n");
}

#[cfg(test)]

// #[test]
// pub fn wont_you_be_my_neighbor() {
//     let width = 4;
//     let height = 4;

//     let index = 0;
//     let neighbors = find_neighbors(index, width, height);
//     assert_eq!(
//         neighbors,
//         [Some(4), None, None, Some(1)]
//     );

//     let index = 5;
//     let neighbors = find_neighbors(index, width, height);
//     assert_eq!(
//         neighbors,
//         [Some(9), Some(1), Some(4), Some(6)]
//     );

//     let index = 13;
//     let neighbors = find_neighbors(index, width, height);
//     assert_eq!(
//         neighbors,
//         [None, Some(9), Some(12), Some(14)]
//     );
// }

// #[test]
// pub fn simple() {
//     let index = 0;
//     const WIDTH: usize = 13; // number of horizontal cells in maze
//     const HEIGHT: usize = 13; // number of vertical cells in maze
//     const NUM_CELLS: usize = WIDTH * HEIGHT;
//     const MAX_PASSAGES: usize = NUM_CELLS; // memory to reserve for maze
//     let mut visited = Vec::<bool,NUM_CELLS>::new();
//     visited.extend_from_slice(&[false;NUM_CELLS]).unwrap();
//     let mut passages = Vec::<(usize,usize),MAX_PASSAGES>::new();
//     let mut rng = SmallRng::seed_from_u64(11);
//     find_passages(index, WIDTH, HEIGHT, &mut visited, &mut passages, &mut rng);

//     let mut horizontal_walls = Vec::<u16,{HEIGHT+1}>::new();
//     horizontal_walls.extend_from_slice(&[0b0000000000000000;{HEIGHT+1}]).unwrap();
//     let mut vertical_walls = Vec::<u16,{WIDTH+1}>::new();
//     vertical_walls.extend_from_slice(&[0b0000000000000000;{WIDTH+1}]).unwrap();

//     find_walls(WIDTH, HEIGHT, &mut passages, &mut horizontal_walls, &mut vertical_walls);

//     // for hw in horizontal_walls { println!("{:#018b}", hw) }
//     // println!("");
//     // for vw in vertical_walls { println!("{:#018b}", vw) }
// }

#[test]
pub fn paths() {
    let index = 0;
    const WIDTH: usize = 13; // number of horizontal cells in maze
    const HEIGHT: usize = 13; // number of vertical cells in maze
    const NUM_CELLS: usize = WIDTH * HEIGHT;
    const MAX_PASSAGES: usize = NUM_CELLS; // memory to reserve for maze
    let mut visited = Vec::<bool,NUM_CELLS>::new();
    visited.extend_from_slice(&[false;NUM_CELLS]).unwrap();
    let mut passages = Vec::<(usize,usize),MAX_PASSAGES>::new();
    let mut rng = SmallRng::seed_from_u64(42);
    find_passages(index, WIDTH, HEIGHT, &mut visited, &mut passages, &mut rng);

    let mut paths = Vec::<usize, MAX_PASSAGES>::new();
    let mut pruned_path = Vec::<usize, MAX_PASSAGES>::new();
    let mut stack = Vec::<usize, MAX_PASSAGES>::new();
    visited.clear();
    visited.extend_from_slice(&[false;NUM_CELLS]).unwrap();
    let end = NUM_CELLS-1;
    find_path(index, end, WIDTH, HEIGHT, &mut passages, &mut visited, &mut stack, &mut paths, &mut pruned_path, &mut rng);
    
    print_maze(WIDTH, HEIGHT, &mut passages);
    print!("{:?}\n", pruned_path);
    visited.clear();
    visited.extend_from_slice(&[false;NUM_CELLS]).unwrap();
    paths.clear();
    pruned_path.clear();
    stack.clear();
    print!("{}\n", next_node_in_path(index, end, WIDTH, HEIGHT, &mut passages, &mut visited, &mut stack, &mut paths, &mut pruned_path, &mut rng));

}
