#![allow(unreachable_code)]
#[macro_use]
extern crate rouille;
extern crate serde;
#[macro_use] extern crate serde_derive;
use chrono::prelude::*;
use chrono_tz::US::Pacific;
use chrono_tz::Europe::Berlin;
use chrono_tz::Asia::Tokyo;
fn main() {
    #[derive(Serialize)]
    struct Health {
        status: String,
        message: String 
    }
    println!("Now listening on localhost:8080");
    let utc = Utc::now().naive_utc();
    let ny = Pacific.from_utc_datetime(&utc);
    let berlin = Berlin.from_utc_datetime(&utc);
    let tokyo = Tokyo.from_utc_datetime(&utc);
    rouille::start_server("localhost:8080", move |request| {
        router!(request,
            (GET) (/) => {
                rouille::Response::html(format!("<!DOCTYPE html> \
                    <html> \
                    <body> \
                    <h1>Time application</h1> \
                    <table style='width:50%'> \
                        <tr> \
                            <th>Timezone</th> \
                            <th>Time</th> \
                        </tr> \
                        <tr> \
                            <td>NewYork</td> \
                            <td>{}</td> \
                        </tr> \
                        <tr> \
                            <td>Berlin</td> \
                            <td>{}</td> \
                        </tr> \
                        <tr> \
                            <td>Tokyo</td> \
                            <td>{}</td> \
                        </tr> \
                    </table> \
                    </body> \
                    </html>", ny, berlin, tokyo))
            },

            (GET) (/health) => {
                rouille::Response::json(&Health { status: "success".to_owned(), message: "null".to_owned()})
            },

            _ => rouille::Response::empty_404()
        )
    });
}