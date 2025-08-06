use std::net::{IpAddr, SocketAddr};

fn main() {
    println!("Refactor me!");

    let err = Error::new("NO_USER".to_string());
    err.status(404).message("User not found".to_string());
}

#[derive(Debug)]
pub struct Error {
    code: String,
    status: u16,
    message: String,
}

impl Default for Error {
    fn default() -> Self {
        Self {
            code: "UNKNOWN".into(),
            status: 500,
            message: "Unknown error has happened.".into(),
        }
    }
}

impl Error {
    pub fn new<S: Into<String>>(code: S) -> Self {
        Self {
            code: code.into(),
            ..Self::default()
        }
    }

    pub fn status(mut self, s: u16) -> Self {
        self.status = s;
        self
    }

    pub fn message<S: Into<String>>(mut self, m: S) -> Self {
        self.message = m.into();
        self
    }
}

#[derive(Debug, Default)]
pub struct Server(Option<SocketAddr>);

impl Server {
    pub fn bind(&mut self, ip: impl Into<IpAddr>, port: u16) {
        self.0 = Some(SocketAddr::new(ip.into(), port))
    }
}

#[cfg(test)]
mod server_spec {
    use super::*;

    mod bind {
        use std::net::Ipv4Addr;

        use super::*;

        #[test]
        fn sets_provided_address_to_server() {
            let mut server = Server::default();

            server.bind(Ipv4Addr::new(127, 0, 0, 1), 8080);
            assert_eq!(format!("{}", server.0.unwrap()), "127.0.0.1:8080");

            server.bind("::1".parse::<IpAddr>().unwrap(), 9911);
            assert_eq!(server.0.unwrap().to_string(), "[::1]:9911");
        }
    }
}
