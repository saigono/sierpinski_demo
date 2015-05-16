extern crate sfml;

use sfml::system::{Vector2f, Vector2u};
use sfml::window::{ContextSettings, VideoMode, event, Close};
use sfml::graphics::{RenderWindow, RenderTarget, ConvexShape, Color};

struct SierpinskiTriangle {
    v1: Vector2f,
    v2: Vector2f,
    v3: Vector2f,
}

impl SierpinskiTriangle {
    fn new(v1: &Vector2f, v2: &Vector2f, v3: &Vector2f) -> SierpinskiTriangle {
        SierpinskiTriangle {
            v1: *v1,
            v2: *v2,
            v3: *v3
        }
    }

    fn as_convex_shape(&self) -> ConvexShape {
        return new_triangle(&self.v1, &self.v2, &self.v3);
    }

    fn children(&self) -> [SierpinskiTriangle; 3] {
        return [
            SierpinskiTriangle::new(&self.v1, &((self.v1+self.v2)/2.), &((self.v1+self.v3)/2.)),
            SierpinskiTriangle::new(&self.v2, &((self.v1+self.v2)/2.), &((self.v2+self.v3)/2.)),
            SierpinskiTriangle::new(&self.v3, &((self.v3+self.v2)/2.), &((self.v1+self.v3)/2.)),
        ];
    }

    fn draw(&self, window: &mut RenderWindow, max_steps: u8) {
        let mut triangle = self.as_convex_shape();
        triangle.set_fill_color(&Color::black());
        window.draw(&triangle);
        self._draw(window, 0, max_steps);
    }

    fn _draw(&self, window: &mut RenderWindow, current_level: u8, max_steps: u8) {
        if current_level >= max_steps {
            return;
        }
        let mut triangle = new_triangle(&((self.v1+self.v2)/2.), 
                                    &((self.v2+self.v3)/2.), 
                                    &((self.v3+self.v1)/2.));
        triangle.set_fill_color(&Color::white());
        window.draw(&triangle);

        for t in self.children().iter() {
            t._draw(window, current_level+1, max_steps);
        }
    }
}

fn new_triangle<'s>(v1: &Vector2f, v2: &Vector2f, v3: &Vector2f) -> ConvexShape<'s>{
    let mut triangle = ConvexShape::new(3).expect("Can not create a triangle");
    triangle.set_point(0, v1);
    triangle.set_point(1, v2);
    triangle.set_point(2, v3);
    return triangle;
}

fn main() {
    // Create the window of the application
    let pad: u32 = 10;
    let size = Vector2u::new(1600, 1600);
    let mut window = RenderWindow::new(VideoMode::new_init(size.x+2*pad, size.y+2*pad, 32),
                                       "Sierpinski demo",
                                       Close,
                                       &ContextSettings::default())
                         .expect("Cannot create a new Render Window.");

    let main_triangle = SierpinskiTriangle::new(&Vector2f::new((0+pad) as f32, (size.y+pad) as f32), 
                                                &Vector2f::new((size.x/2+pad) as f32, (0+pad) as f32), 
                                                &Vector2f::new((size.x+pad) as f32, (size.y+pad) as f32));               
    
    while window.is_open() {
        // Handle events
        for event in window.events() {
            match event {
                event::Closed => window.close(),
                _             => {/* do nothing */}
            }
        }

        // Clear the window
        window.clear(&Color::white());
        // Draw the Sierpinski Triangle
        main_triangle.draw(&mut window, 8);
 
        // Display things on screen
        window.display()
    }
}