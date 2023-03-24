fn main() {
    let a_binding;

    {
        let x = 2;
        a_binding = x * x;
    }

    println!("a bindings: {}", a_binding);

    let another_binding;
    // println!("another bindings: {}", another_binding);

    another_binding = 1;
    println!("another bindings: {}", another_binding);
}
