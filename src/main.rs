fn lagrange_interpolation(points: &[(i32, i32)], x: i32) -> f64 {
    let mut result = 0.0;

    for (i, &(xi, yi)) in points.iter().enumerate() {
        let mut term = yi as f64;

        for (j, &(xj, _)) in points.iter().enumerate() {
            if i != j {
                term *= (x - xj) as f64 / (xi - xj) as f64;
            }
        }
        result += term;
    }

    result
}

fn main() {
    let points = [(0, 4), (-2, 1), (2, 3)];

    let x = -2; // Example input for which to interpolate
    let interpolated_value = lagrange_interpolation(&points, x);

    println!("The interpolated value at x = {} is: {}", x, interpolated_value);
}

