extern crate nickel;
extern crate slow_primes;

use std::io::net::ip::Ipv4Addr;
use nickel::{ Nickel, Request, Response, HttpRouter };

fn main() {
    let mut server = Nickel::new();

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

    server.get("/", root);
    server.get("/style.css", style);
    server.get("/script.js", script);
    server.get("/is_prime/:number", is_prime);

    server.listen(Ipv4Addr(0, 0, 0, 0), 80);
}
