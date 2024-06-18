//! Provides variable definition data structures for optimisation.
use serde::Deserialize;
use std::path::Path;

/// The definition of a variable to be optimised.
///
/// The coefficients represent the multiplying factors in the objective function to maximise or
/// minimise, i.e. the Cs in:
///
/// f = c1*x1 + c2*x2 + ...
///
/// with x1, x2... taking values between min and max.
#[derive(PartialEq, Debug, Deserialize)]
pub struct VariableDefinition {
    /// The variable's name
    pub name: String,
    /// The variable's minimum value
    pub min: f64,
    /// The variable's maximum value
    pub max: f64,
    /// The coefficient of the variable in the objective
    pub coefficient: f64,
}

impl VariableDefinition {
    /// Read variable definitions from the specified path.
    ///
    /// Returns a variable definitions or an error.
    ///
    /// # Arguments:
    ///
    /// * `path`: The path to the variable definitions CSV file
    pub fn vec_from_csv(path: &Path) -> Result<Vec<VariableDefinition>, csv::Error> {
        let mut reader = csv::Reader::from_path(path)?;
        let mut vars = Vec::new();
        for result in reader.deserialize() {
            let var: VariableDefinition = result?;
            vars.push(var);
        }

        Ok(vars)
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_common::{get_example_path, get_example_variable_definitions};
    use super::VariableDefinition;
    use std::path::PathBuf;

    /// Get the path to the example variables.csv file.
    fn get_variables_file_path() -> PathBuf {
        get_example_path().join("variables.csv")
    }

    #[test]
    fn test_read_variables() {
        let definitions = VariableDefinition::vec_from_csv(&get_variables_file_path()).unwrap();
        assert_eq!(definitions, &get_example_variable_definitions());
    }
}
