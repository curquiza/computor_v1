mod test;

fn first_guess(r: f64) -> f64 {
    let rslt: f64 = (r / 2.0).floor();
    if rslt == 0.0 {
        1.0
    } else {
        rslt
    }
}

/*
** Babylonian method to calculate sqrt
*/
fn babylonian_method(r: f64) -> f64 {
    let epsilon: f64 = 1e-10; // precision
    let mut x: f64 = first_guess(r);
    let mut prev: f64 = 0.0;
    while (x - prev).abs() > epsilon {
        prev = x;
        x = (prev + (r / prev)) / 2.0;
    }
    return x;
}

pub fn sqrt(r: f64) -> Option<f64> {
    if r < 0.0 {
        None
    } else if r == 0.0 {
        Some(0.0) 
    } else {
        Some(babylonian_method(r))
    }
}
