use crate::model::MyItem;
use std::env;
use warp::Filter;

pub fn start_server_if_test() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "test" {
        let route = warp::path("endpoint")
            .and(warp::post())
            .and(warp::body::json())
            .map(|item: MyItem| {
                println!("{:#?}", item);
                warp::reply()
            });
        println!("Launching test server at http://localhost:3030/endpoint");
        tokio::spawn(warp::serve(route).run(([127, 0, 0, 1], 3030)));
    }
}
