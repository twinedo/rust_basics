
// O1 — Predict: Copy or Move
fn copy_or_move() {
    let a = 10; // => Copy
    let b = a; // => Copy

    let s1 = String::from("hi"); // => Move
    let s2 = s1; // => Move

    println!("{a} {b} {s2}");
}

// O2 — Fix “use of moved value” (minimal fix)
fn fix_use_of_moved_value() {
    let s = String::from("hello");
    let t = s.clone();
    println!("{t}");
    println!("{s}"); // ❌ fix it (minimal)
}

// O3 — Function takes ownership
fn consume(s: &String) {
    println!("{s}");
}
fn takes_ownership() {
    let s = String::from("abc");
    consume(&s);
    println!("{s}"); // ❌ fix it (choose 1: clone OR change function signature later)
}

// O4 — Return ownership back
fn give_back(s: String) -> String {
    // TODO
    s
}
fn return_ownership() {
    let s = String::from("x");
    let s = give_back(s);
    println!("{s}");
}

// O5 — Clone vs Move
fn clone_vs_move() {
    let s1 = String::from("data"); // copy
    let s2 = s1.clone(); // copy
    let s3 = s1.clone(); // move

    println!("{s2} {s3}");
    println!("{s1}"); // compile? why?
}

// O6 — Scope drop (predict valid region)
fn scope_drop() {
    let s;
    {
        s = String::from("inside");
        println!("{s}");
    }
}

// O7 — Vec move
fn vec_move() {
    let v1 = vec![1, 2, 3];
    let v2 = v1;
    println!("{v2:?}");
    // println!("{v1:?}"); // compile? why?
}

// O8 — Move in if/else (ownership path)
fn move_in_if_else() {
    let s = String::from("abc");

    let t = if s.len() > 0 {
        s
    } else {
        String::from("empty")
    };

    println!("{t}");
    // println!("{s}"); // compile? why?
}

// O9 — Drop early
fn drop_early() {
    let big = vec![0u8; 1_000_000];
    println!("len={}", big.len());
    drop(big);
    // TODO: drop early here
    println!("after drop line");
}

// O10 — Ownership in tuple
fn ownership_in_tuple() {
    let s = String::from("hi");
    let x = 1;

    let t = (s, x);
    println!("{}", t.1);

    // println!("{}", s.clone()); // compile? why?
    println!("{}", t.0);
}