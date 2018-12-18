extern crate rand;
use rand::Rng;

fn main() {
    let mut a: Vec<i32> = (1..=10).collect();
    {
        let slice: &mut [i32] = &mut a;
        let mut rng = rand::thread_rng();
        rng.shuffle(slice);
        println!("Initial values: {:?}", slice);
    }
    let end: usize = a.len() - 1;
    int32_sort(&mut a, 0, end);
    println!("Final values: {:?}", a);
}

fn int32_sort(mut vec: &mut Vec<i32>, lo: usize, hi: usize){
    if hi <= lo { return };
    let j: usize = partition(vec, lo, hi);
    int32_sort(&mut vec, lo, j-1);
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
            if j == lo{ break;}
            j -= 1;
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
