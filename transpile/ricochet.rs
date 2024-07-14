        //! ```cargo
        //! [package]
        //! edition = "2021"
        //! [dependencies]
        //! anyhow = "*"
//! collections = "*"
//! lazy_static = "*"
//! model = "*"
//! random = "*"
//! time = "*"
        //! ```
        

#![allow(clippy::assertions_on_constants)]
#![allow(clippy::bool_comparison)]
#![allow(clippy::collapsible_else_if)]
#![allow(clippy::comparison_to_empty)]
#![allow(clippy::double_parens)] // https://github.com/adsharma/py2many/issues/17
#![allow(clippy::eq_op)]
#![allow(clippy::let_with_type_underscore)]
#![allow(clippy::map_identity)]
#![allow(clippy::needless_return)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::partialeq_to_none)]
#![allow(clippy::print_literal)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::redundant_static_lifetimes)] // https://github.com/adsharma/py2many/issues/266
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::useless_vec)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]


extern crate anyhow;
extern crate collections;
extern crate lazy_static;
extern crate model;
extern crate random;
extern crate time;
use anyhow::Result;
use lazy_static::lazy_static;
use std::collections;
use std::collections::HashMap;


lazy_static! { pub static ref COLORS: HashMap<i32, &'static str> = [(0, "R"), (1, "G"), (2, "B"), (3, "Y")].iter().cloned().collect::<HashMap<_,_>>(); }
lazy_static! { pub static ref DIRECTIONS: HashMap<i32, &'static str> = [(1, "N"), (2, "E"), (4, "S"), (8, "W")].iter().cloned().collect::<HashMap<_,_>>(); }
pub const dll = CDLL("./_ricochet");
pub struct Game {

}

impl Game {
pub const _fields_: &[_; 5] = &[("grid", ((c_uint as i32)*256)), ("moves", ((c_uint as i32)*256)), ("robots", ((c_uint as i32)*4)), ("token", c_uint), ("last", c_uint)]; 
}
pub const CALLBACK_FUNC = CFUNCTYPE(None, c_uint, c_uint, c_uint, c_uint);
pub fn search<T0, T1>(game: T0, callback: T1) -> List {
callback = if callback { CALLBACK_FUNC(callback) } else { None };
let data = game.export();
game = Game{};
game.token = data["token"];
game.last = 0;
for (index, value) in data["grid"].iter().enumerate() {
game.grid[index] = value;
}
for (index, value) in data["robots"].iter().enumerate() {
game.robots[index] = value;
}
let mut robot = data["robot"];
let mut colors = "RGBY".collect::<Vec<_>>();
let (__tmp1, __tmp2) = (colors[robot], colors[0]);
colors[0] = __tmp1;
colors[robot] = __tmp2;
let (__tmp3, __tmp4) = (game.robots[robot], game.robots[0]);
game.robots[0] = __tmp3;
game.robots[robot] = __tmp4;
let path = create_string_buffer(256);
let depth = dll.search(byref(game), path, callback);
let mut result: List = vec![];
for value in path.raw[..depth] {
let color = colors[(((value as i32) >> 4) & 15)];
let direction: &'static str = DIRECTIONS[&((value as i32) & 15)];
result.push((color, direction));
}
return result;
 }

pub fn main() -> Result<()> {




let count: i32 = 0;
let best = (0, 0);
let hist = collections.defaultdict(int);
pub fn callback<T0, T1, T2, T3>(depth: T0, nodes: T1, inner: T2, hits: T3)  {
println!("{}",("Depth: %d, Nodes: %d (%d inner, %d hits)" % (depth, nodes, inner, hits)));
 }

let mut seed: i32 = 0;
loop {
count += 1;
seed += 1;
let start = time.clock();
let mut path: List = search(model.Game(seed));
let mut moves = path.len() as i32;
hist[moves] += 1;
let key = (moves, seed);
if key > best {
best = key;
}
path = path.iter().map(|move_| move_.join("")).collect::<Vec<_>>();
path = path.join(",");
let duration = (time.clock() - start);
println!("{}",("%d %d [%s]" % (seed, moves, path)));
}

 Ok(())}

