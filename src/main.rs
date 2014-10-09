extern crate nickel;
extern crate slow_primes;
extern crate time;

use std::io::{mod, File};
use std::io::net::ip::Ipv4Addr;
use nickel::{ Nickel, Request, Response, HttpRouter, MiddlewareResult };

static LOG_FILE: &'static str = "requests.log";

fn main() {
    let mut server = Nickel::new();

    fn log(request: &Request, _response: &mut Response) -> MiddlewareResult {
        (writeln!(File::open_mode(&Path::new(LOG_FILE), io::Append, io::Write).unwrap(),
                  "{time}\t{ip}\t{uri}",
                  time = time::now_utc().rfc3339(),
                  ip = request.origin.remote_addr,
                  uri = request.origin.request_uri))
            .unwrap();


        Ok(nickel::Continue)
    }

    fn root(_request: &Request, response: &mut Response) {
        response.content_type("html");
        response.send(include_str!("assets/index.html"));
    }
    fn style(_request: &Request, response: &mut Response) {
        response.content_type("css");
        response.send(include_str!("assets/style.css"));
    }
    fn script(_request: &Request, response: &mut Response) {
        response.content_type("js");
        response.send(include_str!("assets/script.js"))
    }
    fn is_prime(request: &Request, response: &mut Response) {
        response.content_type("json");

        let input = request.param("number");
        match from_str(input) {
            None => response.send(format!("{{\"input\":{},\"error\":\"invalid number\"}}",
                                          input)),
            Some(n) => {
                let is_prime = slow_primes::is_prime_miller_rabin(n);
                response.send(format!("{{\"input\":{},\"is_prime\":{}}}",
                                      input, is_prime));
            }
        }
    }

    server.utilize(log);

    server.get("/", root);
    server.get("/style.css", style);
    server.get("/script.js", script);
    server.get("/is_prime/:number", is_prime);

    server.listen(Ipv4Addr(0, 0, 0, 0), 5000);
}
