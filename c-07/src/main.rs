use std::io::{self};

fn main() {
    fn f7_1_1() {
        fn pirate_share(total: u64, crew_size: usize) -> u64 {
            let half = total / 2;
            half / crew_size as u64
        }
        pirate_share(10, 0);
    }
    fn f7_2_1() {
        #[derive(Debug)]
        struct WetherReport<'a> {
            condition: &'a str,
            temp: u64,
        }
        fn get_weather(location: &'_ str) -> Result<WetherReport<'_>, io::Error> {
            if location.is_empty() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "location must not be empty",
                ));
            }
            Ok(WetherReport {
                condition: "Sunny",
                temp: 20,
            })
        }

        match get_weather(&"japan".to_string()) {
            Ok(report) => {
                println!("japan: {:?}", report);
            }
            Err(err) => {
                print!("eror: {}", err);
            }
        }

        const THE_USUAL: WetherReport = WetherReport {
            condition: "Sunny",
            temp: 22,
        };

        let report = get_weather("").unwrap_or(THE_USUAL);
        println!("report: {:?}", report);
    }

    f7_2_1()
}
