use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> (i32, usize, usize) {
    let mut min_sum = i32::MAX;
    let mut min_index = 0;

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_index = i;
        }
    }

    (min_sum, min_index, min_index + 1)
}

fn print_result(data: &[i32], min_sum: i32, index1: usize, index2: usize) {
    print!("indexes: ");
    for i in 0..data.len() {
        print!("{:>2}. ", i);
    }
    println!();

    print!("data:    ");
    for i in 0..data.len() {
        print!("{:>2}, ", data[i]);
    }
    println!();

    print!("indexes: ");
    for i in 0..data.len() {
        if i == index1 {
            print!("\\__ ");
        } else if i == index2 {
            print!("__/");
        } else {
            print!("    ");
        }
    }
    println!();

    println!("min adjacent sum={}+{}={} at indexes:{},{}\n", data[index1], data[index2], min_sum, index1, index2);
}
#[test]
fn test() {
    for _ in 0..5 {
        let data = gen_random_vector(20);
        let (min_sum, index1, index2) = min_adjacent_sum(&data);
        print_result(&data, min_sum, index1, index2);
    }
}