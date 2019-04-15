use std::collections::HashMap;

const FIB_ZERO: u64 = 1;
const FIB_ONE: u64 = 1;

fn fib(n: u64) -> u64 {
    if n == 0 {
        return FIB_ZERO;
    } else if n == 1 {
        return FIB_ONE;
    }
    else {
        return fib(n - 1) + fib (n - 2);
    }

}
fn fib_dyn(n: u64, map: &mut HashMap<u64, u64>) -> u64
{
    match n {
        0 | 1 => 1,
        n => {
            if map.contains_key(&n)
            {
                return *map.get(&n).unwrap();
            }
            else
            {
                let val = fib_dyn(n-1, map) + fib_dyn(n-2, map);
                map.insert(n, val);
                return val;
            }
        }
    }

}
fn main() {
    println!("Hello, world!");
    let mut map = HashMap::new();
    for i in 1..41 {
        println!("{}: {}", i, fib_dyn(i, &mut map));
    }
}
