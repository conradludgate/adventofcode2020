mod parse;
use crate::{parsers::all, Challenge};

pub struct Day13 {
    time: u128,
    buses: Vec<Option<u128>>,
}

impl Challenge for Day13 {
    fn name() -> &'static str {
        "day13"
    }
    fn new(input: String) -> Self {
        let (time, buses) = all(parse::input(&input));
        Day13 { time, buses }
    }
    fn part_one(&self) -> usize {
        let (earliest_time, bus) = find_earliest_bus(self.time, &self.buses);
        (bus * (earliest_time - self.time)) as usize
    }
    fn part_two(&self) -> usize {
        find_earliest_series(&self.buses) as usize
    }
}

fn find_earliest_bus(mut time: u128, buses: &Vec<Option<u128>>) -> (u128, u128) {
    loop {
        for bus in buses.iter().filter_map(|&x| x) {
            if time % bus == 0 {
                return (time, bus);
            }
        }
        time += 1;
    }
}

#[test]
fn test_find_earliest_bus() {
    let time = 939;
    let buses = vec![
        Some(7),
        Some(13),
        None,
        None,
        Some(59),
        None,
        Some(31),
        Some(19),
    ];
    let (time, bus) = find_earliest_bus(time, &buses);
    assert_eq!(time, 944);
    assert_eq!(bus, 59)
}

fn find_earliest_series(buses: &Vec<Option<u128>>) -> u128 {
    let bus_orders: Vec<_> = buses
        .iter()
        .enumerate()
        .filter_map(|(t, &bus)| bus.map(|bus| (t as u128, bus as u128)))
        .collect();

    let (a, n) = crt(&bus_orders);
    n - (a % n)
}

// Chinese Remainder Theorem
fn crt(xs: &[(u128, u128)]) -> (u128, u128) {
    if xs.len() == 1 {
        xs[0]
    } else {
        let (a0, n0) = xs[0];
        let (a1, n1) = crt(&xs[1..]);
        let (m0, m1) = ee(n0, n1, n0 * n1);
        let n = n0 * n1;
        (mul(mul(a0, m1, n), n1, n) + mul(mul(a1, m0, n), n0, n), n)
    }
}

fn mul(a: u128, b: u128, n: u128) -> u128 {
    let a = a % n;
    let b = b % n;
    match a.checked_mul(b) {
        Some(c) => c % n,
        None => ((n - a) * (n - b)) % n,
    }
}

fn subtract(a: u128, b: u128, n: u128) -> u128 {
    let a = a % n;
    let b = b % n;
    if b > a {
        return (n - b) + a;
    } else {
        return a - b;
    }
}

// Extended Euclidean
fn ee(a: u128, b: u128, n: u128) -> (u128, u128) {
    let mut s: (u128, u128) = (1, 0);
    let mut t: (u128, u128) = (0, 1);
    let mut r: (u128, u128) = (a, b);
    while r.1 != 0 {
        let q = r.0 / r.1;
        r = (r.1, subtract(r.0, q * r.1, n));
        s = (s.1, subtract(s.0, q * s.1, n));
        t = (t.1, subtract(t.0, q * t.1, n));
    }
    (s.0, t.0)
}

#[test]
fn test_find_earliest_series() {
    // The earliest timestamp that matches the list 17,x,13,19 is 3417.
    // 67,7,59,61 first occurs at timestamp 754018.
    // 67,x,7,59,61 first occurs at timestamp 779210.
    // 67,7,x,59,61 first occurs at timestamp 1261476.
    // 1789,37,47,1889 first occurs at timestamp 1202161486.

    let buses = vec![Some(17), None, Some(13), Some(19)];
    let time = find_earliest_series(&buses);
    assert_eq!(time, 3417);

    let buses = vec![Some(1789), Some(37), Some(47), Some(1889)];
    let time = find_earliest_series(&buses);
    assert_eq!(time, 1202161486);
}

