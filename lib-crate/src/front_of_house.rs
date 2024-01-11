// child modules should be declared in a directory with the same name as the parent module
// and the file within that directory must be the same name as the child module
// in this case front_of_house/hosting.rs
pub mod hosting;

mod serving {
    fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}
}