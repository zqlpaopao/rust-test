#![allow(unused)]
use std::future::Future;
use std::pin::Pin;

type DynFuture<T> = Pin<Box<dyn Future<Output = T> + 'static>>;
