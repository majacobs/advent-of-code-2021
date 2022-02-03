use rayon::prelude::*;

pub fn run() {
    println!("Day 24");
    println!("  Problem 1: {}", problem1());
}

macro_rules! build_alu {
    ( $name:ident, $w:ident, $x:ident, $y:ident, $z:ident, { $($rest:tt)* } ) => {
        fn $name(mut input: impl Iterator<Item=i64>) -> (i64, i64, i64, i64) {
            let mut $w = 0;
            let mut $x = 0;
            let mut $y = 0;
            let mut $z = 0;
            build_alu!(@inner input, $w, $x, $y, $z, {$($rest)*});
            ($w, $x, $y, $z)
        }
    };
    ( @inner $input:ident, $w:ident, $x:ident, $y:ident, $z:ident, { inp $a:ident $($rest:tt)* } ) => {
        $a = $input.next().expect("No more input available");
        build_alu!(@inner $input, $w, $x, $y, $z, {$($rest)*});
    };
    ( @inner $input:ident, $w:ident, $x:ident, $y:ident, $z:ident, { add $a:ident $b:ident   $($rest:tt)* } ) => { $a += $b; build_alu!(@inner $input, $w, $x, $y, $z, {$($rest)*} ); };
    ( @inner $input:ident, $w:ident, $x:ident, $y:ident, $z:ident, { add $a:ident $b:literal $($rest:tt)* } ) => { $a += $b; build_alu!(@inner $input, $w, $x, $y, $z, {$($rest)*} ); };
    ( @inner $input:ident, $w:ident, $x:ident, $y:ident, $z:ident, { mul $a:ident $b:ident   $($rest:tt)* } ) => { $a *= $b; build_alu!(@inner $input, $w, $x, $y, $z, {$($rest)*} ); };
    ( @inner $input:ident, $w:ident, $x:ident, $y:ident, $z:ident, { mul $a:ident $b:literal $($rest:tt)* } ) => { $a *= $b; build_alu!(@inner $input, $w, $x, $y, $z, {$($rest)*} ); };
    ( @inner $input:ident, $w:ident, $x:ident, $y:ident, $z:ident, { div $a:ident $b:ident   $($rest:tt)* } ) => { $a /= $b; build_alu!(@inner $input, $w, $x, $y, $z, {$($rest)*} ); };
    ( @inner $input:ident, $w:ident, $x:ident, $y:ident, $z:ident, { div $a:ident $b:literal $($rest:tt)* } ) => { $a /= $b; build_alu!(@inner $input, $w, $x, $y, $z, {$($rest)*} ); };
    ( @inner $input:ident, $w:ident, $x:ident, $y:ident, $z:ident, { mod $a:ident $b:ident   $($rest:tt)* } ) => { $a %= $b; build_alu!(@inner $input, $w, $x, $y, $z, {$($rest)*} ); };
    ( @inner $input:ident, $w:ident, $x:ident, $y:ident, $z:ident, { mod $a:ident $b:literal $($rest:tt)* } ) => { $a %= $b; build_alu!(@inner $input, $w, $x, $y, $z, {$($rest)*} ); };
    ( @inner $input:ident, $w:ident, $x:ident, $y:ident, $z:ident, { eql $a:ident $b:ident   $($rest:tt)* } ) => { $a = ($a == $b).into(); build_alu!(@inner $input, $w, $x, $y, $z, {$($rest)*} ); };
    ( @inner $input:ident, $w:ident, $x:ident, $y:ident, $z:ident, { eql $a:ident $b:literal $($rest:tt)* } ) => { $a = ($a == $b).into(); build_alu!(@inner $input, $w, $x, $y, $z, {$($rest)*} ); };
    ( @inner $input:ident, $w:ident, $x:ident, $y:ident, $z:ident, { } ) => { };
}

include!("../input/day24");
/*
//fn alu(_: impl Iterator<Item=i32>) -> (i32, i32, i32, i32) {
//    todo!()
//}
build_alu!(alu, w, x, y, z, {
// 1
inp w
mul x 0
add x z
mod x 26
div z 1
add x 10
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 12
mul y x
add z y
// 2
inp w
mul x 0
add x z
mod x 26
div z 1
add x 12
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 7
mul y x
add z y
// 3
inp w
mul x 0
add x z
mod x 26
div z 1
add x 10
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 8
mul y x
add z y
// 4
inp w
mul x 0
add x z
mod x 26
div z 1
add x 12
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 8
mul y x
add z y
// 5
inp w
mul x 0
add x z
mod x 26
div z 1
add x 11
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 15
mul y x
add z y
// 6
inp w
mul x 0
add x z
mod x 26
div z 26
add x -16
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 12
mul y x
add z y
// 7
inp w
mul x 0
add x z
mod x 26
div z 1
add x 10
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 8
mul y x
add z y
// 8
inp w
mul x 0
add x z
mod x 26
div z 26
add x -11
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 13
mul y x
add z y
/*
// 9
inp w
mul x 0
add x z
mod x 26
div z 26
add x -13
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 3
mul y x
add z y
// 10
inp w
mul x 0
add x z
mod x 26
div z 1
add x 13
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 13
mul y x
add z y
// 11
inp w
mul x 0
add x z
mod x 26
div z 26
add x -8
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 3
mul y x
add z y
// 12
inp w
mul x 0
add x z
mod x 26
div z 26
add x -1
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 9
mul y x
add z y
// 13
inp w
mul x 0
add x z
mod x 26
div z 26
add x -4
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 4
mul y x
add z y
// 14
inp w
mul x 0
add x z
mod x 26
div z 26
add x -14
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 13
mul y x
add z y
*/
});
*/

fn simplified(mut input: impl Iterator<Item=i32>) -> (i32, i32, i32, i32) {
    let mut w = 0i32;
    let mut x = 0i32;
    let mut y = 0i32;
    let mut z = 0i32;

    // 1
    w = input.next().unwrap();
    z = w + 12;

    // 2
    w = input.next().unwrap();
    z = (z * 26) + w + 7;

    // 3
    w = input.next().unwrap();
    z = (z * 26) + w + 8;

    // 4
    w = input.next().unwrap();
    z = (z * 26) + w + 8;

    // 5
    w = input.next().unwrap();
    z = (z * 26) + w + 15;

    // 6
    w = input.next().unwrap();
    x = z % 26 - 16;
    x = (x != w) as i32;
    z = (z / 26) * ((25 * x) + 1) + (w + 12) * x;

    // 7
    w = input.next().unwrap();
    z = (z * 26) + w + 8;

    // 8
    w = input.next().unwrap();
    x = z % 26 - 11;
    x = (x != w) as i32;
    z /= 26;
    y = 25 * x + 1;
    z *= y;
    y = (w + 13) * x;
    z += y;

    /*
    // 9
    w = input.next().unwrap();
    x = 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -13;
    x = (x == w) as i32;
    x = (x == 0) as i32;
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += w;
    y += 3;
    y *= x;
    z += y;

    // 10
    w = input.next().unwrap();
    x = 0;
    x += z;
    x %= 26;
    x += 13;
    x = (x == w) as i32;
    x = (x == 0) as i32;
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += w;
    y += 13;
    y *= x;
    z += y;

    // 11
    w = input.next().unwrap();
    x = 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -8;
    x = (x == w) as i32;
    x = (x == 0) as i32;
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += w;
    y += 3;
    y *= x;
    z += y;

    // 12
    w = input.next().unwrap();
    x = 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -1;
    x = (x == w) as i32;
    x = (x == 0) as i32;
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += w;
    y += 9;
    y *= x;
    z += y;

    // 13
    w = input.next().unwrap();
    x = 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -4;
    x = (x == w) as i32;
    x = (x == 0) as i32;
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += w;
    y += 4;
    y *= x;
    z += y;

    // 14
    w = input.next().unwrap();
    x = 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -14;
    x = (x == w) as i32;
    x = (x == 0) as i32;
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += w;
    y += 13;
    y *= x;
    z += y;
    */

    (w, x, y, z)
}

const BASE: i64 = 9;
const DIGITS: u32 = 14;
const CHUNK_SIZE: i64 = 10_000_000;

fn problem1() -> i64 {
    let mut max = BASE.pow(DIGITS) - 1;

    let blah: Vec<_> = (0..=max).rev().step_by(CHUNK_SIZE as usize).take(10_000).collect();
    let v = blah.into_par_iter().find(|n| klonk(*n).is_some());
    println!("{:?}", v);
    let mut start = v.unwrap();


    loop {
        let mut digits = Vec::new();
        let mut x = start;
        for _ in 0..DIGITS {
            digits.push(x % BASE + 1);
            x /= BASE;
        }
        //digits.reverse();
        //println!("{:?} {}", digits, start);
        if start % 1_000_000 == 0 {
            println!("{}", start);
        }

        let (_, _, _, z) = alu(digits.into_iter());
        if z == 0 {
            return start;
        }

        start -= 1;
    }

    /*
    let tests = [
        "31288293787554",
        "21656894783444",
        "17612796679537",
        "82219835129373",
        "47885135986739",
        "79185978272988",
        "19362153428195",
        "62149298757822",
        "71361619821749",
        "72316589515119",
        "23364676596428",
        "83188337246959",
        "93565787732619",
        "88716791737243",
        "26382339382993",
        "45712824873488",
        "83671886197764",
        "91738465523587",
        "66553838364458",
        "73155475435269",
        "47328537858657",
        "44545371799452",
        "57687919148336",
        "61696392885269",
        "22973196738751",
    ];

    for t in tests.iter() {
        let input: Vec<_> = t.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
        let a = alu(input.clone().into_iter());
        let b = simplified(input.into_iter());
        if a != b {
            println!("[{}]: {:?} != {:?}", t, a, b);
            break;
        }
    }
    //'outer: for i in 1..=9 {
    //    for j in 1..=9 {
    //        for k in 1..=9 {
    //            let a = alu([i, j, k].into_iter());
    //            let b = simplified([i, j, k].into_iter());
    //            if a != b {
    //                println!("[{}, {}, {}]: {:?} != {:?}", i, j, k, a, b);
    //                break 'outer;
    //            }
    //        }
    //    }
    //}
//    //let x = 99999999999999i64;
//    let x = 88888888888888i64;
//    let raw_input = x.to_string();
//    let input = raw_input.chars().map(|c| c.to_digit(10).unwrap() as i32);
//    let (w, x, y, z) = alu(input);
    //println!("w = {}", w);
    //println!("x = {}", x);
    //println!("y = {}", y);
    //println!("z = {}", z);
    */
    todo!()
}

fn klonk(mut value: i64) -> Option<i64> {
    println!("{}", value);

    for _ in 0..CHUNK_SIZE {
        let mut digits = Vec::new();
        let mut x = value;
        for _ in 0..DIGITS {
            digits.push(x % BASE + 1);
            x /= BASE;
        }

        let (_, _, _, z) = alu(digits.into_iter());
        if z == 0 {
            return Some(value);
        }
        value -= 1;
    }
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "13579246899999";

    build_alu!(sample_alu, w, x, y, z, {
        inp w
        add z w
        mod z 2
        div w 2
        add y w
        mod y 2
        div w 2
        add x w
        mod x 2
        div w 2
        mod w 2
    });

    #[test]
    fn example1() {
        let input = SAMPLE.chars().map(|c| c.to_digit(10).unwrap() as i32);
        let output = sample_alu(input);
        assert_eq!(output, (0, 0, 0, 1));
    }
}
