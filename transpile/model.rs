        //! ```cargo
        //! [package]
        //! edition = "2021"
        //! [dependencies]
        //! itertools = "*"
//! lazy_static = "*"
//! random = "*"
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


extern crate itertools;
extern crate lazy_static;
extern crate random;
use lazy_static::lazy_static;
use std::collections;
use std::collections::HashMap;



pub const NORTH: &'static str = "N";
pub const EAST: &'static str = "E";
pub const SOUTH: &'static str = "S";
pub const WEST: &'static str = "W";
pub const DIRECTIONS: &[&str; 4] = &[NORTH, EAST, SOUTH, WEST];
lazy_static! { pub static ref REVERSE: HashMap<&'static str, &'static str> = [(NORTH, SOUTH), (EAST, WEST), (SOUTH, NORTH), (WEST, EAST)].iter().cloned().collect::<HashMap<_,_>>(); }
lazy_static! { pub static ref OFFSET: HashMap<&'static str, i32> = [(NORTH, -16), (EAST, 1), (SOUTH, 16), (WEST, -1)].iter().cloned().collect::<HashMap<_,_>>(); }
pub const M_NORTH: i32 = 1;
pub const M_EAST: i32 = 2;
pub const M_SOUTH: i32 = 4;
pub const M_WEST: i32 = 8;
pub const M_ROBOT: i32 = 16;
lazy_static! { pub static ref M_LOOKUP: HashMap<&'static str, i32> = [(NORTH, M_NORTH), (EAST, M_EAST), (SOUTH, M_SOUTH), (WEST, M_WEST)].iter().cloned().collect::<HashMap<_,_>>(); }
pub const RED: &'static str = "R";
pub const GREEN: &'static str = "G";
pub const BLUE: &'static str = "B";
pub const YELLOW: &'static str = "Y";
pub const COLORS: &[&str; 4] = &[RED, GREEN, BLUE, YELLOW];
pub const CIRCLE: &'static str = "C";
pub const TRIANGLE: &'static str = "T";
pub const SQUARE: &'static str = "Q";
pub const HEXAGON: &'static str = "H";
pub const SHAPES: &[&str; 4] = &[CIRCLE, TRIANGLE, SQUARE, HEXAGON];
pub const TOKENS = itertools.product(COLORS, SHAPES).iter().map(|token| token.join("")).collect::<Vec<_>>();
pub const QUAD_1A: &'static str = "NW,N,N,N,NE,NW,N,N,W,S,X,X,X,X,SEYH,W,WE,NWGT,X,X,X,X,N,X,W,X,X,X,X,X,X,X,W,X,X,X,X,X,S,X,SW,X,X,X,X,X,NEBQ,W,NW,X,E,SWRC,X,X,X,S,W,X,X,N,X,X,E,NW";
pub const QUAD_1B: &'static str = "NW,NE,NW,N,NS,N,N,N,W,S,X,E,NWRC,X,X,X,W,NEGT,W,X,X,X,X,X,W,X,X,X,X,X,SEYH,W,W,X,X,X,X,X,N,X,SW,X,X,X,X,X,X,X,NW,X,E,SWBQ,X,X,X,S,W,X,X,N,X,X,E,NW";
pub const QUAD_2A: &'static str = "NW,N,N,NE,NW,N,N,N,W,X,X,X,X,E,SWBC,X,W,S,X,X,X,X,N,X,W,NEYT,W,X,X,S,X,X,W,X,X,X,E,NWGQ,X,X,W,X,SERH,W,X,X,X,X,SW,X,N,X,X,X,X,S,NW,X,X,X,X,X,E,NW";
pub const QUAD_2B: &'static str = "NW,N,N,N,NE,NW,N,N,W,X,SERH,W,X,X,X,X,W,X,N,X,X,X,X,X,WE,SWGQ,X,X,X,X,S,X,SW,N,X,X,X,E,NWYT,X,NW,X,X,X,X,S,X,X,W,X,X,X,X,NEBC,W,S,W,X,X,X,X,X,E,NW";
pub const QUAD_3A: &'static str = "NW,N,N,NE,NW,N,N,N,W,X,X,X,X,SEGH,W,X,WE,SWRQ,X,X,X,N,X,X,SW,N,X,X,X,X,S,X,NW,X,X,X,X,E,NWYC,X,W,X,S,X,X,X,X,X,W,X,NEBT,W,X,X,X,S,W,X,X,X,X,X,E,NW";
pub const QUAD_3B: &'static str = "NW,N,NS,N,NE,NW,N,N,W,E,NWYC,X,X,X,X,X,W,X,X,X,X,X,X,X,W,X,X,X,X,E,SWBT,X,SW,X,X,X,S,X,N,X,NW,X,X,X,NERQ,W,X,X,W,SEGH,W,X,X,X,X,S,W,N,X,X,X,X,E,NW";
pub const QUAD_4A: &'static str = "NW,N,N,NE,NW,N,N,N,W,X,X,X,X,X,X,X,W,X,X,X,X,SEBH,W,X,W,X,S,X,X,N,X,X,SW,X,NEGC,W,X,X,X,X,NW,S,X,X,X,X,E,SWRT,WE,NWYQ,X,X,X,X,X,NS,W,X,X,X,X,X,E,NW";
pub const QUAD_4B: &'static str = "NW,N,N,NE,NW,N,N,N,WE,SWRT,X,X,X,X,S,X,W,N,X,X,X,X,NEGC,W,W,X,X,X,X,X,X,X,W,X,SEBH,W,X,X,X,S,SW,X,N,X,X,X,E,NWYQ,NW,X,X,X,X,X,X,S,W,X,X,X,X,X,E,NW";
pub const QUADS: &[_; 4] = &[(QUAD_1A, QUAD_1B), (QUAD_2A, QUAD_2B), (QUAD_3A, QUAD_3B), (QUAD_4A, QUAD_4B)];
pub const ROTATE_QUAD: &[i32; 64] = &[56, 48, 40, 32, 24, 16, 8, 0, 57, 49, 41, 33, 25, 17, 9, 1, 58, 50, 42, 34, 26, 18, 10, 2, 59, 51, 43, 35, 27, 19, 11, 3, 60, 52, 44, 36, 28, 20, 12, 4, 61, 53, 45, 37, 29, 21, 13, 5, 62, 54, 46, 38, 30, 22, 14, 6, 63, 55, 47, 39, 31, 23, 15, 7];
lazy_static! { pub static ref ROTATE_WALL: HashMap<&'static str, &'static str> = [(NORTH, EAST), (EAST, SOUTH), (SOUTH, WEST), (WEST, NORTH)].iter().cloned().collect::<HashMap<_,_>>(); }
pub fn idx<T0, T1, T2, RT>(x: T0, y: T1, size: T2) -> RT {
return ((y*size) + x);
 }

pub fn xy<T0, T1, RT>(index: T0, size: T1) -> RT {
let x = (index % size);
let y = (index / size);
return (x, y);
 }

pub fn rotate_quad<T0, T1, RT>(data: T0, times: T1) -> RT {
for i in (0..times) {
let mut result = ROTATE_QUAD.iter().map(|index| data[index]).collect::<Vec<_>>();
result = result.iter().map(|x| x.iter().map(|c| ROTATE_WALL.get(c, c)).collect::<Vec<_>>().join("")).collect::<Vec<_>>();
data = result;
}
return data;
 }

pub fn create_grid<T0, RT>(quads: T0) -> RT {
if Some(quads) == None {
quads = QUADS.iter().map(|pair| random.choice(pair)).collect::<Vec<_>>();
random.shuffle(quads);
}
quads = quads.iter().map(|quad| quad.split(",")).collect::<Vec<_>>();
quads = vec![0, 1, 3, 2].iter().map(|i| rotate_quad(quads[i], i)).collect::<Vec<_>>();
let mut result = (0..(16*16)).map(|i| None).collect::<Vec<_>>();
for (i, quad) in quads.iter().enumerate() {
let (dx, dy) = xy(i, 2);
for (j, data) in quad.iter().enumerate() {
let (x, y) = xy(j, 8);
x += ((dx as i32)*8);
y += ((dy as i32)*8);
let mut index = idx(x, y);
result[index] = data;
}
}
return result;
 }

pub fn to_mask<T0>(cell: T0) -> i32 {
let mut result: i32 = 0;
for (letter, mask) in M_LOOKUP.items().collect::<Vec<_>>() {
if cell.iter().any(|&x| x == letter) {
result |= mask;
}
}
return result;
 }

pub struct Game {
pub grid: _,
pub robots: Dict,
pub token: bool,
pub moves: i32,
pub last: _,
}

impl Game {
pub fn hardest() -> Game {
pub const quads: &[&str; 4] = &[QUAD_2B, QUAD_4B, QUAD_3B, QUAD_1B];
pub const robots: &[i32; 4] = &[226, 48, 43, 18];
pub const token: &'static str = "BT";
return Game{quads: quads, robots: robots, token: token};
 }

pub fn __init__<T0, T1, T2, T3>(&self, seed: T0, quads: T1, robots: T2, token: T3)  {
if Some(seed) != None {
random.seed(seed);
}
self.grid = create_grid(quads);
if Some(robots) == None {
self.robots = self.place_robots();
} else {
self.robots = dict(zip(COLORS, robots).collect::<Vec<_>>());
}
self.token = token||random.choice(TOKENS);
self.moves = 0;
self.last = None;
 }

pub fn place_robots(&self) -> Dict {
lazy_static! { pub static ref result: HashMap<Dict> = HashMap::new(); }
pub const used = set();
for color in COLORS {
loop {
pub const index = random.randint(0, 255);
if (119, 120, 135, 136).iter().any(|&x| x == index) {
continue;
}
if TOKENS.iter().any(|&x| x == self.grid[index][-2..]) {
continue;
}
if used.iter().any(|&x| x == index) {
continue;
}
result[*color] = index;
used.add(index);
break;
}

}
return result;
 }

pub fn get_robot<T0, RT>(&self, index: T0) -> RT {
for (color, position) in self.robots.items() {
if position == index {
return color;
}
}
return None;
 }

pub fn can_move<T0, T1>(&self, color: T0, direction: T1) -> bool {
if self.last == (color, REVERSE[direction]) {
return false;
}
pub const index = self.robots[color];
if self.grid[index].iter().any(|&x| x == direction) {
return false;
}
pub const new_index: i32 = ((index as i32) + OFFSET[direction]);
if iter(self.robots.values()).iter().any(|&x| x == new_index) {
return false;
}
return true;
 }

pub fn compute_move<T0, T1, RT>(&self, color: T0, direction: T1) -> RT {
pub static index = self.robots[color];
pub const robots = self.robots.values().collect::<Vec<_>>();
loop {
if self.grid[index].iter().any(|&x| x == direction) {
break;
}
pub const new_index: i32 = ((index as i32) + OFFSET[direction]);
if robots.iter().any(|&x| x == new_index) {
break;
}
index = new_index;
}

return index;
 }

pub fn do_move<T0, T1, RT>(&self, color: T0, direction: T1) -> RT {
pub const start = self.robots[color];
pub const last = self.last;
if last == (color, REVERSE[direction]) {
raise!(Exception); //unsupported
}
pub const end = self.compute_move(color, direction);
if start == end {
raise!(Exception); //unsupported
}
self.moves += 1;
self.robots[color] = end;
self.last = (color, direction);
return (color, start, last);
 }

pub fn undo_move<T0>(&self, data: T0)  {
pub const (color, start, last) = data;
self.moves -= 1;
self.robots[color] = start;
self.last = last;
 }

pub fn get_moves<T0>(&self, colors: T0) -> List {
pub static result: &[_; 0] = &[];
colors = colors||COLORS;
for color in colors {
for direction in DIRECTIONS {
if self.can_move(color, direction) {
result.push((color, direction));
}
}
}
return result;
 }

pub fn over(&self) -> bool {
pub const color = self.token[0];
return self.grid[self.robots[color]].iter().any(|&x| x == self.token);
 }

pub fn key<RT>(&self) -> RT {
return tuple(self.robots.values());
 }

pub fn search<RT>(&self) -> RT {
pub static max_depth: i32 = 1;
loop {
pub const result = self._search(vec![], set(), 0, max_depth);
if Some(result) != None {
return result;
}
max_depth += 1;
}

 }

pub fn _search<T0, T1, T2, T3, RT>(&self, path: T0, memo: T1, depth: T2, max_depth: T3) -> RT {
if self.over() {
return path.collect::<Vec<_>>();
}
if depth == max_depth {
return None;
}
key = (depth, self.key());
if memo.iter().any(|&x| x == key) {
return None;
}
memo.add(key);
if (depth as i32) == ((max_depth as i32) - 1) {
colors = vec![self.token[0]];
} else {
colors = None;
}
pub const moves: List = self.get_moves(colors);
for move_ in moves {
pub const data = self.do_move(starred!(move_)/*unsupported*/);
path.append(move_);
pub const result = self._search(path, memo, ((depth as i32) + 1), max_depth);
path.pop(-1);
self.undo_move(data);
if result {
return result;
}
}
return None;
 }

pub fn export<RT>(&self) -> RT {
pub static grid: &[_; 0] = &[];
pub static token = None;
pub const robots = COLORS.iter().map(|color| self.robots[color]).collect::<Vec<_>>();
for (index, cell) in self.grid.iter().enumerate() {
pub static mask: i32 = to_mask(cell);
if robots.iter().any(|&x| x == index) {
mask |= M_ROBOT;
}
grid.push(mask);
if cell.iter().any(|&x| x == self.token) {
token = index;
}
}
pub const robot = COLORS.index(self.token[0]);
return [("grid", grid), ("robot", robot), ("token", token), ("robots", robots)].iter().cloned().collect::<HashMap<_,_>>();
 }
 
}
