use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

pub struct Routes<'a> {
    pub routes_map: HashMap<String, Rc<Handle<'a>>>,
}

impl Display for Routes<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Routes: [")?;

        let mut first = true;
        for (key, value) in &self.routes_map {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", key, value)?;
            first = false;
        }

        write!(f, "]")
    }
}

pub struct Handle<'a> {
    pub code: ResponseCode,
    pub file: &'a str,
}

impl Display for Handle<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code.as_str(), self.file)
    }
}

impl Handle<'_> {
    pub fn new(code: ResponseCode, file: &str) -> Handle {
        return Handle {
            code,
            file: file.clone(),
        };
    }
}

pub enum ResponseCode {
    Ok,
    NotFound,
    Failure,
}

impl ResponseCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            ResponseCode::Ok => "HTTP/1.1 200 OK",
            ResponseCode::NotFound => "HTTP/1.1 404 NOT FOUND",
            ResponseCode::Failure => "HTTP/1.1 500 SERVER FAIL",
        }
    }
}

impl<'a> Routes<'a> {
    pub fn add(&mut self, route: String, handle: Rc<Handle<'a>>) {
        self.routes_map.insert(route, handle);
    }

    pub fn new() -> Routes<'a> {
        return Routes {
            routes_map: HashMap::new(),
        };
    }

    pub fn handle_request(&self, route: &str) -> Option<&Handle<'a>> {
        // Check if the route is present in the routes_map
        if let Some(handle) = self.routes_map.get(route) {
            Some(handle.as_ref())
        } else {
            None
        }
    }
}
