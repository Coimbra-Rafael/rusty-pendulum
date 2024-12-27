mod vector;

use vector::Vector;

fn main() {
    println!("Hello, world!");
}

struct Pendulum {
    //T or is the position of the pendulum.
    origin: Vector,

    //This vector is the position of the ball.
    position: Vector,

    //This is the angle of the pendulum.
    angle: f32,

    angular_velocuty: f32,
    angular_acceleration: f32,

    r: f32, //The length of the pendulum.
    m: f32, //The mass of the ball.
    g: f32, //The gravity.
}

impl Pendulum {
    fn new(x: f32, y: f32, r: f32) -> Pendulum {
        Pendulum {
            //We need to set the origin of the pendulum.
            origin: Vector::new(x, y),

            //We'll set the position when we update the pendulum.
            //For now we'll set it to a default value.
            position: Vector::new(0.0,0.0),
            angle: 1.0,                 //We'll set the angle to 1.0 radian.
            angular_velocuty: 0.0,      //The pendulum is not moving in the beginning.
            angular_acceleration: 0.0,  //The pendulum is not acceleration in the beginning.
            r: r,
            m: 1.0, //The mass of the ball ios 1.0 for this example
            g: 1.5, //The gravity is 0.5 for this example, but play with it.
        }
    }

    fn update(&mut self) {
        //We use the pendulum equation to calculate the angular acceleration.
        self.angular_acceleration = -1.0 * self.g * self.angle.sin() / self.r;

        //The angular velocity is the angular velocity plus the angular acceleration.
        self.angular_velocuty += self.angular_acceleration;

        //The angle is the angle plus angular velocity.
        self.angle += self.angular_velocuty;

        //The posisition is the polar coordinates translated to cartesian coordinates.
        self.position
            .set(self.r * self.angle.sin(), self.r * self.angle.cos());

        //The final position of the ball in the canvas
        //pendulum plus the position vector.
        self.position.add(&self.origin);
    }

    fn draw() {}
}
