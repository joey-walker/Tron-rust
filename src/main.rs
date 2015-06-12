//
//	Tron project, at the moment most work here is from the sample at: 
// https://github.com/PistonDevelopers/Piston-Tutorials/tree/master/getting-started
//

//	Joey Walker
// Fiddling around, hopefully this turns into a real project.

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use piston::input::{ Button, Key};

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
   // rotation: f64,   // Rotation for the square.
    translatex: f64, //translate square
    translatey: f64 //translate square
}

impl App {
	fn render(&mut self, args: &RenderArgs) {
		use graphics::*;

       // const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
       const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
       const BLUE:	 [f32; 4] = [0.30, 0.30, 1.0, 1.0];

       let square = rectangle::square(0.0, 0.0, 25.0);
       //let rotation = self.rotation;
       let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);

       let translatex = self.translatex;
       let translatey = self.translatey;

       self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLUE, gl);

           /* let transform = c.transform.trans(x, y)
                                       .rot_rad(rotation)
                                       .trans(-25.0, -25.0);
                                       */
                                       let transform = c.transform.trans(translatex,translatey);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
   }

   fn update(&mut self, args: &UpdateArgs, xspeed:f64,yspeed:f64) {

    	//args.dt?  <- What exactly is this?

    	//Move our square
    	self.translatex += xspeed * args.dt;
    	self.translatey += yspeed * args.dt;
        // Rotate 2 radians per second.
       // self.rotation += 2.0 * args.dt;

    }
}

fn main() {
	let opengl = OpenGL::_3_2;


	let mut xspeed = 80.0;
	let mut yspeed = 0.0;

    // Create an Glutin window.
    let window = Window::new(
    	opengl,
    	WindowSettings::new(
    		"Tron",
    		[1000, 800]
    		)
    	.exit_on_esc(true)
    	);

    // Create a new game and run it.
    let mut app = App {
    	gl: GlGraphics::new(opengl),
    	//rotation: 0.0,
        translatey: 0.0, //start 
        translatex: 0.0
    };

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
        			println!("You pressed S :: heading down");
        			yspeed = 80.0;
        			xspeed =0.0;
        		},
        		Key::W => {
        			println!("You pressed W :: heading up");
        			yspeed = -80.0; 
        			xspeed =0.0;
        		},
        		Key::A => {
        			println!("You pressed A :: heading left");
        			yspeed = 0.0; 
        			xspeed = -80.0;
        		},
        		Key::D => {
        			println!("You pressed D :: heading right");
        			yspeed = 0.0; 
        			xspeed = 80.0;
        		},
        		_ => println!("You pressed {:?} . No Change", key),
        	}
        }
    }
}