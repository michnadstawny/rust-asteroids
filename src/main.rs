extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point as SDLPoint;
use std::time::Duration;
use std::f64::consts::PI;

mod game;
use game::Point;
use game::Ship;


//functions

fn rotate_points(points: &mut[Point], angle: f64) {
    for point in points {
        point.x = point.x * angle.cos() + point.y * angle.sin();
        point.y = point.x * angle.cos() - point.y * angle.sin();
    }
}

fn move_points(points: &mut[Point], x_offset: f64, y_offset: f64) {
    for point in points {
        point.x = point.x + x_offset;
        point.y = point.y + y_offset;
    }
}

fn points_to_SDLpoints(points: &[Point], SDLpoints: &mut[SDLPoint]) {
    if points.len() == SDLpoints.len() {
        let a = points.len();
        for i in 0..a {
            SDLpoints[i].x = points[i].x as i32;
            SDLpoints[i].y = points[i].y as i32;
        }
    } else { println!("uh oh"); }
}

fn ship_update(ship: &mut Ship, delta: f64) {
    ship.speed.x += ship.accel.x * delta;
    ship.speed.y += ship.accel.y * delta;
    ship.rot_speed += ship.rot_accel * delta;
    ship.rotation += ship.rot_speed * delta;

    let a: f64 = 0.0;
    ship.rotation %= ship.rotation * (4.0_f64 * a.acos());
    ship.rot_speed *= (1.0_f64 - 0.9_f64).powf(delta);

    ship.position.x += ship.speed.x * delta;
    ship.position.y += ship.speed.y * delta;
}




fn main() -> Result<(), String> {

//struct for inputs

    struct Keys {
        up: bool,
        down: bool,
        left: bool,
        right: bool,
        space: bool,
    }

    let mut keys = Keys {
        up: false,
        down: false,
        left: false,
        right: false,
        space: false,
    };

//sdl init

    let res_x = 1280;
    let res_y = 720;

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("asteroids", res_x, res_y)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;

    canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;

    let mut timer = sdl_context.timer()?;

    let mut last_frame = timer.ticks() as f64;

//array for SDL rendering of ship

    let mut SDLship: [SDLPoint; 5] = [SDLPoint::new(0, 0),
                                      SDLPoint::new(0, 0),
                                      SDLPoint::new(0, 0),
                                      SDLPoint::new(0, 0),
                                      SDLPoint::new(0, 0)
    ];

//shape to draw, hardcoded for now

    let ship_shape_template: [Point; 5] = [Point {x: 0.0, y: -25.0},
                                           Point {x: -20.0, y: 15.0},
                                           Point {x: 0.0, y: 5.0},
                                           Point {x: 20.0, y: 15.0},
                                           Point {x: 0.0, y: -25.0}
    ];

//make a mutable copy of ship shape template array to work on

    let mut ship_shape = ship_shape_template.clone();



    let mut ship = Ship {
        position: Point{x: (res_x / 2) as f64, y: (res_y / 2) as f64},
        speed: Point{x: 0.0, y: 0.0},
        accel: Point{x: 0.0, y: 0.0},
        rotation: PI/4.0,
        rot_speed: 0.0,
        rot_accel: 0.0,
    };

//magic numbers for tuning of ship controls
//    let rot_speed_value: f64 = 30.0;
//    let rot_accel_value: f64 = 360.0;
//    let rot_resist_value: f64 = 0.9;



//main loop

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape), ..
                } => {  break 'running  },
//key down
                Event::KeyDown {
                    keycode: Some(Keycode::Up), ..
                } => {  keys.up = true;  },
                Event::KeyDown {
                    keycode: Some(Keycode::Down), ..
                } => {  keys.down = true;  },
                Event::KeyDown {
                    keycode: Some(Keycode::Left), ..
                } => {  keys.left = true;  },
                Event::KeyDown {
                    keycode: Some(Keycode::Right), ..
                } => {  keys.right = true;  },
                Event::KeyDown {
                    keycode: Some(Keycode::Space), ..
                } => {  keys.space = true;  },
//key up
                Event::KeyUp {
                    keycode: Some(Keycode::Up), ..
                } => {  keys.up = false;  },
                Event::KeyUp {
                    keycode: Some(Keycode::Down), ..
                } => {  keys.down = false;  },
                Event::KeyUp {
                    keycode: Some(Keycode::Left), ..
                } => {  keys.left =  false;  },
                Event::KeyUp {
                    keycode: Some(Keycode::Right), ..
                } => {  keys.right = false;  },
                Event::KeyUp {
                    keycode: Some(Keycode::Space), ..
                } => {  keys.space = false;  },
//else
                _ => {}
            }
        }

        canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 255));
        canvas.clear();

//get delta

        let this_frame = timer.ticks() as f64;
        let mut delta = this_frame - last_frame;
        last_frame = this_frame;
        delta = delta / 1000.0;

//temporary controls

        if keys.up {ship.accel.y -= 1.0;}
        if keys.down {ship.accel.y += 1.0;}
        if keys.left {ship.accel.x -= 1.0;}
        if keys.right {ship.accel.x += 1.0;}

//ship physics

        ship_update(&mut ship, delta);
        ship_shape = ship_shape_template;

//rotate_points has unexpected results - need to test this

//        rotate_points(&mut ship_shape[..], ship.rotation);
//        ship.rotation = ship.rotation + PI/16.0;
        move_points(&mut ship_shape[..], ship.position.x, ship.position.y);

//drawing

        points_to_SDLpoints(&ship_shape, &mut SDLship);
        canvas.set_draw_color(sdl2::pixels::Color::RGBA(255, 255, 255, 255));
        canvas.draw_lines(&SDLship[..]);
        canvas.present();

        std::thread::sleep(Duration::from_millis(10));
    }

    Ok(())
}
