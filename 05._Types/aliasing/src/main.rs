type NanoSecond = u64;
type Inch = u64;
type U64 = u64;

fn main() {
    let nanoseconds: NanoSecond = 5 as u64;
    let inches: Inch = 2 as u64;

    println!(
        "{} nanoseconds + {} inches = {} uint？",
        nanoseconds,
        inches,
        nanoseconds + inches
    );
}
