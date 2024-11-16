use geometric_construction::Truth;

fn main() {
    let input = vec![
        vec![7, 0, 5, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 1, 0, 6, 0, 0],
        vec![8, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 6, 0, 0, 0, 0, 8, 0, 5],
        vec![0, 0, 0, 3, 7, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 9, 0],
        vec![4, 2, 0, 0, 0, 0, 1, 0, 0],
        vec![0, 0, 0, 5, 0, 8, 0, 0, 0], 
        vec![0, 0, 0, 9, 0, 0, 0, 0, 0],
    ];
    
    let (truth, is_solvable) = Truth::is(input);
    println!("Truth: {:?}", truth);
    println!("Is solvable: {}", is_solvable);
}