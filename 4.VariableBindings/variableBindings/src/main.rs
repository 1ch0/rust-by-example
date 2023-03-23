fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let uint = ();

    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the uint value: {:?}", uint);

    let _unused_vairable = 3u32;

    let noisy_unused_vairable = 2u32;
}
