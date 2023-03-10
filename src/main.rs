use rand::SeedableRng;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;

fn main() {
    println!("Hello, world!");
}

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

fn there_is_no_passage_here(index: usize, neighbor: usize, passages: &mut Vec<(usize,usize)>) -> bool {
    for pass in passages {
        if (index == pass.0 && neighbor == pass.1) || (index == pass.1 && neighbor == pass.0) {
            return false;
        }
    }
    return true;
}

fn print_maze(width: usize, height: usize, passages: &mut Vec<(usize,usize)>) {
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

fn find_next_passage(
    index: usize, 
    width: usize, 
    height: usize, 
    visited: &mut Vec<bool>, 
    passages: &mut Vec<(usize,usize)>,
    rng: &mut SmallRng
) {

    visited[index] = true;
    // print_maze(width, height, passages);

    let neighbors = find_neighbors(index, width, height);
    let mut potential_passages: Vec<usize> = neighbors.into_iter()
        .flatten()
        .filter(|&n| visited[n] == false)
        .filter(|&n| there_is_no_passage_here(index, n, passages))
        .collect();
    potential_passages.shuffle(rng);

    for pass in potential_passages {
        if visited[pass] == false {
            passages.push((index,pass));
            find_next_passage(pass, width, height, visited, passages, rng);
        }
    }
}

#[cfg(test)]

#[test]
pub fn wont_you_be_my_neight() {
    let width = 4;
    let height = 4;

    let index = 0;
    let neighbors = find_neighbors(index, width, height);
    assert_eq!(
        neighbors,
        [Some(4), None, None, Some(1)]
    );

    let index = 5;
    let neighbors = find_neighbors(index, width, height);
    assert_eq!(
        neighbors,
        [Some(9), Some(1), Some(4), Some(6)]
    );

    let index = 13;
    let neighbors = find_neighbors(index, width, height);
    assert_eq!(
        neighbors,
        [None, Some(9), Some(12), Some(14)]
    );
}

#[test]
pub fn simple() {
    let index = 0;
    let width = 16;
    let height = 16;
    let num_cells = width * height;
    let mut visited = vec![false; num_cells];
    let mut passages = Vec::<(usize,usize)>::new();
    let mut rng = SmallRng::seed_from_u64(11);
    find_next_passage(index, width, height, &mut visited, &mut passages, &mut rng);

    print_maze(width, height, &mut passages);
}
