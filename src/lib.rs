#![warn(clippy::all, rust_2018_idioms)]
pub mod app;
pub mod grid;
pub mod component;
pub mod python_component;
pub mod eval_expr_component;
pub mod component_window;
pub mod connection;
pub mod toolchain;
pub mod execution_process;
pub use app::TemplateApp;
