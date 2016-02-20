#![feature(plugin_registrar, rustc_private)]
#![allow(plugin_as_library)]

extern crate aster;
#[macro_use] extern crate syntax;
extern crate rustc;
extern crate rustc_plugin;

mod data;
mod expand;

use rustc_plugin::Registry;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("extend", expand::expand);
}
