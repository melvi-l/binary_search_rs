use core::panic;

fn main() {
    let list = vec![1,2,3,4,5,6,7,8,10];
    let elem = 9;
    println!("searching for {}", elem);
    let index = binary_search(&list, elem);
    println!("we have list[{}] = {}", index, list[index]);
    println!("\n\nRESULT IS {}", index);
}

fn binary_search(list: &[usize], search_el: usize) -> usize {
    if list.len() == 1 && list.first().unwrap() != &search_el {
        panic!("element not found !");
    }
    let pivot_index = list.len() / 2;
    let pivot = list[pivot_index];
    println!("\tlist: {:?}, \n\tpivot: {}", list, pivot);
    if search_el < pivot {
        return binary_search(&list[..pivot_index], search_el)
    }
    if search_el > pivot {
        return pivot_index + 1 + binary_search(&list[pivot_index + 1..], search_el)
    }
    pivot_index
}

#[test]
#[should_panic]
fn empty() {
    let elem = 5;
    let list = vec![];
    binary_search(&list, elem);
}

#[test]
fn one_elem() {
    let elem = 5;
    let list = vec![5];
    assert_eq!(binary_search(&list, elem), 0);
}

#[test]
#[should_panic]
fn one_wrong_elem() {
    let elem = 4;
    let list = vec![5];
    binary_search(&list, elem);
}

#[test]
fn first_pivot() {
    let elem = 5;
    let list = vec![0,1,2,3,4,5,6,7,8,9];
    assert_eq!(binary_search(&list, elem), 5);
}

#[test]
fn first_elem() {
    let elem = 0;
    let list = vec![0,1,2,3,4,5,6,7,8,9];
    assert_eq!(binary_search(&list, elem), 0);
}

#[test]
fn last_elem() {
    let elem = 9;
    let list = vec![0,1,2,3,4,5,6,7,8,9];
    assert_eq!(binary_search(&list, elem), 9);
}

#[test]
#[should_panic]
fn not_in_list() {
    let elem = 69;
    let list = vec![0,1,2,3,4,5,6,7,8,9];
    binary_search(&list, elem);
}
