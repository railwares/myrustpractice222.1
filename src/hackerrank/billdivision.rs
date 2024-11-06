fn bonAppetit(bill: &[i32], k: i32, b: i32) {
    let mut list = bill.to_vec();
    let price = list.remove(k as usize);
    let total_price = list.iter().sum::<i32>();
    let shared_price = total_price / 2;
    if shared_price == b {
        println!("Bon Appetit");
    } else {
        let refund = b - shared_price;
        println!("{}", refund);
    }
}