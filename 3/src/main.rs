use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let mut topo_lines = Vec::<Vec<bool>>::new();
    for l in contents.split("\n") {
        if l == "" {
            continue
        }
        topo_lines.push(l.chars().map(|c| c == '#').collect())
    }

    let t =
        tree_count(&topo_lines, 1, 1) *
        tree_count(&topo_lines, 3, 1) *
        tree_count(&topo_lines, 5, 1) *
        tree_count(&topo_lines, 7, 1) *
        tree_count(&topo_lines, 1, 2);

    println!("{}", t);
}
fn tree_count(trees: &Vec<Vec<bool>>, right: usize, down: usize) -> usize{
    let height = trees.len();
    let width = trees[0].len();


    let mut x = 0;
    let mut y = 0;
    let mut tree_count = 0;
    while y < height {
        if trees[y][x] {
            tree_count += 1;
        }
        y += down;
        x += right;
        if x >= width {
            x = x % width;
        }
    }
    return tree_count
}
