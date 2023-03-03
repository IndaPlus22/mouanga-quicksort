use std::io;
use std::io::Read;

// for "randomness"
use std::time::{SystemTime, UNIX_EPOCH};

// since global variables must be decided at compile time, i've the decided the time for you. happy?
const SYS_TIME: usize = 1677827006;

fn main() {
let mut input = String::new();

io::stdin().lock().read_to_string(&mut input);
    
let mut _vec = input
    .split_whitespace()
    .skip(1)
    .map(|value| value.parse::<isize>().unwrap())
    .collect::<Vec<isize>>();

println!("{:?}", _vec);
qsort_fp(&mut _vec);
println!("{:?}", _vec);





}

fn isort(list: &mut [isize], len: usize) {
    for i in 1..len - 1 {
        let mut j = i;
        while j > 0 && list[j-1] > list[j] {
            list.swap(j, j-1);
            j -= 1;
        } 
    }
}

// quick sort, fixed pivot
fn qsort_fp(list: &mut Vec<isize>) {
    // initialize the stack
    let mut stack: Vec<(usize, usize)> = Vec::with_capacity(list.len());
    stack.push((0, list.len()));

    // this is equivalent to running the code recursively, except now the stack can actually be more than 0.000001 kB! wow!
    while let Some((low, high)) = stack.pop() { 
        if high - low <= 1 {
            continue;
        }
        let sorted_index = partition(list, low, high, false);
        stack.push((low, low + sorted_index));
        stack.push((low + sorted_index + 1, high));
    }
}

// quick sort, random pivot
fn qsort_rp(list: &mut Vec<isize>) {
    // initialize the stack
    let mut stack: Vec<(usize, usize)> = Vec::with_capacity(list.len());
    stack.push((0, list.len()));

    // this is equivalent to running the code recursively, except now the stack can actually be more than 0.000001 kB! wow!
    while let Some((low, high)) = stack.pop() { 
        if high - low <= 1 {
            continue;
        }
        let sorted_index = partition(list, low, high, true);
        stack.push((low, low + sorted_index));
        stack.push((low + sorted_index + 1, high));
    }
}


// quick sort, fixed pivot, insertion sort
fn qsort_fpi(list: &mut Vec<isize>) {
    // initialize the stack
    let mut stack: Vec<(usize, usize)> = Vec::with_capacity(list.len());
    stack.push((0, list.len()));
    let mut sorted_min: usize = 0;
    let mut sorted_max: usize = 0;
    // this is equivalent to running the code recursively, except now the stack can actually be more than 0.000001 kB! wow!
    while let Some((low, high)) = stack.pop() { 
        if high - low <= 1 || (low >= sorted_min && high <= sorted_max) {
            continue;
        }
        // use insertion sort for small arrays
        if high - low < 6 {
            isort(&mut list[low..high], high-low);
            if low < sorted_min {
                sorted_min = low;
            }
            if high > sorted_max {
                sorted_max = high;
            }
        }
        let sorted_index = partition(list, low, high, false);
        stack.push((low, low + sorted_index));
        stack.push((low + sorted_index + 1, high));
    }
}

// quick sort, insertion sort, random pivot
fn qsort_rpi(list: &mut Vec<isize>) {
    // initialize the stack
    let mut stack: Vec<(usize, usize)> = Vec::with_capacity(list.len());
    stack.push((0, list.len()));
    let mut sorted_min: usize = 0;
    let mut sorted_max: usize = 0;
    // this is equivalent to running the code recursively, except now the stack can actually be more than 0.000001 kB! wow!
    while let Some((low, high)) = stack.pop() { 
        if high - low <= 1 || (low >= sorted_min && high <= sorted_max) {
            continue;
        }
        // use insertion sort for small arrays
        if high - low < 6 {
            isort(&mut list[low..high], high-low);
            if low < sorted_min {
                sorted_min = low;
            }
            if high > sorted_max {
                sorted_max = high;
            }
        }
        let sorted_index = partition(list, low, high, true);
        stack.push((low, low + sorted_index));
        stack.push((low + sorted_index + 1, high));
    }
}

/* 
fn partition(list: &mut Vec<isize>, low: usize, high: usize, rand: bool) -> usize {
    let mut pivot = 0;


    // "randomly" use a different element as the pivot, if requested
    if rand {
        pivot = list[SYS_TIME % list.len()];
        println!("pivot: {}", SYS_TIME % list.len());
    } else {
        println!("pivot: {}", SYS_TIME % list.len());
    }
    let mut i = low;
    let mut j: usize = high - 1;
    while i < j {
        while list[i] <= pivot { i += 1;}
        while list[j] >  pivot { j -= 1;}
        if i < j {
            list.swap(i, j);
        }
    }
    list.swap(low, j);
    return j;
}
*/

