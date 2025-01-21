/// Constant array representing the input feature values.
const X: [f64; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];
/// Constant array representing the target values.
const Y: [f64; 5] = [2.0, 4.0, 5.0, 4.0, 5.0];

/// A structure representing a simple linear regression model.
struct LinearRegression {
    slope: f64,
    intercept: f64,
}

impl LinearRegression {
    /// Creates a new instance of `LinearRegression` with default slope and intercept.
    fn new() -> Self {
        LinearRegression {
            slope: 0.0,
            intercept: 0.0,
        }
    }

    /// Fits the linear regression model to the provided data.
    ///
    /// # Arguments
    ///
    /// * `x` - A vector of feature values.
    /// * `y` - A vector of target values.
    fn fit(&mut self, x: &Vec<f64>, y: &Vec<f64>) {
        let x_mean = vec_mean(x);
        let y_mean = vec_mean(y);
        let l: f64 = x.iter().zip(y.iter())
            .map(|(x, y)| (x - x_mean) * (y - y_mean))
            .sum();
        let m: f64 = x.iter().map(|x| (x - x_mean)
            .powi(2))
            .sum();
        self.slope = l / m;
        self.intercept = y_mean - self.slope * x_mean;
    }

    /// Predicts the target value for a given feature value using the fitted model.
    ///
    /// # Arguments
    ///
    /// * `x` - A feature value for which to predict the target value.
    ///
    /// # Returns
    ///
    /// The predicted target value.
    fn predict(&self, x: f64) -> f64 {
        self.slope * x + self.intercept
    }

    /// Calculates the mean squared error of the model predictions.
    ///
    /// # Arguments
    ///
    /// * `x` - A vector of feature values.
    /// * `y` - A vector of actual target values.
    ///
    /// # Returns
    ///
    /// The mean squared error of the predictions.
    fn calculate_error(&self, x: &Vec<f64>, y: &Vec<f64>) -> f64 {
        let n = x.len() as f64;
        let error: f64 = x.iter().zip(y.iter())
            .map(|(x, y)| {
                let prediction = self.predict(*x);
                (y - prediction).powi(2)
            })
            .sum();
        error / n
    }
}

/// Computes the mean of a vector of floating-point numbers.
///
/// # Arguments
///
/// * `v` - A vector of floating-point numbers.
///
/// # Returns
///
/// The mean of the numbers in the vector.
fn vec_mean(v: &Vec<f64>) -> f64 {
    let sum: f64 = v.iter().sum();
    sum / v.len() as f64
}

fn main() {
    let x = X.to_vec();
    let y = Y.to_vec();

    let mut model = LinearRegression::new();
    model.fit(&x, &y);

    println!("y = {}x + {}", model.slope, model.intercept);
    
    let test_x = 3.0;
    let prediction = model.predict(test_x);
    println!("Prediction for x={}: {}", test_x, prediction);

    let error = model.calculate_error(&x, &y);
    println!("Mean Squared Error: {}", error);
}
