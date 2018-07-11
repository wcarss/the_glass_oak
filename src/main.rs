extern crate tcod;
extern crate rand;

use tcod::console::*;
use tcod::colors;
use tcod::Color;
use tcod::map::{Map as FovMap, FovAlgorithm};
use rand::Rng;
use std::cmp;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const LIMIT_FPS: i32 = 20;
const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;
const COLOR_LIGHT_WALL: Color =  Color { r: 130, g: 110, b: 50};
const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_LIGHT_GROUND: Color = Color { r: 200, g: 180, b: 50};
const COLOR_DARK_GROUND: Color = Color { r: 50, g: 50, b: 150 };
const ROOM_MAX_SIZE: i32 = 10;
const ROOM_MIN_SIZE: i32 = 6;
const MAX_ROOMS: i32 = 30;
const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;
const FOV_LIGHT_WALLS: bool = true;
const TORCH_RADIUS: i32 = 12;
const MAX_ROOM_MONSTERS: i32 = 3;
const PLAYER: usize = 0;

type Map = Vec<Vec<Tile>>;

fn make_map(objects: &mut Vec<Object>) -> (Map, (i32, i32)) {
  // fills map with unblocked tiles... odd macro syntax!
  let mut map = vec![vec![Tile::wall(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];
  let mut rooms = vec![];
  let mut starting_position = (0, 0);

  for _ in 0..MAX_ROOMS {
    let w = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
    let h = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);

    let x = rand::thread_rng().gen_range(0, MAP_WIDTH - w);
    let y = rand::thread_rng().gen_range(0, MAP_HEIGHT - h);

    let new_room = Rect::new(x, y, w, h);
    let failed = rooms.iter().any(|other_room| new_room.intersects_with(other_room));

    if !failed {
      create_room(new_room, &mut map);
      place_objects(new_room, &map, objects);

      let (new_x, new_y) = new_room.center();

      if rooms.is_empty() {
        starting_position = (new_x, new_y);
      } else {
        let (prev_x, prev_y) = rooms[rooms.len() - 1].center();

        if rand::random() {
          create_h_tunnel(prev_x, new_x, prev_y, &mut map);
          create_v_tunnel(prev_y, new_y, new_x, &mut map);
        } else {
          create_v_tunnel(prev_y, new_y, prev_x, &mut map);
          create_h_tunnel(prev_x, new_x, new_y, &mut map);
        }
      }
      rooms.push(new_room);
    }
  }

  (map, starting_position)
}

fn create_room(room: Rect, map: &mut Map) {
  for x in (room.x1 + 1) .. room.x2 {
    for y in (room.y1 + 1) .. room.y2 {
      map[x as usize][y as usize] = Tile::empty();
    }
  }
}

fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut Map) {
  for x in cmp::min(x1, x2)..(cmp::max(x1, x2)+1) {
    map[x as usize][y as usize] = Tile::empty();
  }
}

fn create_v_tunnel(y1: i32, y2: i32, x: i32, map: &mut Map) {
  for y in cmp::min(y1, y2)..(cmp::max(y1, y2)+1) {
    map[x as usize][y as usize] = Tile::empty();
  }
}


#[derive(Clone, Copy, Debug)]
struct Rect {
  x1: i32,
  y1: i32,
  x2: i32,
  y2: i32,
}

impl Rect {
  pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
    Rect { x1: x, y1: y, x2: x + w, y2: y + h }
  }

  pub fn center(&self) -> (i32, i32) {
    let center_x = (self.x1+self.x2)/2;
    let center_y = (self.y1+self.y2)/2;
    (center_x, center_y)
  }

  pub fn intersects_with(&self, other: &Rect) -> bool {
    (self.x1 <= other.x2) && (self.x2 >= other.x1) &&
      (self.y1 <= other.y2) && (self.y2 >= other.y1)
  }
}


#[derive(Clone, Copy, Debug)]
struct Tile {
  blocked: bool,
  block_sight: bool,
  explored: bool,
}

impl Tile {
  pub fn empty () -> Self {
    Tile { blocked: false, explored: false, block_sight: false }
  }

  pub fn wall () -> Self {
    Tile { blocked: true, explored: false, block_sight: true }
  }
}


#[derive(Debug)]
struct Object {
  x: i32,
  y: i32,
  char: char,
  color: Color,
  name: String,
  blocks: bool,
  alive: bool,
}

impl Object {
  pub fn new (x: i32, y: i32, char: char, name: &str, color: Color, blocks: bool) -> Self {
    Object {
      x: x,
      y: y,
      char: char,
      color: color,
      name: name.into(),
      blocks: blocks,
      alive: false,
    }
  }

  pub fn draw (&self, con: &mut Console) {
    con.set_default_foreground(self.color);
    con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
  }

  pub fn clear (&self, con: &mut Console) {
    con.put_char(self.x, self.y, ' ', BackgroundFlag::None);
  }

  pub fn pos(&self) -> (i32, i32) {
    (self.x, self.y)
  }

  pub fn move_by (&mut self, dx: i32, dy: i32) {
    let (x, y) = self.pos();
    self.set_pos(x + dx, y + dy);
  }

  pub fn set_pos(&mut self, x: i32, y: i32) {
    self.x = x;
    self.y = y;
  }
}



fn place_objects(room: Rect, map: &Map, objects: &mut Vec<Object>) {
  let num_creatures = rand::thread_rng().gen_range(0, MAX_ROOM_MONSTERS+1);

  for _ in 0..num_creatures {
    let x = rand::thread_rng().gen_range(room.x1+1, room.x2);
    let y = rand::thread_rng().gen_range(room.y1+1, room.y2);

    if !is_blocked(x, y, map, objects) {
      let rand_control = rand::random::<f32>();
      let mut creature = if rand_control < 0.8 {
        Object::new(x, y, 'o', "orc", colors::DESATURATED_GREEN, true)
      } else if rand_control < 0.96 {
        Object::new(x, y, 'T', "troll", colors::DARKER_GREEN, true)
      } else {
        Object::new(x, y, '&', "npc", colors::YELLOW, true)
      };

      creature.alive = true;
      objects.push(creature);
    }
  }
}


fn is_blocked(x: i32, y: i32, map: &Map, objects: &Vec<Object>) -> bool {
  if map[x as usize][y as usize].blocked {
    return true;
  }

  objects.iter().any(|object| {
    object.blocks && object.pos() == (x, y)
  })
}


fn render_all(root: &mut Root, con: &mut Offscreen, objects: &Vec<Object>, map: &mut Map, fov_map: &mut FovMap, fov_recompute: bool) {
  if fov_recompute {
    let player = &objects[PLAYER];
    fov_map.compute_fov(player.x, player.y, TORCH_RADIUS, FOV_LIGHT_WALLS, FOV_ALGO);

    for y in 0 .. MAP_HEIGHT {
      for x in 0 .. MAP_WIDTH {
        let visible = fov_map.is_in_fov(x, y);
        let wall = map[x as usize][y as usize].block_sight;
        let color = match (visible, wall) {
          // outside the fov
          (false, true) => COLOR_DARK_WALL,
          (false, false) => COLOR_DARK_GROUND,
          // inside the fov
          (true, true) => COLOR_LIGHT_WALL,
          (true, false) => COLOR_LIGHT_GROUND,
        };
        let explored = &mut map[x as usize][y as usize].explored;
        if visible {
          *explored = true;
        }
        if *explored {
          con.set_char_background(x, y, color, BackgroundFlag::Set);
        }
      }
    }
  }

  for object in objects {
    if fov_map.is_in_fov(object.x, object.y) {
      object.draw(con);
    }
  }

  blit(con, (0, 0), (MAP_WIDTH, MAP_HEIGHT), root, (0, 0), 1.0, 1.0);
}

fn move_check ((pos_x, pos_y): (i32, i32), (move_x, move_y): (i32, i32), map: &Map, objects: &Vec<Object>) -> (i32, i32) {
  if !is_blocked(pos_x+move_x, pos_y+move_y, map, objects) {
    (move_x, move_y)
  } else {
    (0, 0)
  }
}

fn handle_keys(root: &mut Root, objects: &mut Vec<Object>, map: &Map) -> bool {
  use tcod::input::Key;
  use tcod::input::KeyCode::*;

  let key = root.wait_for_keypress(true);
  let pos = objects[PLAYER].pos();
  let (dx, dy) = match key {
    Key { code: Up, .. } => move_check(pos, (0, -1), map, objects),
    Key { code: Down, .. } => move_check(pos, (0, 1), map, objects),
    Key { code: Left, .. } => move_check(pos, (-1, 0), map, objects),
    Key { code: Right, .. } => move_check(pos, (1, 0), map, objects),
    _ => (0, 0),
  };

  if dx != 0 || dy != 0 {
    objects[PLAYER].move_by(dx, dy);
  }

  match key {
    Key { code: Enter, alt: true, .. } => {
      let fullscreen = root.is_fullscreen();
      root.set_fullscreen(!fullscreen);
    },
    Key { code: Escape, .. } => return true,
    _ => {},
  };

  false
}


fn main() {
  let mut root = Root::initializer()
    .font("square10x10.png", FontLayout::Tcod)
    .font_type(FontType::Greyscale)
    .size(SCREEN_WIDTH, SCREEN_HEIGHT)
    .title("The Glass Oak")
    .init();

  let mut con = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);

  tcod::system::set_fps(LIMIT_FPS);

  let mut previous_player_position = (-1, -1);
  let player = Object::new(0, 0, '%', "player", colors::WHITE, true);
  let mut objects = vec![player];
  let (mut map, (player_x, player_y)) = make_map(&mut objects);
  let mut fov_map = FovMap::new(MAP_WIDTH, MAP_HEIGHT);
  for y in 0..MAP_HEIGHT {
    for x in 0..MAP_WIDTH {
      fov_map.set(
        x,
        y,
        !map[x as usize][y as usize].block_sight,
        !map[x as usize][y as usize].blocked
      );
    }
  }

  objects[PLAYER].set_pos(player_x, player_y);
  objects[PLAYER].alive = true;

  while !root.window_closed() {
    let fov_recompute = previous_player_position != (objects[PLAYER].x, objects[PLAYER].y);
    render_all(&mut root, &mut con, &objects, &mut map, &mut fov_map, fov_recompute);
    root.flush();

    for object in &objects {
      object.clear(&mut con);
    }

    previous_player_position = (objects[PLAYER].x, objects[PLAYER].y);
    let exit = handle_keys(&mut root, &mut objects, &map);
    if exit {
      break
    }
  }
}
