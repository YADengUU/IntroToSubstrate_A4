// traffic lights with different duration for different color

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// get the durations
trait GetDuration {
    fn duration(&self) -> u32;
}

impl GetDuration for TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 25,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 60,
        }
    }
}

// main function to print the durations
fn main() {
    let red_light = TrafficLight::Red;
    let yellow_light = TrafficLight::Yellow;
    let green_light = TrafficLight::Green;

    println!("Duration of red light: {} seconds", red_light.duration());
    println!("Duration of yellow light: {} seconds", yellow_light.duration());
    println!("Duration of green light: {} seconds", green_light.duration());
}
