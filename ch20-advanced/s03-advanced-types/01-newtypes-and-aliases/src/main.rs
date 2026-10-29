// Newtypes: Millimeters and Meters are different types, so the compiler stops them being mixed up.
struct Millimeters(u32);
struct Meters(u32);

// A type alias gives an existing type another name. Kilometers is still i32, so no protection is added.
type Kilometers = i32;

// A type alias for a long type, used in many places.
type Thunk = Box<dyn Fn() + Send + 'static>;

fn to_millimeters(m: Meters) -> Millimeters {
    Millimeters(m.0 * 1000)
}

fn run_thunk(f: Thunk) {
    f();
}

fn main() {
    // Kilometers and i32 are the same type, so they can be added without any conversion.
    let km: Kilometers = 5;
    let total = km + 10;
    println!("kilometers: {total}");

    let thunk: Thunk = Box::new(|| println!("thunk runs"));
    run_thunk(thunk);

    // to_millimeters only accepts Meters. Passing a plain u32 would not compile.
    let mm = to_millimeters(Meters(2));
    println!("millimeters: {}", mm.0);

    // Try: call to_millimeters(2) with a plain number. What does the compiler say?
}
