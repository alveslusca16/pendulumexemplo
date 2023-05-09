use speedy2d::Window;
use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;


use vector::Vector;
fn main() {
    let window = Window::new_centered("Pendulum", (800, 480)).unwrap();

    let win: MyWindowHandler = MyWindowHandler {  
        p: Pendulum::new(400.0, 0.0, 200.0),
        p2: Pendulum::new(400.0, 0.0, 400.0),
    };

    window.run_loop(win);
}

struct MyWindowHandler {
    p: Pendulum,
    p2: Pendulum,
    
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        
        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));

        self.p.update();
        self.p.draw(graphics);

        self.p2.update();
        self.p2.draw(graphics);

        helper.request_redraw();
    }
}
struct Pendulum {
    //This vector is the position of the pendulum.
    origin: vector::Vector,

    //This vector is the position of the ball.
    position: vector::Vector,
    //This is the angle of the pendulum.
    angle: f32,

    angular_velocity: f32,  
    angular_accelaration: f32,

    r: f32,    //The length of the pendulum.
    m: f32,    //The mass.
    g: f32,    //The gravity.
}

impl Pendulum {

    fn new(x: f32, y: f32, r: f32) -> Pendulum {
        Pendulum { 
            origin: vector::Vector::new(x,y),
            position: vector::Vector::new(0.0, 0.0),
            angle: 1.0, 
            angular_velocity: 0.0, 
            angular_accelaration: 0.0, 
            r: r, 
            m: 1.0, 
            g: 1.5, 
        }
    }

    fn update(&mut self) {  

        self.angular_accelaration = -1.0 * self.g * self.angle.sin() / self.r;


        self.angular_velocity += self.angular_accelaration;


        self.angle += self.angular_velocity;


        self.position
            .set(self.r * self.angle.sin(), self.r * self.angle.cos());


        self.position.add(&self.origin);
    }

    fn draw(&self, graphics: &mut Graphics2D) {
        graphics.draw_line((self.origin.x, self.origin.y), (self.position.x, self.position.y), 3.0, Color::RED);

        graphics.draw_circle((self.position.x, self.position.y), 30.0, Color::RED);
    }
}

mod vector {
    pub struct Vector {
        pub x: f32,
        pub y: f32,
    }

    impl Vector {
        pub fn new(x: f32,y: f32) -> Vector {
            Vector {
                x,
                y,
            }
        }

        pub fn add(&mut self, other: &Vector) -> &Vector {
            self.x += other.x;
            self.y += other.y;
            self
        }

        pub fn set(&mut self, x: f32, y: f32) -> &Vector {
            self.x = x;
            self.y = y;
            self
        }
    }
}