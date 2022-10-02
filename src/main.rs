use std::env;

fn main() {
    let argv: Vec<String> = env::args().collect();
    for arg in argv {
        println!("{arg}");
    }

    if true {
        println!("1")
    } else {
        println!("2")
    }

    while false {
        println!("3")
    }

    for _ in 1..=1 {
        println!("4")
    }

    loop {
        println!("5");
        if true {
            break;
        }
    }

    let mut o = Some(1);
    match o {
        // а - пример деструктуризации
        Some(a) => {
            println!("a={a}")
        }
        None => {
            println!("None")
        }
    }

    if let Some(a) = o {
        println!("{a}");
    }

    while let Some(_a) = o {
        o = None;
    }
}
