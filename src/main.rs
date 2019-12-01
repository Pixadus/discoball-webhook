#[macro_use]
extern crate nickel;
extern crate sysfs_gpio;

use nickel::{Nickel, HttpRouter, Request, Response, MiddlewareResult, StaticFilesHandler};
use sysfs_gpio::{Direction, Pin};

fn enable_cors<'mw>(_req: &mut Request, mut res: Response<'mw>) -> MiddlewareResult<'mw> {
    // Set appropriate headers
    res.headers_mut().set_raw("Access-Control-Allow-Origin", vec![b"*".to_vec()]);
    res.headers_mut().set_raw("Access-Control-Allow-Methods", vec![b"*".to_vec()]);
    res.headers_mut().set_raw("Access-Control-Allow-Headers", vec![b"Origin, X-Requested-With, Content-Type, Accept".to_vec()]);

    // Pass control to the next middleware
    res.next_middleware()
}

fn main() {

    let mut server = Nickel::new();

    server.get("/system/:object/:state", middleware! { |request, response|
        let object = request.param("object").unwrap_or("No object specified");
        let state = request.param("state").unwrap_or("No state specified");

        match object {
            "ball" => match state {
                "0" => gpio_0(166),
                "1" => gpio_1(166),
                _ => println!("No state specified")
            },
            "light1" => match state {
                "0" => gpio_0(164),
                "1" => gpio_1(164),
                _ => println!("No state specified")
            },
            "light2" => match state {
                "0" => gpio_0(184),
                "1" => gpio_1(184),
                _ => println!("No state specified")
            },
            "party" => match state {
                "0" => { 
					gpio_0(166);
					gpio_0(164);
					gpio_0(184);
				},
                "1" => {
					gpio_1(166);
					gpio_1(164);
					gpio_1(184);
				},
                _ => println!("No state specified")
            },
            _ => println!("No object specified")
        };

        format!("Got it!");
    });

    server.utilize(StaticFilesHandler::new("/opt/discoball/site/"));
    server.utilize(enable_cors);
	server.options("**", middleware!(""));
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
