use splits::lib::split_at_mut;
use rand::Rng;


fn partition(arr: &mut [i32]) -> usize {
    let len = arr.len();
    let pivot = arr[len - 1];
    let mut i = 0;

    for j in 0..len - 1 {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, len - 1);
    i
}

fn main() {
    fn quicksort(arr: &mut [i32]) {
        if arr.len() <= 1 {
            return;
        }

        let mid = partition(arr);
        let (left, right) = split_at_mut(arr, mid);

        quicksort(left);
        quicksort(&mut right[1..]);
    }

    let mut test_vec: Vec<i32> = vec![];

    // used ai to generate this random number generator.
    let mut rng: rand::prelude::ThreadRng = rand::rng();
    let len: i32 = rng.random_range(5..20);

    for _ in 0..len {
        test_vec.push(rng.random_range(-100..100));
    }

    println!("running quicksort that uses split_at_mut using the following Vec<i32>: \n {test_vec:#?}");

    quicksort(&mut test_vec);
    println!("Result: \n {test_vec:#?}");

}
