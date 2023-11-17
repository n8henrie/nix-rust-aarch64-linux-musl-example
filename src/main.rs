use hyper_proxy::{Intercept, Proxy};

fn main() {
    let proxy_uri = "http://my-proxy:8080".parse().unwrap();
    let proxy = Proxy::new(Intercept::All, proxy_uri);
    dbg!(proxy);
}
