enum Command {
    Left,
    Right,
    Stay,
    Quit,
}

fn command_from_code(code: i32) -> Command {
    match code {
        1 => Command::Left,
        2 => Command::Right,
        3 => Command::Stay,
        _ => Command::Quit, // default case for any other code
    }
}

fn controls() {
  // Tiny deliverable input (you can change this number to test)
    let n: i32 = 17;

    // 1) if/else deeper: condition must be bool, blocks are expressions
    let sign = if n > 0 {
        "positive"
    } else if n < 0 {
        "negative"
    } else {
        "zero"
    };

    // Nested if example (readability tradeoff)
    let magnitude = if n == 0 {
        "none"
    } else {
        if n.abs() >= 100 {
            "large"
        } else {
            "small"
        }
    };

    // Using if as expression safely: BOTH branches return same type (&str)
    let parity = if n % 2 == 0 { "even" } else { "odd" };

    // 2) match intro: basic matching on numbers + wildcard
    let label = match n {
        0 => "exactly zero",
        1..=9 => "1-9",
        10..=99 => "10-99",
        _ => "100+ or negative",
    };

    // 3) loops basics

    // loop: repeats forever until break
    // loop can also RETURN a value via `break value;`
    let first_multiple_of_7_at_or_after = {
        let mut x = n;
        loop {
            if x % 7 == 0 {
                break x; // break with a value
            }
            x += 1;
        }
    };

    // while: repeat while condition is true
    let mut countdown = 3;
    while countdown > 0 {
        println!("countdown: {countdown}");
        countdown -= 1;
    }
    println!("liftoff!");

    // for: best for ranges and iterating known counts
    // 0..5 is 0,1,2,3,4  (end excluded)
    // 0..=5 is 0,1,2,3,4,5 (end included)
    let mut sum = 0;
    for i in 1..=5 {
        // continue: skip this iteration
        if i == 3 {
            continue;
        }
        sum += i;
    }

    println!("n={n} => sign={sign}, magnitude={magnitude}, parity={parity}");
    println!("label={label}");
    println!(
        "first multiple of 7 at/after n is {first_multiple_of_7_at_or_after}"
    );
    println!("sum of 1..=5 skipping 3 is {sum}");

    let codes = [2, 9, 1, 0, 2];
    let mut position = 0;

    for code in codes {
        let cmd = command_from_code(code);

        match cmd {
            Command::Left => {
                position -= 1;
                println!("code={code} => Left  | position={position}");
            }
            Command::Right => {
                position += 1;
                println!("code={code} => Right | position={position}");
            }
            Command::Stay => {
                println!("code={code} => Stay  | position={position}");
                continue;
            }
            Command::Quit => {
                println!("code={code} => Quit  | position={position}");
                break;
            }
        }
    }

    println!("Final position: {position}");

    // Exercise 1 (match): FizzBuzz label (but Rust-y)
    for n in 1..=20 {
        let label = match (n%3 == 0, n%5 == 0) {
            (true, true) => "FizzBuzz",
            (true, false) => "Fizz",
            (false, true) => "Buzz",
            _ => "Other"
        };
        println!("{n}: {label}");
    }

    // Exercise 2 (loops): Nested loops + labeled break (basic)
    'search: for a in 1..=10 {
        for b in 1..=10 {
            if (a*b) == 42 {
                println!("found: a={a}, b={b}");
                break 'search;
            }
        }
    }
}