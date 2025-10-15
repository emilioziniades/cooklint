#[derive(Debug)]
pub struct LintResult {
    pub parse_failures: Vec<String>,
    pub ingredients_no_aisle: Vec<String>,
    pub duplicate_ingredients: Vec<(String, String)>,
}

impl LintResult {
    pub fn summarize(&self) {
        if self.parse_failures.is_empty() {
            println!("PASS: All recipes parsed successfully");
        } else {
            println!("FAIL: Parse failures");

            for parse_failure in &self.parse_failures {
                println!("\t{parse_failure}");
            }
        }

        if self.ingredients_no_aisle.is_empty() {
            println!("PASS: All ingredients have an aisle");
        } else {
            println!("FAIL: Ingredients missing an aisle");

            for parse_failure in &self.ingredients_no_aisle {
                println!("\t{parse_failure}");
            }
        }

        if self.duplicate_ingredients.is_empty() {
            println!("PASS: No duplicate ingredients found");
        } else {
            println!("FAIL: Duplicate ingredients found");

            for (i0, i1) in &self.duplicate_ingredients {
                println!("\t{i0} {i1}");
            }
        }
    }

    #[must_use]
    pub const fn is_success(&self) -> bool {
        self.duplicate_ingredients.is_empty()
            && self.ingredients_no_aisle.is_empty()
            && self.parse_failures.is_empty()
    }
}
