fn square(number: f64) -> f64 {
    number * number
}

fn root(number: f64) -> f64 {
    if number <= 1.0 {
        return number;
    }

    let mut start = 0.0;
    let mut end = number;
    let precision = 0.0001;

    while end - start > precision {
        let mid = (start + end) / 2.0;

        if square(mid) > number {
            end = mid;
        } else {
            start = mid;
        }
    }

    ((start + end) / 2.0) as i64 as f64
}

fn delta(a: f64, b: f64, c: f64) -> f64 {
    square(b) - 4.0 * a * c
}

fn x0(a: f64, b: f64) -> f64 {
    -b / 2.0 * a
}

fn x1(a: f64, b: f64, delta: f64) -> f64 {
    (-b + root(delta)) / 2.0 * a
}

fn x2(a: f64, b: f64, delta: f64) -> f64 {
    (-b - root(delta)) / 2.0 * a
}

pub fn quadratic(a: f64, b: f64, c: f64) {
    let delta = delta(a, b, c);

    if delta < 0.0 {
        return;
    }

    if delta == 0.0 {
        x0(a, b);
        return;
    }

    x1(a, b, delta);
    x2(a, b, delta);
}

pub fn horner(degree: i64, x: f64, coefficients: &[f64]) -> f64 {
    let mut result: f64 = 0.0;

    for i in 0..=degree {
        if let Some(coefficient) = coefficients.get(i as usize) {
            result = result * x + coefficient;
        }
    }

    result
}
