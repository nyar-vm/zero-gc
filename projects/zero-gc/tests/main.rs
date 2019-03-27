#[allow(unused, dead_code)]
use zgc::{GcObject, GcPointer};

#[test]
fn ready() {
    println!("it works!")
}


// #[test]
// pub fn test() {
//     let a: i32 = -2333;
//     println!("{} at {:p}", a, &a);
//     let mut ptr = GcPointer::make(&a);
//     ptr.set_marked0(true);
//     ptr.set_marked1(false);
//     ptr.set_remapped(true);
//     ptr.set_finalize(false);
//     println!("{:#?}", ptr);
//     // println!("{:p}", ptr.as_pointer());
//     let d = *unsafe { ptr.cast::<i32>() };
//     println!("value: {}", d);
// }


// #[test]
// pub fn test2() {
//     let a: [i32; 4] = [2, -3, -5, -7];
//     println!("{:?} at {:p}", a, &a);
//     let obj = GcObject::make(a);
//     println!("{:#?}", obj);
//     println!("{:?}", obj.as_bytes());
//     let d = unsafe { obj.cast::<[u32; 4]>() };
//     println!("value: {:?}", d);
// }