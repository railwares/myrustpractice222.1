use rand::Rng;

fn count_permutation(shipments: &Vec<u32>) -> isize {
    let n = shipments.len();
    let total_cargo: u32 = shipments.iter().sum();
    if total_cargo % n as u32 != 0 {
        println!("-1");
        return -1;
    }

    let average = total_cargo as i32 / n as i32;
    let mut balance = 0;
    let mut moves = 0;

    println!("Shipments: {:?}", shipments);
    println!("Average cargo per ship: {}", average);
    println!("\nCargo redistribution steps:");

    for (i, &cargo) in shipments.iter().enumerate() {
        let diff = (cargo as i32) - average;

        if diff > 0 {
            println!("Ship {}: Remove {} units", i + 1, diff);
        } else if diff < 0 {
            println!("Ship {}: Add {} units", i + 1, -diff);
        } else {
            println!("Ship {}: Balanced (0 units)", i + 1);
        }
        balance += diff;
        moves += balance.abs();
    }
    let result = moves / 2;
    println!("\nTotal moves needed to balance cargo: {}", result);
    result as isize
}

fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let mut shipments = Vec::new();
    for _ in 0..n {
        let cargo = rng.gen_range(1..=9);
        shipments.push(cargo);
    }
    shipments
}
#[test]
fn main() {
    // Generate a random shipment
    let shipments = gen_shipments(5);
    let moves = count_permutation(&shipments);
    }