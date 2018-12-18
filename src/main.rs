extern crate rand;
use rand::Rng;
use std::time::{Instant};

fn main() {
    //time object used to measure execution time
    let mut now = Instant::now();

    //vector containing all integers in defined range
    let mut a: Vec<i32> = (1..=100_000).collect();
    let mut b: Vec<i32> = (1..=100_000).collect();
    
    //shuffle vector
    {
        let slice: &mut [i32] = &mut a;
        let mut rng = rand::thread_rng();
        rng.shuffle(slice);
    }
    let end: usize = a.len() - 1;

    //sort vector
    int32_sort(&mut a, 0, end);

    println!("{} items sorted using quicksort.", a.len());
    let mut sec = now.elapsed().as_secs();
    let mut nano = now.elapsed().subsec_nanos();
    println!("Execution time: {}ms", (sec * 1_000) + (nano / 1_000_000) as u64);
    
    now = Instant::now();
    
    b.sort();
    
    sec = now.elapsed().as_secs();
    nano = now.elapsed().subsec_nanos();
    println!("{} items sorted using sort().", b.len());
    println!("Execution time: {}ms", (sec * 1_000) + (nano / 1_000_000) as u64);
}

fn int32_sort(mut vec: &mut Vec<i32>, lo: usize, hi: usize){
    if hi <= lo { return };

    //find partition index
    let j: usize = partition(vec, lo, hi);
    //sort lower partition
    int32_sort(&mut vec, lo, j-1);
    //sort upper partition
    int32_sort(&mut vec, j+1, hi);
}

fn partition(vec: &mut Vec<i32>, lo: usize, hi: usize) -> usize{
    let mut i: usize = lo;
    let mut j: usize = hi + 1;
    let v: i32 = vec[lo];
    loop {
        i += 1;
        while vec[i] < v {
            if i == hi{ break;}
            i += 1;
        }
        j -= 1;
        while v < vec[j] {
            if j == lo || j == 1{ break;}
            if j > 1{
                j -= 1
            };
        }
        if i >= j { break; }
        if j == vec.len() { j -= 1; }
        exch(vec, i, j);
    }
    exch(vec, lo, j);
    return j;
}

fn exch(vector: &mut Vec<i32>, i: usize, j: usize){
    let temp: i32 = vector[i];
    vector[i] = vector[j];
    vector[j] = temp;
}
