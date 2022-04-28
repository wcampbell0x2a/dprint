#[cfg(feature = "std")]
use dprint::dprintln;

#[cfg(feature = "replace")]
use dprint::println;

fn main() {
    #[cfg(feature = "std")]
    dprintln!("hi");

    #[cfg(feature = "replace")]
    println!("hi");
}
