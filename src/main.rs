#![feature(test)]

extern crate test;
#[macro_use]
extern crate scan_fmt;

mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

#[bench] fn bench_day1(b: &mut test::Bencher) { b.iter(|| day1::main()); }
#[bench] fn bench_day2(b: &mut test::Bencher) { b.iter(|| day2::main()); }
#[bench] fn bench_day3(b: &mut test::Bencher) { b.iter(|| day3::main()); }
#[bench] fn bench_day4(b: &mut test::Bencher) { b.iter(|| day4::main()); }
#[bench] fn bench_day5(b: &mut test::Bencher) { b.iter(|| day5::main()); }
#[bench] fn bench_day6(b: &mut test::Bencher) { b.iter(|| day6::main()); }
#[bench] fn bench_day7(b: &mut test::Bencher) { b.iter(|| day7::main()); }
#[bench] fn bench_day8(b: &mut test::Bencher) { b.iter(|| day8::main()); }
#[bench] fn bench_day9(b: &mut test::Bencher) { b.iter(|| day9::main()); }
#[bench] fn bench_day10(b: &mut test::Bencher) { b.iter(|| day10::main()); }
#[bench] fn bench_all(b: &mut test::Bencher) { b.iter(|| main()); }

fn main() {
    day1::main();
    day2::main();
    day3::main();
    day4::main();
    day5::main();
    day6::main();
    day7::main();
    day8::main();
    day9::main();
    day10::main();
}
