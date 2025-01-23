/// Constant array representing the input feature values.
pub const X: [f64; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];
/// Constant array representing the target values.
pub const Y: [f64; 5] = [2.0, 4.0, 5.0, 4.0, 5.0];

/// A structure representing a simple linear regression model.
pub struct LinearRegression {
    pub slope: f64,
    pub intercept: f64,
}

impl LinearRegression {
    /// Creates a new instance of `LinearRegression` with default slope and intercept.
    pub fn new() -> Self {
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
    pub fn fit(&mut self, x: &Vec<f64>, y: &Vec<f64>) {
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
    pub fn predict(&self, x: f64) -> f64 {
        self.slope * x + self.intercept
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
