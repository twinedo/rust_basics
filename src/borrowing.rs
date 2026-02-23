fn print_len(s: &String) {
    println!("len = {}", s.len());
}

fn borrowing() {
    // Borrowing
    // Immutable reference (borrow)
    let s = String::from("hello");
    print_len(&s); // borrow
    println!("{s}"); // ✅ masih bisa dipakai (tidak move)

    // Mutable reference (borrow)
    let mut s2 = String::from("hello");
    print_len(&s2); // borrow
    s2.push_str(" world"); // ✅ masih bisa dipakai (tidak move)
    println!("{s2}");

    // More exercises
    let mut s3 = String::from("abc");
    {
        let r = &s3;
        println!("{r}"); // pemakaian terakhir r ada di sini
    }

    let m = &mut s3; // harus boleh karena r sudah selesai dipakai
    m.push('!');
    // println!("{s3}");

    // Exercise 1 — Mixed borrows (immutable + mutable conflict)
    let mut s = String::from("abc");
    {
        let r = &s;
        println!("before: {r}");
    }
    let m = &mut s; // ❌ ERROR: cannot borrow as mutable because it is also borrowed as immutable

    m.push('!');
    println!("after: {s}");

    // Exercise 2 — Two mutable borrows (two writers)
    let mut v = vec![1, 2, 3];

    {
        let a = &mut v;
        a.push(4);
    }

    {
        let b = &mut v; // ❌ ERROR: cannot borrow `v` as mutable more than once at a time
        b.push(5);
    }

    println!("v = {:?}", v);
}
