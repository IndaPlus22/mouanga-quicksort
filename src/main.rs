use std::io;

fn main() {
// totally original i/o code (not)
let mut input = String::new();


let mut VEC: Vec<isize> = vec![3, 4, 1, 2, 5, 8, 9, 10, 15, 29, 0, -5, 4, -33, -33, -6, 0, -8];
println!("{:?}", VEC);
isort(&mut VEC, 19);
println!("{:?}", VEC);


}

// quicksort fixed pivot
// pivot = list[0]
fn qsort_fp(list: &mut Vec<isize>, len: usize) {
let mut i: usize = 0;
let mut j: usize = len;
}

fn isort(list: &mut Vec<isize>, len: usize) {
    for i in 1..len - 1 {
        let mut j = i;
        while j > 0 && list[j-1] > list[j] {
            let temp = list[j];
            list[j] = list[j-1];
            list[j-1] = temp;
            j -= 1;
        } 
    }
}