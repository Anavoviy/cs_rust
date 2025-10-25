mod two_pointers;

pub fn array_statistics(arr: &[i32]) -> (i32, i32, i32, f64){

    let mut min = arr[0];
    let mut max = arr[0];
    let mut sum = arr[0];

    for &value in arr.iter().skip(1) {
        if value < min {
            min = value;
        }
        if value > max {
            max = value;
        }
        sum += value;
    }

    (min, max, sum, sum as f64 / arr.len() as f64)
}

pub fn find_element(arr: &[i32], target: i32) -> Option<usize>{
    for (index, &value) in arr.iter().enumerate() {
        if value == target{
            return Some(index);
        }
    }
    None
}

pub fn filter_even(arr: &[i32]) -> Vec<i32> {
    let mut even: Vec<i32> = Vec::new();

    for &value in arr.iter(){
        if value % 2 == 0 {
            even.push(value);
        }
    }

    even
}

pub fn is_sorted(arr: &[i32]) -> bool {

    if arr.len() <= 1 { return true; }

    for i in 1..arr.len() {
        if arr[i] < arr[i-1] {
            return false;
        }
    }

    true
}