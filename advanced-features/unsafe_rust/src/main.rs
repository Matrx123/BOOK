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
