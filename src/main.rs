//
//	Tron project, at the moment most work here is from the sample at: 
// https://github.com/PistonDevelopers/Piston-Tutorials/tree/master/getting-started
//

//	Joey Walker
// Fiddling around, hopefully this turns into a real project.

extern crate piston;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event::*;
use sdl2_window::Sdl2Window as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use piston::input::{ Button, Key};
use graphics::*;


#[derive(Debug)]
enum PlayerDirection{
  Up,
  Left,
  Right,
  Down
}


// Our player object
pub struct Player {
  x: f64, // our initial xlocation
  y: f64, // our initial ylocation
  color: [f32; 4], // color of player, example: .. = [0.0, 1.0, 0.0, 1.0];
  playerobj : types::Rectangle,
  direction : PlayerDirection
}



pub struct App {
  gl: GlGraphics, // OpenGL drawing backend.

  player1: Player, 

  player2: Player,

  //use vector for trails.
  trails: Vec<types::Rectangle>,

  trail1_startx: f64,

  trail1_starty: f64,

  trail1_length: f64,

  trail2_length:f64
}

impl App {

 fn render(&mut self, args: &RenderArgs) {
 // const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
     //const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
     const BLUE:	 [f32; 4] = [0.30, 0.30, 1.0, 1.0];

     //let rotation = self.rotation;

     let (trail1_startx, trail1_starty) = (self.trail1_startx, self.trail1_starty);

     //player 1

     let (player1x, player1y) = (self.player1.x, self.player1.y); // initial coordinates
     let player1_color = self.player1.color; //player's color
     let player1_obj = self.player1.playerobj; //player object, currently rectangle

     //player 2 

     let (player2x, player2y) = (self.player2.x, self.player2.y);
     let player2_color = self.player2.color;
     let player2_obj = self.player2.playerobj;

    //draw the scene

    let testtrail = self.trail1_length;

    self.gl.draw(args.viewport(), |c, gl| {
          // Clear the screen.
          clear(BLUE, gl);

         // let transform = c.transform.trans(x, y).rot_rad(rotation).trans(-25.0, -25.0);

         //coordinates of our player
         let transform = c.transform.trans(player1x,player1y);
         let trailtransform = c.transform.trans(0.0,0.0);
         let transform1 = c.transform.trans(player2x,player2y);

          // Draw a box rotating around the middle of the screen.
          rectangle(player1_color, player1_obj, transform, gl);
          rectangle(player1_color, [20.0,25.75,testtrail,10.0], trailtransform, gl);
          //second player
          rectangle(player2_color, player2_obj, transform1, gl);

        });

  }

  fn update(&mut self, args: &UpdateArgs, xspeed:f64,yspeed:f64) {

  	//args.dt?  <- What exactly is this? <- Delta time.

  	//Move our square
  	self.player1.x += xspeed * args.dt;
  	self.player1.y += yspeed * args.dt;
    self.trail1_length += xspeed * args.dt;
      // Rotate 2 radians per second.
     // self.rotation += 2.0 * args.dt;

   }
 }

 fn main() {
   let opengl = OpenGL::_3_2;


   let mut xspeed = 80.0;
   let mut yspeed = 0.0;
  // Create a window.
  let window = Window::new(
  	opengl,
  	WindowSettings::new(
  		"Tron",
  		[1000, 800]
  		)
  	.exit_on_esc(true)
    .fullscreen(false)
    .vsync(false),
    );

  // Create a new game and run it.
  let mut app = App {
  	gl: GlGraphics::new(opengl),

     player1: Player { //create our player
      x: 20.0,
      y: 20.0,
      color: [1.0, 0.0, 0.0, 1.0],
      playerobj: rectangle::square(0.0, 0.0, 25.0),
      direction: PlayerDirection::Right
    },

     player2: Player { //create our player
      x: 600.0,
      y: 600.0,
      color: [0.0, 1.0, 0.0, 1.0],
      playerobj: rectangle::square(0.0, 0.0, 25.0),
      direction: PlayerDirection::Left
    },

    trails: Vec::new(),

    trail1_startx: xspeed,
    trail1_starty: yspeed,
    trail1_length: 1.0,

    trail2_length: 2.0

  };

  //our event loop.
  for e in window.events() {
  	if let Some(r) = e.render_args() {
  		app.render(&r);
  	}

  	if let Some(u) = e.update_args() {
  		app.update(&u, xspeed,yspeed);

  	}

      //Keyboard Events
      if let Some(Button::Keyboard(key)) = e.press_args() {
      	match key {
      		Key::S => {
            match app.player1.direction{
              PlayerDirection::Up =>{},
              _=>{
                println!("You pressed S :: heading down");
                yspeed = 80.0;
                xspeed =0.0;
                app.player1.direction = PlayerDirection::Down;
              }
            }
               // println!("{:?} .", app.player1.direction);
             },
             Key::W => {
              match app.player1.direction{
                PlayerDirection::Down => {},
                _=> {
                 println!("You pressed W :: heading up");
                 yspeed = -80.0; 
                 xspeed =0.0;
                 app.player1.direction = PlayerDirection::Up;
               }
             }
           },
           Key::A => {
            match app.player1.direction{
              PlayerDirection::Right => {},
              _ =>{
               println!("You pressed A :: heading left");
               yspeed = 0.0; 
               xspeed = -80.0;
               app.player1.direction = PlayerDirection::Left;
             }
           }
         },
         Key::D => {
           match app.player1.direction{
            PlayerDirection::Left => {},
            _ => {
             println!("You pressed D :: heading right");
             yspeed = 0.0; 
             xspeed = 80.0;
             app.player1.direction = PlayerDirection::Right;
           }
         }
       },
       _ => println!("You pressed {:?} . No Change", key),
     }

   }
 }
}