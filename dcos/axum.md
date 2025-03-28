此版本是0.7.7



## 高级特性

- 使用无宏的API实现路由(router)功能
- 使用提取器(extractor)对请求进行声明式的解析
- 简单和可预测的错误处理模式。
- 用最少的模板生成响应。
- 充分利用 `tower` 和 `tower-http` 的中间件、服务和工具的生态系统

axum 与现有框架不同的地方。axum 没有自己的中间件系统，而是使用`tower::Service`。这意味着 axum 可以免费获得超时、跟踪、压缩、授权等功能。它还可以让你与使用 `hyper` 或 `tonic` 编写的应用程序共享中间件。

## 使用示例

注意使用`#[tokio::main]`需要您启用 tokio 的`macros`和`rt-multi-thread`功能或者只是`full`启用所有功能（`cargo add tokio --features macros,rt-multi-thread`）。

先来一个Hello World的入门示例：

```ini
ini

 代码解读
复制代码[dependencies]
axum="0.6.16"
tokio = { version = "1.0", features = ["full"] }
```

添加上面的依赖项后，就可以编码了：

```rust
rust

 代码解读
复制代码use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = route("/", get(handler)); // http://127.0.0.1:3000

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // run it with hyper on localhost:3000
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
// 处理器
async fn handler() -> &'static str {
    "Hello, World!"
}
```

对 `GET/` 的请求响应是 `200 OK`，其中正文是 `Hello, World！`。任何其他请求将导致 `404 Not Found` 响应。

> 注：cargo run 启动后，浏览器里跑一下 [http://127.0.0.1:3000](https://link.juejin.cn?target=http%3A%2F%2F127.0.0.1%3A3000) 或者 curl -X GET [http://127.0.0.1:3000](https://link.juejin.cn?target=http%3A%2F%2F127.0.0.1%3A3000)

## 路由（Routers）

`Router`用于设置哪些路径指向哪些服务，可以使用一个简单的 DSL 来组合多个路由。

细节可以查看 [`Router`](https://link.juejin.cn?target=https%3A%2F%2Fdocs.rs%2Faxum%2Flatest%2Faxum%2Fstruct.Router.html)

### 静态路由

```rust
rust

 代码解读
复制代码#[tokio::main]
async fn main() {
    // our router
    let app = Router::new()
        .route("/", get(root))  // http://127.0.0.1:3000
        .route("/foo", get(get_foo).post(post_foo)) // http://127.0.0.1:3000/foo
        .route("/foo/bar", get(foo_bar)); // http://127.0.0.1:3000/foo/bar

   // run it with hyper on localhost:3000
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// which calls one of these handlers
async fn root() -> String {
    String::from("hello axum")
}
async fn get_foo() -> String {
    String::from("get请求的foo")
}
async fn post_foo() -> String {
    String::from("post请求的foo")
}
async fn foo_bar() -> String {
    String::from("foo:bar")
}
```

注：这里 /foo 同时绑定了GET及POST方法的路由。可以用 crul 命令工具测试一下：

> curl -X GET [http://127.0.0.1:3000/foo](https://link.juejin.cn?target=http%3A%2F%2F127.0.0.1%3A3000%2Ffoo)
>
> curl -X POST [http://127.0.0.1:3000/foo](https://link.juejin.cn?target=http%3A%2F%2F127.0.0.1%3A3000%2Ffoo)

### 捕获路由

路径可以包含与`/:key`任何单个段匹配的段，并将存储在处捕获的值`key`。除无效路径外，捕获的值可以为零长度`//`。

例子：

- `/:key`
- `/users/:id`
- `/users/:id/tweets`

可以使用 提取捕获[`Path`](https://docs.rs/axum/latest/axum/extract/struct.Path.html)。有关更多详细信息，请参阅其文档。

无法创建仅匹配某些类型（如数字或正则表达式）的段。您必须在处理程序中手动处理该问题。

[`MatchedPath`](https://docs.rs/axum/latest/axum/extract/struct.MatchedPath.html)可用于提取匹配的路径而不是实际路径。

#### path

```
use axum::extract::Path;
use axum::routing::{get, post};
use axum::Router;

pub fn routes() -> axum::Router {
     Router::new()
         .route("/", get(index))
         .route("/users/:id", get(users_id))
}

async fn index() -> &'static str {
    "Hello world!"
}



/********************************************** 捕获 **************************************/
async fn users_id(Path(id) : Path<String>) -> String {
 format!("users/{}", id)
}

127.0.0.1:3000/users/zhangsan
users/zhangsan



```

#### MatchedPath



```
use axum::extract::{MatchedPath, Path};
use axum::routing::{get, post};
use axum::Router;

pub fn routes() -> axum::Router {
     Router::new()
         .route("/", get(index))
         .route("/users/:id", get(users_id))
         .route("/users/:id/tweets", get(get_tweets))
}

async fn index() -> &'static str {
    "Hello world!"
}



/********************************************** 捕获 **************************************/
async fn users_id(Path(id) : Path<String>) -> String {
 format!("users/{}", id)
}


async fn get_tweets(pth: MatchedPath) {
    println!("Matched path: {:?}", pth.as_str());
}

127.0.0.1:3000/users/zhangsan/tweets
Matched path: "/users/:id/tweets"
```



### 通配符路由

路径可以以`/*key`匹配所有段的结束，并将存储在处捕获的段`key`。

例子：

- `/*key`
- `/assets/*path`
- `/:id/:repo/*tree`

请注意，`/*key`不匹配空段。因此：

- `/*key`不匹配`/`但匹配`/a`，，`/a/`等等。
- `/x/*key`不匹配`/x`或`/x/`但匹配`/x/a`，`/x/a/`等。





```
use axum::extract::{MatchedPath, Path};
use axum::routing::{get, post};
use axum::Router;

pub fn routes() -> axum::Router {
     Router::new()
         .route("/", get(index))
         .route("/*key", get(handler))
}

async fn index() -> &'static str {
    "Hello world!"
}

/********************************************** 通配符 **************************************/
async fn handler(Path(path): Path<String>) -> String {
    println!("Matched path: {:?}", path);
    path
}

127.0.0.1:3000/*key
Matched path: "*key"

127.0.0.1:3000/assets/*path
Matched path: "assets/*path"

127.0.0.1:3000/:id/:repo/*tree
Matched path: ":id/:repo/*tree"



127.0.0.1:3000/345/kkk/aaa
Matched path: "345/kkk/aaa"
```



### 接收多种方法

要接受同一路由的多种方法，您可以同时添加所有处理程序：

```
use axum::{Router, routing::{get, delete}, extract::Path};

let app = Router::new().route(
    "/",
    get(get_root).post(post_root).delete(delete_root),
);

async fn get_root() {}

async fn post_root() {}

async fn delete_root() {}
```

或者您可以逐个添加：

```
let app = Router::new()
    .route("/", get(get_root))
    .route("/", post(post_root))
    .route("/", delete(delete_root));
```

##### 更多示例

```
use axum::{Router, routing::{get, delete}, extract::Path};

let app = Router::new()
    .route("/", get(root))
    .route("/users", get(list_users).post(create_user))
    .route("/users/:id", get(show_user))
    .route("/api/:version/users/:id/action", delete(do_users_action))
    .route("/assets/*path", get(serve_asset));

async fn root() {}

async fn list_users() {}

async fn create_user() {}

async fn show_user(Path(id): Path<u64>) {}

async fn do_users_action(Path((version, id)): Path<(String, u64)>) {}

async fn serve_asset(Path(path): Path<String>) {}
```

### 路由分组

在某条路径上嵌套一个[`Router`](https://docs.rs/axum/latest/axum/struct.Router.html)。

这使您可以将应用程序分解为更小的部分并将它们组合在一起。

##### 例子

```
use axum::{
    routing::{get, post},
    Router,
};

let user_routes = Router::new().route("/:id", get(|| async {}));

let team_routes = Router::new().route("/", post(|| async {}));

let api_routes = Router::new()
    .nest("/users", user_routes)
    .nest("/teams", team_routes);

let app = Router::new().nest("/api", api_routes);

// Our app now accepts
// - GET /api/users/:id
// - POST /api/teams
```

##### URI 如何变化

请注意，嵌套路由不会看到原始请求 URI，而是会删除匹配的前缀。这对于静态文件服务等服务的正常运行至关重要。[`OriginalUri`](https://docs.rs/axum/latest/axum/extract/struct.OriginalUri.html)如果您需要原始请求 URI，请使用。



### 路由合并merge

```
use axum::{routing::get, Router};
use std::net::SocketAddr;

async fn users_list() -> String {
    "Users list".to_string()
}

async fn users_show(Path(id): Path<String>) -> String {
    format!("User with ID {}", id)
}

async fn teams_list() -> String {
    "Teams list".to_string()
}

#[tokio::main]
async fn main() {
    let user_routes = Router::new()
       .route("/users", get(users_list))
       .route("/users/:id", get(users_show));

    let team_routes = Router::new()
       .route("/teams", get(teams_list));

    let app = Router::new()
       .merge(user_routes)
       .merge(team_routes);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
       .serve(app.into_make_service())
       .await
       .unwrap();
}



127.0.0.1:3000/users
Users list

127.0.0.1:3000/users/123
User with ID 123

127.0.0.1:3000/teams
Teams list


```

在这个示例中，我们定义了三个路由处理函数：`users_list`、`users_show` 和 `teams_list`。然后，我们创建了两个 `Router` 实例，一个用于用户路由，另一个用于团队路由。最后，我们使用 `merge` 方法将这两个路由器合并成一个单一的应用程序路由器。

当你运行这个应用程序时，它将监听端口 3000，并可以处理以下请求：

- GET /users
- GET /users/:id
- GET /teams

每个请求将被路由到相应的处理函数。

### 路由状态

你的代码看起来很好！你正确地使用了 `axum` 的 `Router` 和 `with_state` 方法来创建一个包含嵌套路由器的应用程序，并为每个路由器设置了状态。

在这个示例中，你定义了两个结构体 `InnerState` 和 `OuterState`，它们将用作路由器的状态。然后，你创建了两个路由器：`inner_router` 和 `app`。`inner_router` 处理 `/bar` 路径的 GET 请求，并将 `InnerState` 作为状态传递给 `inner_handler` 处理函数。`app` 路由器处理根路径的 GET 请求，并将 `OuterState` 作为状态传递给 `outer_handler` 处理函数。

你使用 `merge` 方法将 `inner_router` 合并到 `app` 路由器中，这意味着 `/bar` 路径也将被 `app` 路由器处理。

以下是完整的示例代码：

```rust
use axum::{
    Router,
    routing::get,
    extract::State,
};

#[derive(Clone)]
struct InnerState {}

#[derive(Clone)]
struct OuterState {}

async fn inner_handler(state: State<InnerState>) {
    println!("Inner handler called with state: {:?}", state);
}

async fn outer_handler(state: State<OuterState>) {
    println!("Outer handler called with state: {:?}", state);
}

let inner_router = Router::new()
   .route("/bar", get(inner_handler))
   .with_state(InnerState {});

let app = Router::new()
   .route("/", get(outer_handler))
   .merge(inner_router)
   .with_state(OuterState {});

#[tokio::main]
async fn main() {
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
       .serve(app.into_make_service())
       .await
       .unwrap();
}


GET /
GET /bar
Outer handler called with state: State(OuterState)
Inner handler called with state: State(InnerState)

```

当你运行这个应用程序时，它将监听端口 3000，并可以处理以下请求：

- GET /
- GET /bar

每个请求将被路由到相应的处理函数，并且处理函数将接收到正确的状态。

### 处理未定义路径

[`Handler`](https://docs.rs/axum/latest/axum/handler/trait.Handler.html)为路由器添加后备功能。

如果没有路线与传入请求匹配，则会调用此服务。

```
use axum::{
    Router,
    routing::get,
    handler::Handler,
    response::IntoResponse,
    http::{StatusCode, Uri},
};

let app = Router::new()
    .route("/foo", get(|| async { /* ... */ }))
    .fallback(fallback);

async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {uri}"))
}
```

回退仅适用于路由器中任何内容均不匹配的路由。如果处理程序与请求匹配但返回 404，则不会调用回退。请注意，这[`MethodRouter`](https://docs.rs/axum/latest/axum/routing/method_routing/struct.MethodRouter.html)也适用于：如果请求命中有效路径但未[`MethodRouter`](https://docs.rs/axum/latest/axum/routing/method_routing/struct.MethodRouter.html)安装适当的方法处理程序，则不会调用回退（ [`MethodRouter::fallback`](https://docs.rs/axum/latest/axum/routing/method_routing/struct.MethodRouter.html#method.fallback)改为用于此目的）。

##### 处理所有没有其他路由的请求

`Router::new().fallback(...)`如果没有其他路线，则使用接受所有请求（无论路径或方法）并不是最佳选择：

```
use axum::Router;

async fn handler() {}

let app = Router::new().fallback(handler);

let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
axum::serve(listener, app).await.unwrap();
```

直接运行处理程序更快，因为它避免了路由的开销：

```
use axum::handler::HandlerWithoutStateExt;

async fn handler() {}

let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
axum::serve(listener, handler.into_make_service()).await.unwrap();
```







## 中间件



请注意，中间件仅适用于现有路由。因此，您必须先添加路由（和/或回退），然后`layer`再调用。调用后添加的其他路由`layer`将不会添加中间件。

如果您想将中间件添加到单个处理程序，则可以使用 [`MethodRouter::layer`](https://docs.rs/axum/latest/axum/routing/method_routing/struct.MethodRouter.html#method.layer)或[`Handler::layer`](https://docs.rs/axum/latest/axum/handler/trait.Handler.html#method.layer)。

##### 例子

添加[`tower_http::trace::TraceLayer`](https://docs.rs/tower-http/0.6.1/x86_64-unknown-linux-gnu/tower_http/trace/layer/struct.TraceLayer.html)：

```
use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;

let app = Router::new()
    .route("/foo", get(|| async {}))
    .route("/bar", get(|| async {}))
    .layer(TraceLayer::new_for_http());
```

如果您需要编写自己的中间件，请参阅[“编写中间件”](https://docs.rs/axum/latest/axum/middleware/index.html#writing-middleware)了解不同的选项。

如果您只想在某些路线上使用中间件，则可以使用[`Router::merge`](https://docs.rs/axum/latest/axum/struct.Router.html#method.merge)：

```
use axum::{routing::get, Router};
use tower_http::{trace::TraceLayer, compression::CompressionLayer};

let with_tracing = Router::new()
    .route("/foo", get(|| async {}))
    .layer(TraceLayer::new_for_http());

let with_compression = Router::new()
    .route("/bar", get(|| async {}))
    .layer(CompressionLayer::new());

// Merge everything into one `Router`
let app = Router::new()
    .merge(with_tracing)
    .merge(with_compression);
```

##### 多个中间件

建议[`tower::ServiceBuilder`](https://docs.rs/tower/0.5.1/x86_64-unknown-linux-gnu/tower/builder/struct.ServiceBuilder.html)在应用多个中间件时使用。查看[`middleware`](https://docs.rs/axum/latest/axum/middleware/index.html)更多详细信息。

##### 路由后运行

使用此方法添加的中间件将在路由*后*运行，因此不能用于重写请求 URI。有关更多详细信息和解决方法，请参阅[“在中间件中重写请求 URI” 。](https://docs.rs/axum/latest/axum/middleware/index.html#rewriting-request-uri-in-middleware)

##### 错误处理

[`middleware`](https://docs.rs/axum/latest/axum/middleware/index.html)有关错误处理如何影响中间件的详细信息，请参阅。



# 处理器（Handlers）

在 axum 中，处理器（handler）是一个异步异步函数或者异步代码块，它接受零个或多个[“ extractors “](https://link.juejin.cn?target=https%3A%2F%2Fdocs.rs%2Faxum%2Flatest%2Faxum%2Fextract%2Findex.html)作为参数，并返回一些可以转换为一个[“IntoResponse”](https://link.juejin.cn?target=https%3A%2F%2Fdocs.rs%2Faxum%2Flatest%2Faxum%2Fresponse%2Findex.html)的内容。

任何实现的东西都[`IntoResponse`](https://docs.rs/axum/latest/axum/response/trait.IntoResponse.html)可以从处理程序中返回。axum 提供了常见类型的实现：

```
use axum::{
    Json,
    response::{Html, IntoResponse},
    http::{StatusCode, Uri, header::{self, HeaderMap, HeaderName}},
};

// `()` gives an empty response
async fn empty() {}

// String will get a `text/plain; charset=utf-8` content-type
async fn plain_text(uri: Uri) -> String {
    format!("Hi from {}", uri.path())
}

// Bytes will get a `application/octet-stream` content-type
async fn bytes() -> Vec<u8> {
    vec![1, 2, 3, 4]
}

// `Json` will get a `application/json` content-type and work with anything that
// implements `serde::Serialize`
async fn json() -> Json<Vec<String>> {
    Json(vec!["foo".to_owned(), "bar".to_owned()])
}

// `Html` will get a `text/html` content-type
async fn html() -> Html<&'static str> {
    Html("<p>Hello, World!</p>")
}

// `StatusCode` gives an empty response with that status code
async fn status() -> StatusCode {
    StatusCode::NOT_FOUND
}

// `HeaderMap` gives an empty response with some headers
async fn headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(header::SERVER, "axum".parse().unwrap());
    headers
}

// An array of tuples also gives headers
async fn array_headers() -> [(HeaderName, &'static str); 2] {
    [
        (header::SERVER, "axum"),
        (header::CONTENT_TYPE, "text/plain")
    ]
}

// Use `impl IntoResponse` to avoid writing the whole type
async fn impl_trait() -> impl IntoResponse {
    [
        (header::SERVER, "axum"),
        (header::CONTENT_TYPE, "text/plain")
    ]
}
```



此外，您还可以返回元组来从各个部分构建更复杂的响应。

```
use axum::{
    Json,
    response::IntoResponse,
    http::{StatusCode, HeaderMap, Uri, header},
    extract::Extension,
};

// `(StatusCode, impl IntoResponse)` will override the status code of the response
async fn with_status(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("Not Found: {}", uri.path()))
}

// Use `impl IntoResponse` to avoid having to type the whole type
async fn impl_trait(uri: Uri) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, format!("Not Found: {}", uri.path()))
}

// `(HeaderMap, impl IntoResponse)` to add additional headers
async fn with_headers() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/plain".parse().unwrap());
    (headers, "foo")
}

// Or an array of tuples to more easily build the headers
async fn with_array_headers() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "text/plain")], "foo")
}

// Use string keys for custom headers
async fn with_array_headers_custom() -> impl IntoResponse {
    ([("x-custom", "custom")], "foo")
}

// `(StatusCode, headers, impl IntoResponse)` to set status and add headers
// `headers` can be either a `HeaderMap` or an array of tuples
async fn with_status_and_array_headers() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        [(header::CONTENT_TYPE, "text/plain")],
        "foo",
    )
}

// `(Extension<_>, impl IntoResponse)` to set response extensions
async fn with_status_extensions() -> impl IntoResponse {
    (
        Extension(Foo("foo")),
        "foo",
    )
}

#[derive(Clone)]
struct Foo(&'static str);

// Or mix and match all the things
async fn all_the_things(uri: Uri) -> impl IntoResponse {
    let mut header_map = HeaderMap::new();
    if uri.path() == "/" {
        header_map.insert(header::SERVER, "axum".parse().unwrap());
    }

    (
        // set status code
        StatusCode::NOT_FOUND,
        // headers with an array
        [("x-custom", "custom")],
        // some extensions
        Extension(Foo("foo")),
        Extension(Foo("bar")),
        // more headers, built dynamically
        header_map,
        // and finally the body
        "foo",
    )
}
```



在Axum的上下文中，`Extension`是一个类型，它允许你将任意数据附加到响应上。它通常用于从中间件传递信息到处理程序，或者将数据附加到响应上以便其他中间件或最终的响应处理程序可以访问。

在你提供的示例中，`Extension(Foo("foo"))`和`Extension(Foo("bar"))`用于将`Foo`结构体的实例附加到响应上。这些扩展可以随后被其他中间件或最终的响应处理程序访问。

以下是一个在中间件中使用扩展的示例：

```rust
use axum::response::IntoResponse;
use axum::http::HeaderMap;
use std::convert::Infallible;

struct MyMiddleware;

impl axum::middleware::Middleware for MyMiddleware {
    fn transform(self, layer: axum::Layer) -> axum::Layer {
        axum::middleware::LayerFn::new(move |req, next| {
            let res = next.call(req).await;
            if let Some(foo) = res.extensions().get::<Foo>() {
                // 做一些与foo相关的事情
            }
            res
        })
    }
}

// 使用：
let app = axum::Router::new()
  .route("/", axum::routing::get(all_the_things))
  .layer(MyMiddleware);
```

在这个示例中，`MyMiddleware`中间件检查响应是否有一个类型为`Foo`的扩展。如果有，它就可以访问存储在`Foo`实例中的数据。

一般来说，你可以返回如下元组：

- `(StatusCode, impl IntoResponse)`
- `(Parts, impl IntoResponse)`
- `(Response<()>, impl IntoResponse)`
- `(T1, .., Tn, impl IntoResponse)``T1`在哪里`Tn`全部实现[`IntoResponseParts`](https://docs.rs/axum/latest/axum/response/trait.IntoResponseParts.html)。
- `(StatusCode, T1, .., Tn, impl IntoResponse)``T1`在哪里`Tn`全部实现[`IntoResponseParts`](https://docs.rs/axum/latest/axum/response/trait.IntoResponseParts.html)。
- `(Parts, T1, .., Tn, impl IntoResponse)``T1`在哪里`Tn`全部实现[`IntoResponseParts`](https://docs.rs/axum/latest/axum/response/trait.IntoResponseParts.html)。
- `(Response<()>, T1, .., Tn, impl IntoResponse)``T1`在哪里`Tn`全部实现[`IntoResponseParts`](https://docs.rs/axum/latest/axum/response/trait.IntoResponseParts.html)。

## 常规处理

这意味着您不会意外覆盖状态或正文，因为[`IntoResponseParts`](https://docs.rs/axum/latest/axum/response/trait.IntoResponseParts.html)只允许设置标题和扩展。

用于[`Response`](https://docs.rs/axum/latest/axum/response/type.Response.html)更低级别的控制：

```
use axum::{
    Json,
    response::{IntoResponse, Response},
    body::Body,
    http::StatusCode,
};

async fn response() -> Response {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .header("x-foo", "custom header")
        .body(Body::from("not found"))
        .unwrap()
}
```



## 返回不同的响应类型

如果需要返回多种响应类型，并且`Result<T, E>`不合适，您可以调用 `.into_response()`将其转换为`axum::response::Response`：

```
use axum::{
    response::{IntoResponse, Redirect, Response},
    http::StatusCode,
};

async fn handle() -> Response {
    if something() {
        "All good!".into_response()
    } else if something_else() {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong...",
        ).into_response()
    } else {
        Redirect::to("/").into_response()
    }
}

fn something() -> bool {
    // ...
}

fn something_else() -> bool {
    // ...
}
```



## 简化返回书写

您可以使用`impl IntoResponse`处理程序的返回类型来避免输入大类型。例如

```
use axum::http::StatusCode;

async fn handler() -> (StatusCode, [(&'static str, &'static str); 1], &'static str) {
    (StatusCode::OK, [("x-foo", "bar")], "Hello, World!")
}
```

使用起来更容易`impl IntoResponse`：

```
use axum::{http::StatusCode, response::IntoResponse};

async fn impl_into_response() -> impl IntoResponse {
    (StatusCode::OK, [("x-foo", "bar")], "Hello, World!")
}
```

但是也`impl IntoResponse`有一些限制。首先，它只能用于返回单一类型：

[ⓘ](https://docs.rs/axum/latest/axum/response/index.html#)

```
use axum::{http::StatusCode, response::IntoResponse};

async fn handler() -> impl IntoResponse {
    if check_something() {
        StatusCode::NOT_FOUND
    } else {
        "Hello, World!"
    }
}

fn check_something() -> bool {
    // ...
}
```

此函数返回 a`StatusCode`或 a ，`&'static str`这`impl Trait`不允许。

其次，与和 一起`impl IntoResponse`使用时会导致类型推断问题 ：`Result``?`

[ⓘ](https://docs.rs/axum/latest/axum/response/index.html#)

```
use axum::{http::StatusCode, response::IntoResponse};

async fn handler() -> impl IntoResponse {
    create_thing()?;
    Ok(StatusCode::CREATED)
}

fn create_thing() -> Result<(), StatusCode> {
    // ...
}
```

这是因为`?`支持使用[`From`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html)特征转换为不同的错误类型，但它不知道要转换为哪种类型，因为我们只指定`impl IntoResponse`为返回类型。

`Result<impl IntoResponse, impl IntoResponse>`也不总是有效：

[ⓘ](https://docs.rs/axum/latest/axum/response/index.html#)

```
use axum::{http::StatusCode, response::IntoResponse};

async fn handler() -> Result<impl IntoResponse, impl IntoResponse> {
    create_thing()?;
    Ok(StatusCode::CREATED)
}

fn create_thing() -> Result<(), StatusCode> {
    // ...
}
```

解决方案是使用具体的错误类型，例如`Result<impl IntoResponse, StatusCode>`：

```
use axum::{http::StatusCode, response::IntoResponse};

async fn handler() -> Result<impl IntoResponse, StatusCode> {
    create_thing()?;
    Ok(StatusCode::CREATED)
}

fn create_thing() -> Result<(), StatusCode> {
    // ...
}
```

因此，`impl IntoResponse` 除非您熟悉其`impl Trait`工作原理的细节，否则通常不建议使用。

# 提取器

请求可以使用 “提取器(Extractor)” 进行声明式的解析，是一个实现了`FromRequest`或`FromRequestParts`的类型，作用是分离传入请求以获得处理程序所需的部分(比如解析异步函数的参数)，如果请求的URI匹配，就会运行。



```rust
rust

 代码解读
复制代码use axum::extract::{Path, Query, Json};  
use std::collections::HashMap;  
  
// Path路径，eg. /users/<id>  
async fn path(Path(user_id): Path<u32>) {}  
  
// Query参数，eg. /users?id=123&name=jim  
async fn query(Query(params): Query<HashMap<String, String>>) {}

// Json 格式参数，一般用于 POST 请求  
async fn json(Json(payload): Json<serde_json::Value>) {}
```

例如，`Json` 是一个提取器，它消耗请求主体并将其解析为`JSON`:

```rust
rust

 代码解读
复制代码use axum::{ routing::get, Router, extract::Json};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct CreateUser {
    username: String,
}

// curl -H "Content-Type: application/json" -d '{"username":"someName"}' -X POST http://127.0.0.1:3000/users
async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<CreateUser>){
    // `payload` is a `CreateUser`
    // 响应内容为Json格式，状态码是201
    (StatusCode::CREATED, Json(payload))
}
     
#[tokio::main]
async fn main() {
    // our router
    let app = Router::new()
        .route("/users", post(create_user)); // http://127.0.0.1:3000/users

    // run it with hyper on localhost:3000
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

> 注：cargo run 启动后，运行 curl 命令：
>
> curl -H "Content-Type: application/json" -d '{"username":"someName"}' -X POST [http://127.0.0.1:3000/users](https://link.juejin.cn?target=http%3A%2F%2F127.0.0.1%3A3000%2Fusers)

`axum` 提供了许多有用的提取器，例如:

- [`Bytes`](https://link.juejin.cn?target=https%3A%2F%2Fdocs.rs%2Fbytes%2F1.latest%2Fbytes%2Fstruct.Bytes.html), `String`, [`Body`](https://link.juejin.cn?target=https%3A%2F%2Fdocs.rs%2Faxum%2Flatest%2Faxum%2Fbody%2Fstruct.Body.html), 和 [`BodyStream`](https://link.juejin.cn?target=https%3A%2F%2Fdocs.rs%2Faxum%2Flatest%2Faxum%2Fextract%2Fstruct.BodyStream.html) 用于获取请求正文
- [`Method`](https://link.juejin.cn?target=https%3A%2F%2Fdocs.rs%2Fhttp%2Flatest%2Fhttp%2Fmethod%2Fstruct.Method.html), [`HeaderMap`](https://link.juejin.cn?target=https%3A%2F%2Fdocs.rs%2Fhttp%2Flatest%2Fhttp%2Fheader%2Fstruct.HeaderMap.html), 和 [`Uri`](https://link.juejin.cn?target=https%3A%2F%2Fdocs.rs%2Fhttp%2Flatest%2Fhttp%2Furi%2Fstruct.Uri.html) 用于获取请求的特定部分
- [`Form`](https://link.juejin.cn?target=https%3A%2F%2Fdocs.rs%2Faxum%2Flatest%2Faxum%2Fextract%2Fstruct.Form.html), [`Query`](https://link.juejin.cn?target=https%3A%2F%2Fdocs.rs%2Faxum%2Flatest%2Faxum%2Fextract%2Fstruct.Query.html), [`UrlParams`](https://link.juejin.cn?target=https%3A%2F%2Fdocs.rs%2Faxum%2Flatest%2Faxum%2Fextract%2Fstruct.UrlParams.html), 和 [`UrlParamsMap`](https://link.juejin.cn?target=https%3A%2F%2Fdocs.rs%2Faxum%2Flatest%2Faxum%2Fextract%2Fstruct.UrlParamsMap.html) 用于更高级别的请求解析
- [`Extension`](https://link.juejin.cn?target=https%3A%2F%2Ftokio.rs%2Fblog%2F2021-07-announcing-axum) 用于跨处理程序共享状态的扩展
- `Request<hyper::Body>` 如果你想完全控制
- `Result<T, E>` and `Option<T>` 使提取器成为可选

你也可以通过实现 `FromRequest` 来定义你自己的提取器。

> 更多细节可以参看 [`extract`](https://link.juejin.cn?target=https%3A%2F%2Fdocs.rs%2Faxum%2Flatest%2Faxum%2Fextract%2Findex.html) 

# 构建响应（IntoResponse）

处理程序可以返回任何实现了 `IntoResponse` 的东西，它将被自动转换为响应:

```rust
use http::StatusCode;
use axum::response::{Html, Json};
use serde_json::{json, Value};

// We've already seen returning &'static str
async fn text() -> &'static str {
    "Hello, World!"
}

// String works too
async fn string() -> String {
    "Hello, World!".to_string()
}

// Returning a tuple of `StatusCode` and another `IntoResponse` will
// change the status code
async fn not_found() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "not found")
}

// `Html` gives a content-type of `text/html`
async fn html() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

// `Json` gives a content-type of `application/json` and works with any type
// that implements `serde::Serialize`
async fn json() -> Json<Value> {
    Json(json!({ "data": 42 }))
}
```

这意味着在实践中，你很少需要建立你自己的响应。你也可以实现 `IntoResponse` 来创建你自己的特定领域响应。

> 更多细节参看 [`response`](https://link.juejin.cn?target=https%3A%2F%2Fdocs.rs%2Faxum%2Flatest%2Faxum%2Fresponse%2Findex.html) 

# 错误处理（Error handling）

axum旨在提供一个简单且可预测的错误处理模型，这意味着将错误转换为响应很简单，并且可以保证所有错误都得到处理。

```rust
use std::time::Duration;

use axum::{
    body::Body,
    error_handling::{HandleError, HandleErrorLayer},
    http::{Method, Response, StatusCode, Uri},
    response::IntoResponse,
    routing::get,
    BoxError, Router,
};
use tower::ServiceBuilder;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(router_fallible_service()) // 模拟使用 Service的错误处理
        .merge(router_fallible_middleware()) // 模拟使用中间件的错误处理
        .merge(router_fallible_extractor());  // 模拟使用提取器的错误处理  

    let addr = "127.0.0.1:3000";
    println!("listening on {}", addr);
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// 错误处理方式1: 模拟使用 Service的错误处理
fn router_fallible_service() -> Router {
    // 这个 Service 可能出现任何错误
    let some_fallible_service = tower::service_fn(|_req| async {
        thing_that_might_fail().await?;
        Ok::<_, anyhow::Error>(Response::new(Body::empty()))
    });

    Router::new().route_service(
        "/",
        // Service 适配器通过将错误转换为响应来处理错误。
        HandleError::new(some_fallible_service, handle_anyhow_error),
    )
}

// 业务处理逻辑，可能出现失败而抛出 Error
async fn thing_that_might_fail() -> Result<(), anyhow::Error> {
    // 模拟一个错误
    anyhow::bail!("thing_that_might_fail")
}

// 把错误转化为 IntoResponse
async fn handle_anyhow_error(err: anyhow::Error) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Something went wrong: {}", err),
    )
}

// 处理器：模拟超时
async fn handler_timeout() -> impl IntoResponse {
    println!("sleep 3 seconds");
    tokio::time::sleep(Duration::from_secs(3)).await; // 休眠3秒，模拟超时
    format!("Hello Error Handling !!!")
}

// 错误处理方式2 : 用中间件处理错误的路由
fn router_fallible_middleware() -> Router {
    Router::new()
        .route("/fallible_middleware", get(handler_timeout))
        .layer(
            ServiceBuilder::new()
                // `timeout` will produce an error if the handler takes
                // too long so we must handle those
                .layer(HandleErrorLayer::new(handler_timeout_error))
                .timeout(Duration::from_secs(1)),
        )
}

// 用中间件处理错误
async fn handler_timeout_error(err: BoxError) -> (StatusCode, String) {
    if err.is::<tower::timeout::error::Elapsed>() {
        (
            StatusCode::REQUEST_TIMEOUT,
            "Request time too long， Timeout！！！".to_string(),
        )
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {}", err),
        )
    }
}

// 错误处理方式3: 用运行时提取器处理错误的路由
fn router_fallible_extractor() -> Router {
    Router::new()
        .route("/fallible_extractor", get(handler_timeout))
        .layer(
            ServiceBuilder::new()
                // `timeout` will produce an error if the handler takes
                // too long so we must handle those
                .layer(HandleErrorLayer::new(handler_timeout_fallible_extractor))
                .timeout(Duration::from_secs(1)),
        )
}

// 用运行时提取器处理错误
async fn handler_timeout_fallible_extractor(
    // `Method` and `Uri` are extractors so they can be used here
    method: Method,
    uri: Uri,
    // the last argument must be the error itself
    err: BoxError,
) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("`{} {}` failed with {}", method, uri, err),
    )
}
```

> 参见[`error_handling`](https://link.juejin.cn?target=https%3A%2F%2Fdocs.rs%2Faxum%2Flatest%2Faxum%2Ferror_handling%2Findex.html)了解更多关于axum错误处理模型以及如何优雅地处理错误的详细信息。

# 中间件(Middleware)

为 axum 编写中间件有几种不同的方法。详见[`中间件`](https://link.juejin.cn/?target=https%3A%2F%2Fdocs.rs%2Faxum%2Flatest%2Faxum%2Fmiddleware%2Findex.html)

axum 支持来自 `tower` 和 `tower-http` 的中间件。

```ini
[dependencies]
axum = "0.6.16"
tokio = { version = "1.0", features = ["full"] }
tower = { version = "0.4.13", features = ["full"] }
tower-http = { version = "0.4", features = ["fs", "trace", "compression-br"] }
```

添加上面的依赖项后，就可以编码了：

```rust
rust

 代码解读
复制代码use std::net::SocketAddr;
use axum::{routing::get, Router};
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};

#[tokio::main]
async fn main() {
    let middleware_stack = ServiceBuilder::new()
        // add high level tracing of requests and responses
        .layer(TraceLayer::new_for_http())
        // compression responses
        .layer(CompressionLayer::new())
        // convert the `ServiceBuilder` into a `tower::Layer`;
        .into_inner();

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(middleware_stack);

    // run it with hyper on localhost:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

这个功能很关键，因为它允许我们只写一次中间件，并在不同的应用中分享它们。例如，`axum` 不需要提供自己的 `tracing/logging` 中间件，可以直接使用来自 `tower-http` 的 `TraceLayer` 。同样的中间件也可以用于用 tonic 制作的客户端或服务器路由到任何 `tower::Service`

`axum` 也可以将请求路由到任何 tower 服务。可以是你用 `service_fn` 编写的服务，也可以是来自其他 crate 的东西，比如来自 `tower-http` 的`ServeFile`：

```
use axum::{
    body::Body, http::Request, response::Response, routing::get_service, Router,
};
use std::{convert::Infallible, net::SocketAddr};
use tower::service_fn;
use tower_http::services::ServeFile;

#[tokio::main]
async fn main() {
    let app = Router::new()
        // GET `/static/Cargo.toml` goes to a service from tower-http
        .route(
            "/static",
            get_service(ServeFile::new("Cargo.toml")),
        )
        .route(
            // Any request to `/` goes to a some `Service`
            "/",
            get_service(service_fn(|_: Request<Body>| async {
                let res = Response::new(Body::from("Hi from `GET /`"));
                Ok::<_, Infallible>(res)
            })),
        );

    // run it with hyper on localhost:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

作者：RustCoder
链接：https://juejin.cn/post/7233196954401636410
来源：稀土掘金
著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
```



## 中间件 记录请求体和响应内容

```
https://github.com/tokio-rs/axum/blob/main/examples/print-request-response/src/main.rs
```



## trace的中间件

```
 TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    println!("{:?} {}", request.method(), request.uri().path());
                    // Log the matched route's path (with placeholders not filled in).
                    // Use request.uri() or OriginalUri if you want the real path.
                    // let matched_path = request
                    //     .extensions()
                    //     .get::<MatchedPath>()
                    //     .map(MatchedPath::as_str);
                    //
                    // info!(
                    //     "http_request",
                    //     method = request.method(),
                    //     matched_path,
                    //     some_other_field = tracing::field::Empty,
                    // )
                })
                .on_request(|request: &Request<_>, span: &Span| {
                    println!("on_request {},{}",request,span);
                    // You can use `_span.record("some_other_field", value)` in one of these
                    // closures to attach a value to the initially empty field in the info_span
                    // created above.
                })
                .on_response(|response: &Response<_>,latency: Duration, span: &Span| {
                    println!("on_response {},{:?},{}",response,latency,span);

                })
                .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {
                    // ...
                })
                .on_eos(
                    |_trailers: Option<&HeaderMap>, _stream_duration: Duration, _span: &Span| {
                        // ...
                    },
                )
                .on_failure(
                    |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                        // ...
                    },
                ),
```

# 与处理器共享状态（Sharing state with handlers）

在处理程序之间共享某些状态是很常见的。例如，可能需要共享到其他服务的数据库连接或客户端池。

最常见的三种方法是:

- 使用状态提取器：[`State`](https://link.juejin.cn?target=https%3A%2F%2Fdocs.rs%2Faxum%2Flatest%2Faxum%2Fextract%2Fstruct.State.html)
- 使用请求扩展提取器：[`Extension`](https://link.juejin.cn?target=https%3A%2F%2Fdocs.rs%2Faxum%2Flatest%2Faxum%2Fstruct.Extension.html)
- 使用闭包捕获：`Closure`

```rust
rust

 代码解读
复制代码use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
    Extension, Router,
};
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};

// 共享状态结构体
#[derive(Debug)]
struct AppState {
    // ...
    state: AtomicUsize,
}

// 方法1: 使用 State 状态提取器
async fn handler_as_state_extractor(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    // ...
    state.state.fetch_add(1, Ordering::SeqCst); //请求一次 state 的值递增1
    format!("State extract shared_state: {:?}", state)
}

// 方法2: 使用 Extension 请求扩展提取器
// 这种方法的缺点是，如果尝试提取一个不存在的扩展，可能是因为忘记添加中间件，
// 或者因为提取了错误的类型，那么将得到运行时错误(特别是500 Internal Server Error 响应)。
async fn handler_as_extension_extractor(Extension(state): Extension<Arc<AppState>>) -> impl IntoResponse {
    // ...
    state.state.fetch_add(1, Ordering::SeqCst); //请求一次 state 的值递增1
    format!("Extension extract shared_state: {:?}", state)
}

// 方法3: 使用闭包捕获（closure captures）直接传递给处理器
async fn get_user(Path(user_id): Path<String>, state: Arc<AppState>) -> impl IntoResponse {
    // ...
    state.state.fetch_add(1, Ordering::SeqCst); //请求一次 state 的值递增1
    format!("closure captures shared_state: {:?}", state)
}

#[tokio::main]
async fn main() {
    // 处理器共享状态（Sharing state with handlers）
    let shared_state = Arc::new(AppState { state: 0.into()/* ... */ });
    let shared_state_extension = Arc::clone(&shared_state);
    let shared_state_closure = Arc::clone(&shared_state);

    let app = Router::new()
        .route("/state", get(handler_as_state_extractor)) // 1.使用State提取器
        .with_state(shared_state)
        .route("/extension", get(handler_as_extension_extractor)) // 2.使用Extension提取器
        .layer(Extension(shared_state_extension))
        .route(
            "/users/:id",
            get({
                move |path| get_user(path, shared_state_closure)  // 3.使用闭包捕获直接传递给处理器
            }),
        );

    let addr = "127.0.0.1:3000";
    println!("listening on {}", addr);
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

注：可以用浏览器跑一下或者 crul 命令工具测试一下(交替请求)，看下state的是否在共享基础上递增

> curl -X GET [http://127.0.0.1:3000/state](https://link.juejin.cn?target=http%3A%2F%2F127.0.0.1%3A3000%2Fstate)
>
> curl -X GET [http://127.0.0.1:3000/extension](https://link.juejin.cn?target=http%3A%2F%2F127.0.0.1%3A3000%2Fextension)
>
> curl -X GET [http://127.0.0.1:3000/users/111](https://link.juejin.cn?target=http%3A%2F%2F127.0.0.1%3A3000%2Fusers%2F111)

## 总结

axum 是一个易于使用，功能强大的 Web 框架，旨在充分利用 Tokio 的生态系统，使用无宏 API 实现了路由功能，基于 [hyper](https://link.juejin.cn?target=https%3A%2F%2Fcrates.io%2Fcrates%2Fhyper) 构建的，中间件基于 tower 和 tower-http 生态实现，可利用其中中间件、服务以及实用程序。支持 WebSocket 和其他协议，以及异步 I/O。

- axum 的中间件是直接使用 tower 的抽象，这样的好处就是:
  1. 使用了统一 的 Service 和 Layer 抽象标准，方便大家来繁荣生态
  2. 复用和充分利用 tokio / hyper/ tonic 生态，潜力很大
- axum 的路由机制并没有使用像 rocket那样的属性宏，而是提供了简单的 DSL （链式调用）。路由是基于迭代和正则表达式来匹配的，所以路由性能和 actix-web 差不多。
- 也提供了方便的提取器 ，只要实现 FromRequest 就是一个提取器，实现起来也非常方便。

总之，Axum 是 Rust 在 Web 开发领域的一个里程碑，它强势带动了 tokio/tower 生态，潜力很大。

## 























































































































































