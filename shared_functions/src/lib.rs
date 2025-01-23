/// Calculates the mean squared error between predicted and actual values.
///
/// # Arguments
///
/// * `predictions` - A vector of predicted values
/// * `actual` - A vector of actual target values
///
/// # Returns
///
/// The mean squared error between the predictions and actual values
pub fn mean_squared_error(predictions: &Vec<f64>, actual: &Vec<f64>) -> f64 {
    let n = predictions.len() as f64;
    let error: f64 = predictions.iter().zip(actual.iter())
        .map(|(pred, act)| {
            (act - pred).powi(2)
        })
        .sum();
    error / n
}

/// Calculates the root mean squared error between predicted and actual values.
///
/// # Arguments
///
/// * `predictions` - A vector of predicted values
/// * `actual` - A vector of actual target values
///
/// # Returns
///
/// The root mean squared error between the predictions and actual values
pub fn root_mean_squared_error(predictions: &Vec<f64>, actual: &Vec<f64>) -> f64 {
    let mse = mean_squared_error(predictions, actual);
    mse.sqrt()
}

/// Calculates the mean absolute error between predicted and actual values.
///
/// # Arguments
///
/// * `predictions` - A vector of predicted values
/// * `actual` - A vector of actual target values
///
/// # Returns
///
/// The mean absolute error between the predictions and actual values
pub fn mean_absolute_error(predictions: &Vec<f64>, actual: &Vec<f64>) -> f64 {
    let n = predictions.len() as f64;
    let error: f64 = predictions.iter().zip(actual.iter())
        .map(|(pred, act)| {
            (act - pred).abs()
        })
        .sum();
    error / n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean_squared_error() {
        let predictions = vec![2.0, 4.0, 6.0];
        let actual = vec![1.8, 3.7, 5.9];

        let mse = mean_squared_error(&predictions, &actual);

        // Expected MSE calculation:
        // ((2.0-1.8)^2 + (4.0-3.7)^2 + (6.0-5.9)^2) / 3
        // (0.2^2 + 0.3^2 + 0.1^2) / 3
        // (0.04 + 0.09 + 0.01) / 3
        // 0.14 / 3
        // ≈ 0.0467

        assert!((mse - 0.0467).abs() < 0.0001);
    }

    #[test]
    fn test_root_mean_squared_error() {
        let predictions = vec![2.0, 4.0, 6.0];
        let actual = vec![1.8, 3.7, 5.9];
        let rmse = root_mean_squared_error(&predictions, &actual);
        assert!((rmse - 0.216).abs() < 0.001);
    }

    #[test]
    fn test_mean_absolute_error() {
        let predictions = vec![2.0, 4.0, 6.0];
        let actual = vec![1.8, 3.7, 5.9];
        let mae = mean_absolute_error(&predictions, &actual);
        assert!((mae - 0.2).abs() < 0.001);
    }
}
