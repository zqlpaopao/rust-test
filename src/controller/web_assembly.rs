// #![allow(unused)]
// use sled::Db;
// use std::fmt::Debug;
//
// // https://mp.weixin.qq.com/s/TCVYXB8ssd2UrI9edeJARQ
// //https://mp.weixin.qq.com/s?__biz=Mzk1NzQyOTA1Nw==&mid=2247486216&idx=1&sn=59e939058211e30d805780c4a3c4cb85&chksm=c3df2f26f4a8a630844e0d1aa63a4536eb84134902b5d99ce285bbac323d7e7d6f1d7d108b4e&scene=178&cur_album_id=3760690408318058504#rd
//
// /*************************   键值存储      ********************************/
// // 该模块提供了保存和检索WebAssembly模块的方法。
// // 它使用sled持久化函数名称与其Base64编码的WASM二进制的映射。
// pub struct Storage {
//     db: Db,
// }
//
// impl Storage {
//     pub fn init_with_path(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
//         let db = sled::open(path)?;
//         Ok(Self { db })
//     }
//
//     pub fn init() -> Result<Self, Box<dyn std::error::Error>> {
//         Self::init_with_path(
//             "/Users/zhangqiuli24/Desktop/rust/rust_test/my_test
// /functions_db",
//         )
//     }
//
//     pub fn save_function(&self, name: String, code: String) -> Result<(), sled::Error> {
//         self.db.insert(name, code.as_bytes())?;
//         Ok(())
//     }
//
//     pub fn load_function(&self, name: &str) -> Result<String, sled::Error> {
//         if let Some(code) = self.db.get(name)? {
//             Ok(String::from_utf8(code.to_vec()).unwrap())
//         } else {
//             Err(sled::Error::Io(std::io::Error::new(
//                 std::io::ErrorKind::NotFound,
//                 format!("Function {} not found", name),
//             )))
//         }
//     }
// }
//
// /***************************** 执行引擎 **********************************/
// // 执行引擎将使用wasmtime动态加载和执行WebAssembly模块。
//
// use serde_json::Value;
// // use wasmtime::*;
//
// pub fn execute(
//     code: &str,
//     function_name: &str,
//     inputs: &[Value],
// ) -> Result<Value, Box<dyn std::error::Error>> {
//     let engine = Engine::default();
//     let module = Module::new(&engine, code)?;
//     let mut store = Store::new(&engine, ());
//     let instance = Instance::new(&mut store, &module, &[])?;
//
//     println!("111");
//     // let func = instance.get_func(&mut store,function_name).ok_or_else(
//     //     || format!("Function '{}' not found in module", function_name)
//     // );
//     let func = instance.get_func(&mut store, function_name).unwrap();
//
//     let func_ty = func.ty(&store);
//     println!("444");
//
//     let params: Vec<_> = func_ty.params().collect();
//     let results: Vec<_> = func_ty.results().collect();
//
//     if params.len() != inputs.len() {
//         return Err(format!(
//             "Function '{}' expected {} arguments, but got {}",
//             function_name,
//             params.len(),
//             inputs.len()
//         )
//         .into());
//     }
//     let mut wasm_inputs = Vec::new();
//
//     for (param, input) in params.iter().zip(inputs.iter()) {
//         let value = match (param, input) {
//             (ValType::I32, Value::Number(n)) => Val::I32(n.as_i64().ok_or("Invalid i32")? as i32),
//             _ => return Err(format!("Unsupported parameter type: {:?}", param).into()),
//         };
//         wasm_inputs.push(value);
//     }
//
//     let mut wasm_results = vec![Val::I32(0); results.len()];
//     func.call(&mut store, &wasm_inputs, &mut wasm_results)?;
//
//     if wasm_results.len() > 1 {
//         return Err("Multiple return values are not supported yet".into());
//     }
//
//     let result = match wasm_results.get(0) {
//         Some(Val::I32(v)) => Value::Number((*v).into()),
//         _ => return Err("Unsupported return type".into()),
//     };
//
//     Ok(result)
// }
//
// /******************************* api层 **********************************/
// use std::sync::Arc;
// use warp::Filter;
//
// pub fn server(
//     storage: Arc<Storage>,
// ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
//     let register = warp::post()
//         .and(warp::path("register"))
//         .and(warp::body::json())
//         .and(with_storage(storage.clone()))
//         .and_then(register_function);
//     let invoke = warp::post()
//         .and(warp::path("invoke"))
//         .and(warp::body::json())
//         .and(with_storage(storage.clone()))
//         .and_then(invoke_function);
//     register.or(invoke)
// }
//
// fn with_storage(
//     storage: Arc<Storage>,
// ) -> impl Filter<Extract = (Arc<Storage>,), Error = std::convert::Infallible> + Clone {
//     warp::any().map(move || storage.clone())
// }
//
// async fn register_function(
//     body: Value,
//     storage: Arc<Storage>,
// ) -> Result<impl warp::Reply, warp::Rejection> {
//     println!("{:?}", body.as_str());
//
//     let function_name = body["name"].as_str().ok_or_else(warp::reject::not_found)?;
//     println!("Function name: {}", function_name);
//     let code = body["code"].as_str().ok_or_else(warp::reject::not_found)?;
//     storage
//         .save_function(function_name.to_string(), code.to_string())
//         .map_err(|_err| warp::reject::not_found())?;
//     Ok(warp::reply::json(&format!(
//         "Function {} registered!",
//         function_name
//     )))
// }
//
// async fn invoke_function(
//     body: Value,
//     storage: Arc<Storage>,
// ) -> Result<impl warp::Reply, warp::Rejection> {
//     println!("{:?}", body.as_str());
//     let function_name = body["name"].as_str().ok_or_else(warp::reject::not_found)?;
//     let input = body["input"]
//         .as_array()
//         .ok_or_else(warp::reject::not_found)?;
//     let code = storage
//         .load_function(function_name)
//         .map_err(|_| warp::reject::not_found())?;
//     println!("code name: {}", code);
//     let result = execute(&code, function_name, input).map_err(|err| {
//         println!("Error: {:?}", err);
//         warp::reject::not_found()
//     })?;
//     Ok(warp::reply::json(&result))
// }
//
// pub async fn test_web_assembly() {
//     let storage = Arc::new(Storage::init().expect("Failed to initialize storage"));
//     warp::serve(server(storage))
//         .run(([127, 0, 0, 1], 3030))
//         .await;
// }
