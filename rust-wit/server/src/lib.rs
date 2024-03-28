#[allow(warnings)]
mod bindings;

pub use bindings::wasi::http::types::{
    Fields, IncomingRequest, OutgoingBody, OutgoingResponse, ResponseOutparam,
};
use bindings::{exports::bachelor::server::server_helper::Guest, wasi::http::types::Method};

struct Component;

bindings::export!(Component with_types_in bindings);

impl bindings::exports::bachelor::server::server_helper::Guest for Component {
    fn hello_world(outparam: ResponseOutparam) {
        let content = "Hello, wasi:http/proxy world!\n".to_string();
        Self::send_response(content, outparam);
    }

    fn unknown_request(outparam: ResponseOutparam) {
        let content = "Root".to_string();
        Self::send_response(content, outparam);
    }

    fn send_response(content: String, outparam: ResponseOutparam) {
        let hdrs = Fields::new();
        let resp = OutgoingResponse::new(hdrs);
        let body = resp.body().expect("outgoing response");

        ResponseOutparam::set(outparam, Ok(resp));

        let out = body.write().expect("outgoing stream");
        out.blocking_write_and_flush(content.as_bytes())
            .expect("writing response");

        drop(out);
        OutgoingBody::finish(body, None).unwrap();
    }

    fn print_headers(request: IncomingRequest) {
        print!("Received Request with Header:");
        for (key, value) in request.headers().entries().iter() {
            let value_str = String::from_utf8_lossy(value);
            println!("{}: {}", key, value_str);
        }
    }

    fn parse_path(
        path: String,
        query: Option<String>,
        request: IncomingRequest,
        outparam: ResponseOutparam,
    ) {
        Self::print_headers(request);
        match path.as_str() {
            "/hello_world" => Component::hello_world(outparam),
            _ => Component::unknown_request(outparam),
        }
    }

    fn dht11(query: Option<String>, request: IncomingRequest, outparam: ResponseOutparam) {
        let meth = request.method();
        match meth {
            Method::Get => Self::dht11_get(outparam),
            Method::Post => Self::dht11_post(query, outparam),
            Method::Delete => Self::dht11_delete(query, outparam),
            _ => {}
        }
    }

    fn dht11_get(outparam: ResponseOutparam) {
        // Perform sql query for dht11 values

        // Send query results back
    }

    fn dht11_post(query: Option<String>, outparam: ResponseOutparam) {
        // Insert query into sql database

        // Respond if insertion was successful
    }

    fn dht11_delete(query: Option<String>, outparam: ResponseOutparam) {
        // Delete entry specified in query from database

        // Respond if deletion was successful
    }
}

impl bindings::exports::wasi::http::incoming_handler::Guest for Component {
    /// Say hello!
    fn handle(request: IncomingRequest, outparam: ResponseOutparam) {
        let path = request.path_with_query().unwrap_or_default();

        // Splitting at the first occurrence of '?'
        if let Some((path_str, query)) = path.split_once('?') {
            Component::parse_path(
                path_str.to_string(),
                Some(query.to_string()),
                request,
                outparam,
            );
        } else {
            Component::parse_path(path, None, request, outparam);
        }
    }
}
