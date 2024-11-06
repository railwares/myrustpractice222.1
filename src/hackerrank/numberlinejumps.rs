fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    if v1 == v2 {
        if x1 == x2 {
            "YES".to_string()
        } else {
            "NO".to_string()
        }
    } else {
        let delta_x = x2 - x1;
        let delta_v = v1 - v2;
        if (delta_x % delta_v == 0) && (delta_x / delta_v >= 0) {
            "YES".to_string()
        } else {
            "NO".to_string()
        }
    }
}