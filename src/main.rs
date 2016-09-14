#[macro_use] extern crate nickel;
extern crate crypto;
extern crate url;

use std::collections::HashMap;
use nickel::{Nickel, HttpRouter, QueryString, StaticFilesHandler};


fn main() {
    let mut server = Nickel::new();

    server.utilize(StaticFilesHandler::new("static/"));

    server.get("/", middleware! { |_req, _resp|
        let mut data = HashMap::new();
        let query = _req.query();
        let cont = query.get("continue").unwrap_or("/");
        data.insert("continue", cont);
        return _resp.render("tmpl/index.html", &data);
    });

    server.listen("127.0.0.1:8080");
}
