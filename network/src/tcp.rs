use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;
pub trait Io {
    fn send(&self, data: &String) -> Option<i32>;
    fn recv(&self) -> Option<String>;
}
pub struct TcpSer {
    con: std::net::TcpStream,
}
pub struct TcpCli {
    con: std::net::TcpStream,
}
impl TcpSer {
    pub fn new(ip_port: &String) -> Result<TcpSer, io::Error> {
        let listener = std::net::TcpListener::bind(ip_port.as_str())?;
        // let mut tser=TcpSer{con:.0};
        // for stream in listener.incoming() {
        //     match stream {
        //         Ok(s) => return Ok(TcpSer { con: s }),
        //         Err(e) => {
        //             println!("TcpSer err:{}", e);
        //             return Err(e);
        //         }
        //     }
        // };
        // None
        let Some(tmp) = listener.incoming().next() else {
            return Err(io::Error::new(io::ErrorKind::Other, "no incoming"));
        };
        match tmp {
            Ok(s) => Ok(TcpSer { con: s }),
            Err(e) => {
                println!("TcpSer err:{}", e);
                Err(e)
            }
        }
    }
}
impl TcpCli {
    pub fn new(ip_port: &String) -> Result<TcpCli, io::Error> {
        // let stream = TcpStream::connect(ip_port);
        // match stream {
        //     Ok(s) => {
        //         println!("TcpCli connect ok");
        //         Ok(TcpCli { con: s })
        //     }
        //     Err(e) => {
        //         println!("TcpCli err:{}", e);
        //         Err(e)
        //     }
        // }
        let Ok(s) = TcpStream::connect(ip_port.as_str()) else {
            return Err(io::Error::new(io::ErrorKind::Other, "connect fail"));
        };
        println!("TcpCli connect ok");
        Ok(TcpCli { con: s })
    }
}
impl Io for TcpSer {
    fn send(&self, data: &String) -> Option<i32> {
        let n = data.len() as i32;
        let head: [u8; 4] = n.to_le_bytes();
        // println!("[ _send ] data len head:{:?}", head);
        let mut c2 = &self.con;
        let Ok(()) = c2.write_all(&head) else {
            return None;
        };
        let Ok(()) = c2.write_all(data.as_bytes()) else {
            return None;
        };
        Some(n)
    }
    fn recv(&self) -> Option<String> {
        let mut head: [u8; 4] = [0, 0, 0, 0];
        let mut c2 = &self.con;
        let Ok(()) = c2.read_exact(&mut head) else {
            return None;
        };
        let n = i32::from_le_bytes(head);
        // println!("[ _recv ] head to n:{:?}", n);
        let mut data = Vec::new();
        data.resize(n as usize, 0);
        let Ok(()) = c2.read_exact(&mut data) else {
            return None;
        };
        let data_s = String::from_utf8(data);
        match data_s {
            Ok(s) => Some(s),
            Err(e) => {
                println!("recv error:{}", e);
                None
            }
        }
    }
}

impl Io for TcpCli {
    fn send(&self, data: &String) -> Option<i32> {
        let n = data.len() as i32;
        let head: [u8; 4] = n.to_le_bytes();
        // println!("[ _send ] data len head:{:?}", head);
        let mut c2 = &self.con;
        let Ok(()) = c2.write_all(&head) else {
            return None;
        };
        let Ok(()) = c2.write_all(data.as_bytes()) else {
            return None;
        };
        Some(n)
    }
    fn recv(&self) -> Option<String> {
        let mut head: [u8; 4] = [0, 0, 0, 0];
        let mut c2: &TcpStream = &self.con;
        let Ok(()) = c2.read_exact(&mut head) else {
            return None;
        };
        let n = i32::from_le_bytes(head);
        // println!("[ _recv ] head to n:{:?}", n);
        let mut data = Vec::new();
        data.resize(n as usize, 0);
        let Ok(()) = c2.read_exact(&mut data) else {
            return None;
        };
        let data_s = String::from_utf8(data);
        match data_s {
            Ok(s) => Some(s),
            Err(e) => {
                println!("recv error:{}", e);
                None
            }
        }
    }
}
