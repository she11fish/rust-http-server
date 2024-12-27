use std::{io::{BufRead, BufReader, Write}, net::TcpStream};
use std::str;
use std::fs::read_to_string;
use server::HttpStatus;

const ALLOWED_PATHS: [&str; 3] = ["/", "/index", "/index.html"];


pub struct Controller {
    pub stream: TcpStream,
    pub port: &'static str,
}

impl Controller {
    fn read_lines(&self, filename: &str) -> Vec<String> {
        let mut result = Vec::new();
    
        for line in read_to_string(filename).unwrap().lines() {
            result.push(line.to_string())
        }
        result
    }
    fn get_http_info<'a>(&self, test: &'a mut String, buf: &'a mut BufReader<TcpStream>) -> [&'a str; 2] {
        match buf.read_line(test) {
            Ok(_) => "",
            Err(e) => panic!("{e}")
        };
        let strings: Vec<&str> = test.split(" ").collect();
        println!("{:?}", strings);
        let method: &str = strings[0];
        let path: &str = strings[1];
        let ans = [method, path];
        return ans
    }
    
    fn send_message(&mut self, http_status: HttpStatus,  html_filename: &str, path: &str) {
        let port = self.port;
        let header = http_status.status_line() + 
        "Content-Type: text/html; charset=utf-8" +
        &format!("Location: http://localhost:{port}{path}");
        println!("Help");
        let body: String = self.render(html_filename);
        println!("me");
        let response = header.to_string() + &body;
        match self.stream.write_all(response.as_bytes()) {
            Ok(_) => "",
            Err(e) => panic!("{e}")
        };
        self.stream.flush().unwrap();
    }
    fn render(&self, filename: &str) -> String {
        "\n\n".to_string() + &self.read_lines(filename).join("\n")
    }

    fn parse_request(mut s: &mut String, buf: &mut BufReader<TcpStream>) -> String {     
        /*Example usage
        let mut s: String = String::new();
        s = parse_request(&s, &mut buf);
        */   
        loop {
            let size = match buf.read_line(&mut s) {
                Ok(size) => size,
                Err(_) => panic!("Why??")
            };
            if size == 2 {
                break;
            }
        }
        s.to_string()
    }
    
    pub fn route_handler(&mut self) {
        let mut test: String = String::new();
        let mut buf: BufReader<TcpStream> = BufReader::new(self.stream.try_clone().unwrap());
        let request: [&str; 2] = self.get_http_info(&mut test, &mut buf);
        let method = request[0];
        let path: &str = request[1];
        if !ALLOWED_PATHS.contains(&path) {
            self.send_message( HttpStatus::NotFound, "./templates/not_found.html", path);
        }
        if method == "GET" && ["/", "/index", "/index.html"].contains(&path) {
            self.send_message( HttpStatus::OK, "./templates/index.html", path);
        }
    } 
}