//! A high level 2d renderer that is heavily based on amethyst_renderer

#![crate_name = "look_renderer"];
#![crate_type = "lib"];

extern crate fnv;
extern crate specs;
extern crate cgmath;
extern crate glutin;

#[macro_use]
extern crate gfx;
#[macro_use]
extern crate mopa;

use fnv::FnvHashMap as HashMap;
use specs::{Component, VecStorage};
use std::any::TypeId;
