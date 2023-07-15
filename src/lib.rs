use std::{
    collections::HashMap,
    fmt::format,
    fs,
    io::{BufRead, BufReader, Read, Write},
    net::{IpAddr, Shutdown, TcpListener, TcpStream},
    path::Path,
    sync::Arc,
};
pub struct Server {
    pub addr: IpAddr,
    pub port: u16,
}
impl Server {
    fn routing(req_line: String) -> String {
        let index = String::from("/");

        //FIXME: через match было бы удобнее
        if req_line == index {
            "views/index.html".to_string()
        } else {
            req_line
        }
    }

    pub fn start(self) {
        let socket = TcpListener::bind(format!("{}:{}", self.addr, self.port)).unwrap();
        println!("server run at {}:{}", self.addr, self.port);
    }
}

//Запрос
#[derive(Debug)]
//Client to Server
pub struct HttpRequest {
    pub method: String,
    pub uri: String,
    version: String,
    headers: HashMap<String, String>,
    body: String,
}

impl HttpRequest {
    pub fn handle(
        buffer: Vec<u8>,
        function_dictionary: Arc<
            HashMap<&str, Box<dyn Fn(HttpRequest) -> HttpResponse + Send + Sync>>,
        >,
    ) -> HttpResponse {
        let request_data = String::from_utf8_lossy(&buffer);
        let request = HttpRequest::new(request_data.to_string());

        let base_dir = "./www/";

        if request.method == "GET" {
            let filename: String = Server::routing(request.uri.clone());

            let path = format!("{}/{}", base_dir, filename);

            if Path::new(&path).exists() {
                HttpResponse::ok().file(path)
            } else {
                let uri = request.uri.clone();
                let uri = uri.as_str();
                if function_dictionary.contains_key(&uri) {
                    let f = function_dictionary.get(&uri).unwrap();
                    f(request)
                } else {
                    HttpResponse::not_found()
                }
            }
        } else if request.method == "POST" {
            let mut stream =
                TcpStream::connect("0.0.0.0:8787").expect("Couldn't connect to server...");

            let response = format!("connect...\n");

            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
            stream.shutdown(Shutdown::Both).unwrap();
            // let req = stream.read(&mut [0; 128]).unwrap();

            // println!("{}", req);

            HttpResponse::ok()
        } else {
            HttpResponse::internal_server_error()
        }
    }
    pub fn new(request_data: String) -> Self {
        let r: Vec<&str> = request_data.splitn(2, "\r\n\r\n").collect();
        println!("{:?}", r);
        let request_data = r[0];
        let body = r[0].to_string();

        let r: Vec<&str> = request_data.splitn(2, "\r\n").collect();

        let status_line = r[0];

        let s: Vec<&str> = status_line.split(" ").collect();

        let method = s[0].to_string();
        let uri = s[1].to_string();
        let version = s[2].to_string();

        let header_raw_data = r[0];
        let header_data: Vec<&str> = header_raw_data.split("\r\n").collect();
        let mut headers: HashMap<String, String> = HashMap::new();

        for header in header_data {
            let key_value: Vec<&str> = header.splitn(2, ":").collect();
            headers.insert(key_value[0].to_string(), key_value[0].to_string());
        }

        HttpRequest {
            method,
            uri,
            version,
            headers,
            body,
        }
    }
}

//Server to Client
#[derive(Debug)]
pub struct HttpResponse {
    version: String,
    status_code: usize,
    status_text: String,
    headers: HashMap<String, String>,
    body: String,
}

impl HttpResponse {
    pub fn ok() -> HttpResponse {
        let version = "HTTP/1.1".to_string();
        let status_code = 200;
        let status_text = "OK".to_string();
        let headers: HashMap<String, String> = HashMap::new();
        let body = "".to_string();

        HttpResponse {
            version,
            status_code,
            status_text,
            headers,
            body,
        }
    }
    pub fn not_found() -> HttpResponse {
        let version = "HTTP/1.1".to_string();
        let status_code = 404;
        let status_text = "NOT FOUND".to_string();
        let headers: HashMap<String, String> = HashMap::new();
        let body = "Page could not be found.".to_string();

        HttpResponse {
            version,
            status_code,
            status_text,
            headers,
            body,
        }
    }

    pub fn internal_server_error() -> HttpResponse {
        let version = "HTTP/1.1".to_string();
        let status_code = 500;
        let status_text = "INTERNAL SERVER ERROR".to_string();
        let headers: HashMap<String, String> = HashMap::new();
        let body = "500 - Internal Server Error".to_string();

        HttpResponse {
            version,
            status_code,
            status_text,
            headers,
            body,
        }
    }
    pub fn body(mut self, body: &str) -> Self {
        let content_type = "text/html".to_string();
        let content_length = body.len();

        self.headers
            .insert("Content-Type".to_string(), content_type);
        self.headers
            .insert("Content-Length".to_string(), content_length.to_string());
        self.body = body.to_string();

        self
    }
    //FIXME: now work media files
    pub fn file(mut self, path: String) -> Self {
        let body = fs::read_to_string(&path).unwrap();

        let mime_type = Path::new(&path).extension().unwrap().to_string_lossy();

        let mime_type = if mime_type == "js" {
            "javascript".to_string()
        } else {
            mime_type.to_string()
        };

        let content_type = format!("text/{}", mime_type);
        let content_length = body.len();

        self.headers
            .insert("Content-Type".to_string(), content_type);
        self.headers
            .insert("Content-Length".to_string(), content_length.to_string());
        self.body = body;

        self
    }
    pub fn data(self) -> String {
        if self.headers.is_empty() {
            format!(
                "{version} {status_code} {status_text}\r\n\r\n{body}",
                version = self.version,
                status_code = self.status_code,
                status_text = self.status_text,
                body = self.body
            )
        } else {
            format!(
                "{version} {status_code} {status_text}\r\n{headers}\r\n\r\n{body}",
                version = self.version,
                status_code = self.status_code,
                status_text = self.status_text,
                headers = self.headers(),
                body = self.body
            )
        }
    }

    pub fn headers(&self) -> String {
        let mut headers_string = Vec::new();

        for (key, value) in &self.headers {
            headers_string.push(format!("{}:{}", key, value));
        }

        headers_string.join("\r\n")
    }
}
