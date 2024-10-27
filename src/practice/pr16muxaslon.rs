#[test]
fn main() {
    let mut solutions = 0;
    let mut found_first_solution = false;

    for m in 1..=9 {  // `m` cannot be 0 for a four-digit number `muxa`
        for u in 0..=9 {
            if u == m { continue; }
            for x in 0..=9 {
                if x == m || x == u { continue; }
                for a in 1..=9 {  // `a` should not be 0 in multiplication
                    if a == m || a == u || a == x { continue; }
                    for s in 1..=9 {  // `s` cannot be 0 for a four-digit number `slon`
                        if s == m || s == u || s == x || s == a { continue; }
                        for l in 0..=9 {
                            if l == m || l == u || l == x || l == a || l == s { continue; }
                            for o in 0..=9 {
                                if o == m || o == u || o == x || o == a || o == s || o == l { continue; }
                                for n in 0..=9 {
                                    if n == m || n == u || n == x || n == a || n == s || n == l || n == o { continue; }

                                    // Form the numbers `muxa` and `slon`
                                    let muxa = 1000 * m + 100 * u + 10 * x + a;
                                    let slon = 1000 * s + 100 * l + 10 * o + n;

                                    // Check if `muxa * a == slon`
                                    if muxa * a == slon {
                                        solutions += 1;
                                        if !found_first_solution {
                                            found_first_solution = true;
                                            println!("First solution:");
                                            println!("  {}", muxa);
                                            println!("x {}", a);
                                            println!("  ------");
                                            println!("  {}", slon);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Total solutions: {}", solutions);
}
