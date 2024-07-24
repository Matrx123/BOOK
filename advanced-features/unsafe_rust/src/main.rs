use core::slice;

fn main() {
    println!("Raw pointer, unsafe block and unsafe function");

    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("R1 : {:?}", *r1);
        println!("R2 : {:?}", *r2);
    }

    println!("Calling an Unsafe Function or Method");

    unsafe fn dangerous() {
        println!("Dangerous method has been called!!!");
    }

    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}
//this will throw error as slice can't be borrowed twice, though they both are different slices but
//compiler thinks they are same

//fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
// let len = slice.len();

//assert!(mid <= len);

//  (&mut slice[..mid], &mut slice[mid..])
//}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();

    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
