#[allow(unused_variables)]
#[allow(arithmetic_overflow)]
// lecture-2

fn main() {
    let s = String::new();
    let str = s.as_str();
    let ui8 = 0u8;
    let ui64: u64 = 0;
    let b = false;
    let mut vec: Vec<i128> = vec![];

    // ownership
    // let mut vec1 = vec;
    // f1(vec);
    f2(&mut vec);
    vec.push(2);
    // let p = vec.as_mut_ptr();
    // *p += 1;

    // lifetime
    // {
    // let v = vec![1];
    // }
    // v.push(1);

    // debug (medleno) release (bystro)
    // let mut i = 0u128;
    // for _ in 0..=1000000000 {
    //     i += 1;
    // }
    // println!("{i}");

    // debug -> panic
    // release -> wrap
    // let mut u = u8::MAX;
    // u += 1;
    // println!("{u}")
}

fn f1(mut vec: Vec<i128>) {
    vec.push(1);
}

fn f2(vec: &mut Vec<i128>) {
    vec.push(1);
}

#[derive(Debug)]
struct S1 {
    a1: u8,
    a2: String,
}

impl S1 {
    fn add(&mut self, wta: u8) {
        self.a1 += wta;
    }

    fn get_a1(&self) -> u8 {
        self.a1
    }
}

struct LF<'a> {
    a1: &'a str,
}

#[derive(PartialEq, Eq)]
enum CoinState {
    Orel,
    Reshka,
}
