use std::time::Instant;
use shared_functions::mean_squared_error;
use shared_functions::root_mean_squared_error;

mod lr_classic;

fn main() {
    // Linear Regression example
    let now = Instant::now();

    let x = lr_classic::X.to_vec();
    let y = lr_classic::Y.to_vec();

    let mut model = lr_classic::LinearRegression::new();
    model.fit(&x, &y);

    println!("Linear Regression Model: y = {}x + {}", model.slope, model.intercept);

    let test_x = 3.0;
    let prediction = model.predict(test_x);
    let mse_error = mean_squared_error(&x, &y);
    let rmse_error = root_mean_squared_error(&x, &y);
    let elapsed = now.elapsed();

    println!("Prediction for x={}: {}", test_x, prediction);
    println!("Mean Squared Error: {}", mse_error);
    println!("Root Mean Squared Error: {}", rmse_error);
    println!("Time elapsed: {:?}", elapsed);
}
