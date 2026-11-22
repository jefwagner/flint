use float_next_after::NextAfter;

// Previous before value
pub fn pb(a: f64) -> f64 {
    a.next_after(f64::NEG_INFINITY)
}

// Next after value
pub fn na(b: f64) -> f64 {
    b.next_after(f64::INFINITY)
}

// Sort 2 floats
pub fn fsort2(a: f64, b: f64) -> (f64, f64) {
    if a < b { (a,b) } else { (b,a) }
}

// Find min and max for four floating point values
pub fn fminmax4(a: f64, b: f64, c: f64, d: f64) -> (f64, f64) {
    let (a, b) = fsort2(a, b);
    let (c, d) = fsort2(c, d);
    let (a, _) = fsort2(a, c);
    let (_, d) = fsort2(b, d);
    (a, d)
}
