use std::collections::BTreeMap;
use crate::term::Term;


pub fn get_coefficient_in_terms(degree: &i32, terms: &BTreeMap<i32, Term>) -> f64 {

    // Return the coefficient of the degree passed in arguments in terms

    let coefficient = terms.get(degree);

    if coefficient.is_none() {
        return 0.0;
    }
    coefficient.unwrap().coefficient

}
