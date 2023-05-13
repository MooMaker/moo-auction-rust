pub fn compare_solution(solution_json: String) {
    //TODO get best solution from storage
    let mut best_solution = "{}";
    if compute_value(solution_json) > compute_value(best_solution) {
        best_solution = solution_json;
    }
}

fn compute_value(solution_json: String) -> f64 {
    return 0.0;
}
