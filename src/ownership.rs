// Stage 1.4 — Ownership Basics playground
// Run: cargo run
// Tips: Uncomment lines marked "UNCOMMENT to see error" one-by-one.

#[derive(Clone, Debug)]
struct User {
    name: String,
    age: u8,
}

fn consume_string(s: String) {
    println!("consume_string got: {s}");
}

struct Droppable {
    name: &'static str,
}

// This trivial implementation of `drop` adds a print to console.
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

fn ownership() {
    println!("=== COPY examples ===");

    // Copy #1: i32
    let a: i32 = 10;
    let b = a; // copy
    println!("Copy i32 -> a={a}, b={b}");

    // Copy #2: bool
    let ok: bool = true;
    let ok2 = ok; // copy
    println!("Copy bool -> ok={ok}, ok2={ok2}");

    // Copy #3: tuple (all elements are Copy)
    let p = (10i32, 20i32);
    let q = p; // copy
    println!("Copy tuple -> p={p:?}, q={q:?}");

    println!("\n=== MOVE examples ===");

    // Move #1: String assignment
    let s1 = String::from("hello");
    let s2 = s1; // move
    // println!("UNCOMMENT to see error: s1={s1}"); // ❌ use of moved value: `s1`
    println!("Move String -> s2={s2}");

    // Move #2: Vec assignment
    let v1 = vec![1, 2, 3];
    let v2 = v1; // move
    // println!("UNCOMMENT to see error: v1={v1:?}"); // ❌ use of moved value: `v1`
    println!("Move Vec -> v2={v2:?}");

    // Move #3: pass String into function (ownership to function)
    let s3 = String::from("bye");
    consume_string(s3); // move into function
    // println!("UNCOMMENT to see error: s3={s3}"); // ❌ use of moved value: `s3`

    println!("\n=== CLONE examples ===");

    // Clone #1: String clone
    let cs1 = String::from("clone me");
    let cs2 = cs1.clone(); // duplicate heap data
    println!("Clone String -> cs1={cs1}, cs2={cs2}");

    // Clone #2: Vec clone
    let cv1 = vec![10, 20, 30];
    let cv2 = cv1.clone(); // duplicate buffer
    println!("Clone Vec -> cv1={cv1:?}, cv2={cv2:?}");

    // Clone #3: struct clone
    let u1 = User {
        name: "Irsyad".into(),
        age: 30,
    };
    let u2 = u1.clone();
    println!("Clone struct -> u1={u1:?}, u2={u2:?}");

    println!("\nDone. Now try uncommenting the error lines one by one.");

    // Mini-drill
    // 1. Create Copy
    let x = 100;
    let y = x;
    println!("Mini-drill Copy -> x={x}, y={y}");

    // 2. Create Move dan sengaja bikin error “use of moved value”.
    let s4 = String::from("Move me");
    let s5 = s4;
    // println!("UNCOMMENT to see error: s4={s4}"); // ❌ use of moved value: `s4`
    println!("After move -> s5={s5}");

    // 3. Fix Error dengan Clone
    let s6 = s5.clone();
    println!("Mini-drill Clone -> s5={s5}, s6={s6}");

    // 4. Create Drop
    let _a = Droppable { name: "a" };

    // block A
    {
        let _b = Droppable { name: "b" };

        // block B
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };

            println!("Exiting block B");
        }
        println!("Just exited block B");

        println!("Exiting block A");
    }
    println!("Just exited block A");

    // Variable can be manually dropped using the `drop` function
    drop(_a);
    // TODO ^ Try commenting this line

    println!("end of the main function");

    // `_a` *won't* be `drop`ed again here, because it already has been
    // (manually) `drop`ed
}
