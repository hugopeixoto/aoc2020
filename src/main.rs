#![feature(test)]

extern crate test;
#[macro_use]
extern crate scan_fmt;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;

#[bench] fn bench_day01(b: &mut test::Bencher) { b.iter(|| day01::main()); }
#[bench] fn bench_day02(b: &mut test::Bencher) { b.iter(|| day02::main()); }
#[bench] fn bench_day03(b: &mut test::Bencher) { b.iter(|| day03::main()); }
#[bench] fn bench_day04(b: &mut test::Bencher) { b.iter(|| day04::main()); }
#[bench] fn bench_day05(b: &mut test::Bencher) { b.iter(|| day05::main()); }
#[bench] fn bench_day06(b: &mut test::Bencher) { b.iter(|| day06::main()); }
#[bench] fn bench_day07(b: &mut test::Bencher) { b.iter(|| day07::main()); }
#[bench] fn bench_day08(b: &mut test::Bencher) { b.iter(|| day08::main()); }
#[bench] fn bench_day09(b: &mut test::Bencher) { b.iter(|| day09::main()); }
#[bench] fn bench_day10(b: &mut test::Bencher) { b.iter(|| day10::main()); }
#[bench] fn bench_day11(b: &mut test::Bencher) { b.iter(|| day11::main()); }
#[bench] fn bench_all(b: &mut test::Bencher) { b.iter(|| main()); }

fn main() {
    day01::main();
    day02::main();
    day03::main();
    day04::main();
    day05::main();
    day06::main();
    day07::main();
    day08::main();
    day09::main();
    day10::main();
    day11::main();
}
