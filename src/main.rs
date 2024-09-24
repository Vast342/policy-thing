use train::train;

pub mod types;
pub mod inference;
pub mod dataloader;
pub mod arch;
pub mod train;

fn main() {
    println!("ig this is a policy net trainer");
    train();
}
