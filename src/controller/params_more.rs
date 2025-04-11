#![allow(unused)]
// https://mp.weixin.qq.com/s/BvrUxLlb7rUuQUK7MUo5PA

#[derive(Debug, Clone)]
struct Request {
    path: String,
    method: String,
    param: String,
}

trait FromRequest {
    fn from_request(req: &Request) -> Self;
}

pub struct Path(pub String);

impl FromRequest for Path {
    fn from_request(req: &Request) -> Self {
        Self(req.path.clone())
    }
}

pub struct Method(pub String);

impl FromRequest for Method {
    fn from_request(req: &Request) -> Self {
        Self(req.method.clone())
    }
}

trait Handler<T> {
    fn call(self, req: Request);
}

impl<T, F> Handler<T> for F
where
    F: Fn(T),
    T: FromRequest,
{
    fn call(self, req: Request) {
        self(T::from_request(&req))
    }
}

/// 如果只实现了一种参数的闭包，这里只能存在一个参数。
fn hello_world(Path(p): Path) {
    dbg!(p);
}

// ************************** 多参数 **********************
impl<T1, T2, F> Handler<(T1, T2)> for F
where
    F: Fn(T1, T2),
    T1: FromRequest,
    T2: FromRequest,
{
    fn call(self, req: Request) {
        self(T1::from_request(&req), T2::from_request(&req))
    }
}

fn hello_world2(Path(p): Path, Method(m): Method) {
    dbg!(p);
    dbg!(m);
}

fn server<R, H>(req: Request, handler: H)
where
    H: Handler<R>,
{
    handler.call(req);
}

fn main() {
    let req = Request {
        path: "/".to_string(),
        method: "GET".to_string(),
        param: "".to_string(),
    };

    server(req.clone(), hello_world);
    server(req.clone(), hello_world2);
}
