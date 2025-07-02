pub mod car;
use car::car_frame::CarFrame;

fn main() {
    println!("Hello, world!");
    let cool_car = CarFrame::new();
    println!("{}", cool_car.get_string());
}
