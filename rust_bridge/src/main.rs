#[macro_use]
extern crate log;
extern crate phf;

mod bridge;
mod bridge_manager;
mod cube;

use bridge::{IDInfo, Message};
use bridge_manager::BridgeManager;
use cube::CubeReceiver;
use crossbeam_channel::{unbounded, Receiver, Sender};
use cube::CubeManager;
use env_logger::{Builder, Env};
use glutin::dpi::PhysicalSize;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, TextureSettings};
use phf::phf_map;
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, ButtonArgs, ButtonEvent, ButtonState, Key, RenderArgs, RenderEvent, ResizeEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::thread;
use std::time::Instant;
use std::io;
mod cubelist;
use cubelist::return_toio_mac;
use std::fs::File;
use std::io::{prelude::*, BufReader};

static KEYWORDS: phf::Map<&'static str, u8> = phf_map! {
    //enter bridge id 
    "192.168.0.100" => 1, 
};

const DOTS_PER_METER: f64 = 411.0 / 0.560;
const CUBE_SIZE: f64 = 0.0318 * DOTS_PER_METER; // 31.8mm

const TOIO_BLUE: [f32; 4] = [0.000, 0.684, 0.792, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

const COLORS: [[u8; 3]; 3] = [[255, 255, 0], [255, 0, 255], [0, 255, 255]];

#[derive(Debug, Clone)]
struct Cube {
    x: f64,
    y: f64,
    a: f64,
    hit: bool,
    last_message_time: Instant,
    bridge: String,
}

#[derive(Debug, Clone)]
struct Bridge {
    id: u8,
    cubes: HashMap<String, Cube>,
    last_message_time: Instant,
}

struct App<'a> {
    gl: GlGraphics,
    sender: Option<Sender<Message>>,
    receiver: Option<Receiver<Message>>,
    key_state: HashMap<Key, ButtonState>,
    bridges: HashMap<String, Bridge>,
    cubes: HashMap<String, Cube>,
    count: usize,
    glyph_cache: GlyphCache<'a>,
    cubelist: Vec<String>
}

impl App<'_> {
    fn new(gl_version: OpenGL, cubelist: Vec<String>) -> App<'static> {
        App {
            gl: GlGraphics::new(gl_version),
            sender: None,
            receiver: None,
            key_state: HashMap::with_capacity(256),
            cubes: HashMap::with_capacity(256),
            bridges: HashMap::with_capacity(32),
            count: 0,
            glyph_cache: GlyphCache::new("assets/RobotoCondensed-Regular.ttf", (), TextureSettings::new()).unwrap(),
            cubelist: cubelist,
        }
    }

    fn start(&mut self) {
        let (to_cubes_sender, to_cubes_receiver) = unbounded(); // -> Cubes
        let (to_bridges_sender, to_bridges_receiver) = unbounded(); // -> Bridge
        let (to_ui_sender, to_ui_receiver) = unbounded(); // -> UI
        self.receiver = Some(to_ui_receiver);
        let cubelist = self.cubelist.clone();
        {
            let to_cubes_sender = to_cubes_sender.clone();
            thread::spawn(move || {
                BridgeManager::new(to_cubes_sender, to_bridges_receiver, cubelist).start();
            });
        }
        {
            let to_ui_sender = to_ui_sender.clone();
            thread::spawn(move || {
                CubeManager::new(to_bridges_sender, to_cubes_receiver, to_ui_sender).start();
            });
        }
        {
            let to_cubes_sender = to_cubes_sender.clone();
            thread::spawn(move || {
                CubeReceiver::new(to_cubes_sender).start();
            });
        }
        self.sender = Some(to_cubes_sender);
    }

    fn input(&mut self, args: &ButtonArgs) {
        match args.button {
            Button::Keyboard(key) => match self.key_state.entry(key) {
                Occupied(mut entry) => {
                    let prev = entry.insert(args.state);
                    if prev != args.state && args.state == ButtonState::Press {
                        self.on_press(&key);
                    }
                }
                Vacant(entry) => {
                    entry.insert(args.state);
                    if args.state == ButtonState::Press {
                        self.on_press(&key);
                    }
                }
            },
            _ => (),
        }
    }

    fn on_press(&mut self, key: &Key) {
        trace!("on_press: {:?}", key);
        match key {
            Key::D1 => {
                self.sender.as_ref().unwrap().try_send(Message::ShowBatteryInfoAll).unwrap();
            }
            Key::D2 => {
                let [r, g, b] = COLORS[self.count % COLORS.len()];
                self.count = self.count + 1;
                self.sender.as_ref().unwrap().try_send(Message::SetLampAll(r, g, b)).unwrap();
            }
            Key::D3 => {
                let angle = (self.count * 135) % 360;
                self.count = self.count + 1;
                self.sender.as_ref().unwrap().try_send(Message::SetDirectionAll(angle as u16)).unwrap();
            }
            Key::D4 => {
                self.sender.as_ref().unwrap().try_send(Message::LookCenterAll).unwrap();
            }
            Key::D5 => {
                self.sender.as_ref().unwrap().try_send(Message::StartGoAround).unwrap();
            }
            Key::D6 => {
                self.sender.as_ref().unwrap().try_send(Message::StopAll).unwrap();
            }
            _ => (),
        }
    }

    fn update(&mut self, _args: &UpdateArgs) {
        for message in self.receiver.as_mut().unwrap().try_iter() {
            match message {
                Message::Available(ip_address, _, mac_address) => {
                    let key: &str = &mac_address;
                    self.bridges
                        .entry(ip_address)
                        .and_modify(|bridge| bridge.last_message_time = Instant::now())
                        .or_insert_with(|| Bridge {
                            id: *KEYWORDS.get(key).unwrap_or(&0),
                            cubes: HashMap::with_capacity(10),
                            last_message_time: Instant::now(),
                        });
                }

                Message::Connected(bridge_address, cube_address) => {
                    let cube = self.cubes.entry(cube_address.clone()).or_insert_with(|| Cube {
                        x: -999.9,
                        y: -999.9,
                        a: 0.0,
                        hit: false,
                        last_message_time: Instant::now(),
                        bridge: bridge_address.clone(),
                    });
                    self.bridges.entry(bridge_address).and_modify(|bridge| {
                        bridge.cubes.entry(cube_address).or_insert(cube.clone());
                    });
                }

                Message::Disconnected(bridge_address, cube_address) => {
                    self.cubes.remove(&cube_address);
                    self.bridges.entry(bridge_address).and_modify(|bridge| {
                        bridge.cubes.remove(&cube_address);
                    });
                }

                Message::IDInfo(cube_address, id_info) => match id_info {
                    IDInfo::PositionID(x, y, a) => match self.cubes.entry(cube_address.clone()) {
                        Occupied(mut entry) => {
                            let mut cube = entry.get_mut();
                            cube.x = From::from(x);
                            cube.y = From::from(y);
                            cube.a = From::from(a);
                            self.bridges.entry(cube.bridge.clone()).and_modify(|bridge| {
                                bridge.cubes.entry(cube_address).and_modify(|cube| {
                                    cube.last_message_time = Instant::now();
                                });
                            });
                        }
                        Vacant(_) => (),
                    },
                    IDInfo::PositionIDMissed => {
                        self.cubes.entry(cube_address).and_modify(|cube| {
                            cube.x = -999.9;
                            cube.y = -999.9;
                            cube.a = 0.0;
                        });
                    }
                    _ => (),
                },

                Message::HitStateChanged(cube_address, hit) => {
                    debug!("Hit state changed: {:?}, {:?}", cube_address, hit);
                    self.cubes.entry(cube_address).and_modify(|cube| cube.hit = hit);
                }

                _ => (),
            }
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        let context = self.gl.draw_begin(args.viewport());

        graphics::clear(TOIO_BLUE, &mut self.gl);

        self.draw_cubes(&context);
        self.draw_ui(&context, args.window_size);

        self.gl.draw_end();
    }

    fn draw_cubes(&mut self, context: &graphics::Context) {
        use graphics::*;
        let gl = &mut self.gl;
        let square = rectangle::centered_square(0.0, 0.0, CUBE_SIZE / 2.0);
        for cube in self.cubes.values() {
            let transform = context.transform.trans(cube.x, cube.y).rot_deg(cube.a - 90.0);
            rectangle(WHITE, square, transform, gl);

            let a = CUBE_SIZE / 2.0;
            let color = if cube.hit { RED } else { GREEN };
            let start: [f64; 2] = [0.0, a + 1.0];
            let end: [f64; 2] = [0.0, start[1] + cube::HIT_LEN];
            line_from_to(color, 1.0, start, end, transform, gl);
        }
    }

    fn draw_ui(&mut self, context: &graphics::Context, window_size: [f64; 2]) {
        use graphics::*;
        self.draw_text(&context, 10.0, 40.0, 30, &WHITE, "ZOROZORO Controller");
        // self.draw_text(&context, 10.0, 70.0, 18, &WHITE, &format!("Bridges: {}", self.cubes.len()));
        self.draw_text(&context, 10.0, 70.0, 18, &WHITE, &format!("Connected Cubes: {}", self.cubes.len()));
        {
            let filled = rectangle::square(0.0, 0.0, 10.0);
            let stroke = rectangle::square(0.0, 0.0, 9.0);
            let mut y = 100.0;
            let now = Instant::now();
            let mut bridges = self.bridges.values().cloned().collect::<Vec<_>>();
            bridges.sort_by(|a, b| a.id.cmp(&b.id));
            for bridge in bridges {
                let dt = now - bridge.last_message_time;
                let color = [1.0, 1.0, 1.0, (1.0 - dt.as_secs_f32() / 3.0).max(0.4).min(1.0)];
                let id_str = format!("{:02}", bridge.id);
                self.draw_text(&context, 10.0, y + 12.0, 12, &color, &id_str);
                y += 2.0;
                let mut x = 30.0;
                let gl = &mut self.gl;
                for cube in bridge.cubes.values() {
                    let dt = now - cube.last_message_time;
                    let color = [1.0, 1.0, 1.0, (1.0 - dt.as_secs_f32() * 2.0).max(0.25).min(1.0)];
                    rectangle(color, filled, context.transform.trans(x, y), gl);
                    x += 15.0;
                }
                for _ in bridge.cubes.len()..10 {
                    Rectangle::new_border([1.0, 1.0, 1.0, 0.3], 0.5).draw(stroke, &Default::default(), context.transform.trans(x + 0.5, y + 0.5), gl);
                    x += 15.0;
                }
                y += 18.0;
            }
        }
        {
            const MENU_TEXT: &str = "1. Battery Status\n2. Random Color\n3. Random Rotate\n4. Look Center\n5. Go Around\n6. Stop\n0. Shutdown";
            let lines = MENU_TEXT.split("\n").collect::<Vec<_>>();
            let mut y = window_size[1] - 25.0 * (lines.len() as f64) - 10.0;
            for line in lines.iter() {
                y += 25.0;
                self.draw_text(&context, 10.0, y, 18, &WHITE, line);
            }
        }
    }

    fn draw_text(&mut self, context: &graphics::Context, x: f64, y: f64, size: u32, color: &[f32; 4], s: &str) {
        use graphics::*;
        let gl = &mut self.gl;
        let glyph_cache = &mut self.glyph_cache;
        text::Text::new_color(*color, size)
            .draw(s, glyph_cache, &DrawState::default(), context.transform.trans(x, y), gl)
            .unwrap();
    }
}

fn main() -> io::Result<()> {
    Builder::from_env(Env::default()).format_timestamp_millis().init();
    info!("Starting up!");
    let gl_version = OpenGL::V3_3;
    let mut window: Window = WindowSettings::new("Cube Controller", [1000, 1000])
        .graphics_api(gl_version)
        .samples(4)
        .vsync(true)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut cube_ids : Vec<usize> = Vec::new();
    let f = File::open("cubes.csv")?;
    let reader = BufReader::new(f);
    for line in reader.lines(){
        for i in line?.to_string().split(","){
            cube_ids.push(i.trim().parse::<usize>().unwrap());
        }
    }
    
    let mut cube_macs = vec![];
    for i in 0..cube_ids.len(){
        let id = cube_ids[i].clone();
        let mac = return_toio_mac(id);
        cube_macs.push(mac);
    }
    let mut app = App::new(gl_version, cube_macs);
    app.start();
    
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        // trace!("{:?}", e);
        if let Some(args) = e.button_args() {
            app.input(&args);
        }
        if let Some(args) = e.update_args() {
            app.update(&args);
        }
        // if let Some(args) = e.resize_args() {
        //     window.ctx.resize(PhysicalSize::new(args.draw_size[0].into(), args.draw_size[1].into()));
        // }
        if let Some(args) = e.render_args() {
            app.render(&args);
        }
    }
    
    info!("Done!");
    Ok(())
}