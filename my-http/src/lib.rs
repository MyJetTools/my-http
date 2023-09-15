#[cfg(feature = "macros")]
pub extern crate my_http_server_swagger as macros;

#[cfg(any(feature = "macros", feature = "controllers"))]
pub extern crate my_http_server_controllers as controllers;

#[cfg(any(feature = "web-sockets", feature = "signalr"))]
pub extern crate my_http_server_web_sockets as web_sockets;

#[cfg(feature = "signalr")]
pub extern crate my_http_signalr_middleware as signalr;

#[cfg(feature = "is-alive")]
pub extern crate my_http_is_alive_middleware as is_alive;

pub extern crate my_http_server_core as core;
