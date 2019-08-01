#[macro_use]
extern crate nickel;
extern crate sysfs_gpio;

use nickel::{Nickel, HttpRouter, StaticFilesHandler};
use sysfs_gpio::{Direction, Pin};

fn main() {

    let mut server = Nickel::new();

    server.get("/system/:object/:state", middleware! { |request, response|
        let object = request.param("object").unwrap_or("No object specified");
        let state = request.param("state").unwrap_or("No state specified");

        match object {
            "ball" => match state {
                "0" => gpio_0(27),
                "1" => gpio_1(27),
                _ => println!("No state specified")
            },
            "light1" => match state {
                "0" => gpio_0(17),
                "1" => gpio_1(17),
                _ => println!("No state specified")
            },
            "light2" => match state {
                "0" => gpio_0(18),
                "1" => gpio_1(18),
                _ => println!("No state specified")
            },
            "party" => match state {
                "0" => { 
					gpio_0(27);
					gpio_0(17);
					gpio_0(18);
				},
                "1" => {
					gpio_1(27);
					gpio_1(17);
					gpio_1(18);
				},
                _ => println!("No state specified")
            },
            _ => println!("No object specified")
        };

        format!("Got it!");
    });

    server.utilize(StaticFilesHandler::new("/home/pi/webserver/site/"));
    server.listen("0.0.0.0:80").unwrap();
}

fn gpio_0(pin_number: u64) {
    let pin = Pin::new(pin_number);
    pin.export().unwrap();
    if pin.is_exported() {
        println!("Pin is now exported");
    }
    else {
        println!("Pin is still unexported");
    }
    pin.set_direction(Direction::Low).unwrap();
    pin.set_value(0).unwrap();
}

fn gpio_1(pin_number: u64) {
    let pin = Pin::new(pin_number);
    pin.export().unwrap();
    if pin.is_exported() {
        println!("Pin is now exported");
    }
    else {
        println!("Pin is still unexported");
    }
    pin.set_direction(Direction::Low).unwrap();
    pin.set_value(1).unwrap();
}
