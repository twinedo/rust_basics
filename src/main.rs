fn main() {
   let exec1 = mul(2, 3);
   println!("Exercise 1: mul: {exec1}");

   let exec2 = full_name("Twin", "Edo");
   println!("Exercise 2: full_name: {exec2}");

   let exec3 = c_to_f(30.0);
   println!("Exercise 3: c_to_f: {exec3}");

   let exec4 = is_even(42);
   println!("Exercise 4: is_even: {exec4}");

//    let exec5 = choose_name(true, "twin", "edo");
//    println!("Exercise 5: choose_name: {exec5}");

   let exec6 = abs(-5);
   println!("Exercise 6: abs: {exec6}");

   let exec7 = min2(100, 3);
   println!("Exercise 7: min2: {exec7}");

   let exec8 = clamp(5, 1, 10);
   println!("Exercise 8: clamp: {exec8}");

   let exec9 = weird_math(30);
   println!("Exercise 9: weird_math: {exec9}");

   let exec10 = add_ok(30, 20);
   println!("Exercise 10: add_ok: {exec10}");
}

// Exercise 1 — Multiply (return value)
fn mul(a: i32, b: i32) -> i32 {
 a * b
}

// Exercise 2 — Full name (params, return String)
fn full_name(first: &str, last: &str) -> String {
    format!("{first} {last}")
}

// Exercise 3 — Temperature converter (f64)
fn c_to_f(c: f64) -> f64 {
    c * 1.8 + 32.0
}


// Exercise 4 — Even check (bool)
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

// // Exercise 5 (Replacement) — Choose a name (if as expression returning &str)
// fn choose_name(use_first: bool, first: &str, second: &str) -> &str {
//     if use_first { first } else { second }
// }

// Exercise 6 — Absolute value (early return allowed)
fn abs(n: i32) -> i32 {
    if n < 0 {-n} else {n}
}

// Exercise 7 — Min of two numbers (if expression)
fn min2(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

// Exercise 8 — Clamp to range (multiple params)
fn clamp(n: i32, low: i32, high: i32) -> i32 {
    if n < low { low } else if n > high { high } else { n }
}

// Exercise 9 — Block expression practice (return from a block) 
fn weird_math(x: i32) -> i32 {
    let y = (x + 2) * 3;
    y - 4
}

// Exercise 10 — The semicolon trap (fix the broken function)
fn add_ok(a: i32, b: i32) -> i32 {
    a + b
}


