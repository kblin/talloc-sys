extern crate talloc_sys as talloc;



fn main() {
    let _mem = talloc::talloc::<u32, u32>(None);
    println!("Hello, world!");
}
