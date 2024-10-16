use quantise::quantise_from_file;
use train::train;

pub mod arch;
pub mod dataloader;
pub mod inference;
pub mod net;
pub mod quantise;
pub mod train;
pub mod types;

fn main() {
    println!("ig this is a policy net trainer");
    quantise_from_file();
    //train();
}
