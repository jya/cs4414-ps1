//
// zhttpto.rs
//
// University of Virginia - cs4414 Fall 2013
// Weilin Xu and David Evans
// Version 0.1

extern mod extra;

use extra::uv;
use extra::{net_ip, net_tcp};
use std::str;
use std::io;
use std::path;

static BACKLOG: uint = 5;
static PORT:    uint = 4414;
static IPV4_LOOPBACK: &'static str = "127.0.0.1";
static mut count : uint = 0;

fn new_connection_callback(new_conn :net_tcp::TcpNewConnection, _killch: std::comm::SharedChan<Option<extra::net_tcp::TcpErrData>>)
{
    do spawn {
        let accept_result = extra::net_tcp::accept(new_conn);
	let mut file_contents: ~[~str] = ~[];
	let mut file_result: ~str = ~"";
        match accept_result {
            Err(err) => {
               println(fmt!("Connection error: %?", err));
            },  
            Ok(sock) => {
                let peer_addr: ~str = net_ip::format_addr(&sock.get_peer_addr());
                println(fmt!("Received connection from: %s", peer_addr));
		unsafe{ count += 1; }
                
                let read_result = net_tcp::read(&sock, 0u);
                match read_result {
                    Err(err) => {
                        println(fmt!("Receive error: %?", err));
                    },
                    Ok(bytes) => {
			let request_str = str::from_bytes(bytes.slice(0, bytes.len() - 1));
			for request_str.any_line_iter().advance() |line| {
				if(line.starts_with("GET")) {
					let loc = line.find_str("HTTP/1.1").get();
					let file = line.slice(5, loc-1).to_owned();
					println(fmt!("%?", file));
					println(file);
					if(file.len() > 1){
						file_contents = load_file(file);
					}
				}
			}
			
			for file_contents.iter().advance() |line| {
				println(line.to_str());
				file_result = file_result + line.to_str();
				file_result = file_result + " \
							      ";
			}
			println(file_result);

                        println(fmt!("Request received:\n%s", request_str));
			unsafe{ println(fmt!("Number of requests: %u", count)); }
                        let mut response: ~str = ~
                            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                             <doctype !html><html><head><title>Hello, Rust!</title>
                             <style>body { background-color: #111; color: #FFEEAA }
                                    h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}
                             </style></head>
                             <body>
                             <h1>Greetings, Rusty!</h1>";
			unsafe{
			        response = response + fmt!("
					   <h2>Number of requests: %u</h2>
					   <p>%s</p>
				 	   </body></html>\r\n", count, file_result); 
			}
                        net_tcp::write(&sock, response.as_bytes_with_null_consume());
                    },
                };
            }
        }
    };
}

fn load_file(pathname : ~str) -> ~[~str] {
	let filereader : Result<@Reader, ~str> = io::file_reader(~path::Path(pathname));
	match filereader { 
		Ok(reader) => reader.read_lines(),
		Err(msg) => fail!("Cannot open file: " + msg),
	}
}

fn main() {
    net_tcp::listen(net_ip::v4::parse_addr(IPV4_LOOPBACK), PORT, BACKLOG,
                    &uv::global_loop::get(),
                    |_chan| { println(fmt!("Listening on tcp port %u ...", PORT)); },
                    new_connection_callback);
}
