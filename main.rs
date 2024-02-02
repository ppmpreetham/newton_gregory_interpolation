fn newton_gregory_interpolation(mut lst: Vec<i32>) -> String {
    let mut constant_terms = vec![lst[0]];

    while lst.len() > 1 {
        for i in 0..lst.len() - 1 {
            lst[i] = lst[i + 1] - lst[i];
        }
        lst.pop();
        constant_terms.push(lst[0]);
    }

    let mut formula = String::new();

    for i in 0..constant_terms.len() {
        formula += &constant_terms[i].to_string();
        formula += "*nC";
        formula += &i.to_string();
        if i != constant_terms.len() - 1 {
            formula += " + ";
        }
    }

    formula
}

// Driver code
fn main() {
    let result = newton_gregory_interpolation(vec![1, 2, 4, 8, 16, 31]);
    println!("{}", result);
}