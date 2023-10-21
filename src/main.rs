use rand::Rng;

trait Monad {
    fn bind<F>(self, f: F) -> Self
        where F: FnOnce() -> Self;
}

// Define struct for Drone
#[derive(Debug)]
struct Drone {
    x_pos: i32,
    y_pos: i32,
    z_pos: i32,
}

// Todo::Handle | Trace side effects
impl Monad for Drone {
    fn bind<F>(self, f: F) -> Self where F: FnOnce() -> Self {
        f()
    }
}

// Define enum for Movement i.e Left, Right etc
#[derive(Clone, Copy)]
enum Movement {
    Forward,
    Back,
    Left,
    Right,
    Up,
    Down,
}

// Define implementation dor Drone
impl Drone {
    fn move_drone(&mut self, movement: Movement) {
        match movement {
            Movement::Forward => self.z_pos += 1,
            Movement::Back => self.z_pos += -1,
            Movement::Left => self.x_pos -= 1,
            Movement::Right => self.z_pos += 1,
            Movement::Up => self.y_pos += 1,
            Movement::Down => self.y_pos -= 1,
        }
    }
}

fn main() {
    // Provide initial drone position
    let mut init_drone = Drone {
        x_pos: 0,
        y_pos: 100,
        z_pos: 0,
    };

    // Generate random values
    let mut rng = rand::thread_rng();
    // Specify range for random_values
    let random_values: Vec<Movement> = (0..15)
        .map(|_| match rng.gen_range(0..6) {
            0 => Movement::Forward,
            1 => Movement::Back,
            2 => Movement::Left,
            3 => Movement::Right,
            4 => Movement::Up,
            _ => Movement::Down,
        })
        .collect();

    for &movement in &random_values {
        init_drone.move_drone(movement);
    }

    println!("{:?}", init_drone);
}