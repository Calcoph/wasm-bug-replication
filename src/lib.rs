//wasm-pack build in /www
//npm run start in /www
mod utils;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct World {
    balls: Option<Vec<Ball>>,
}

#[wasm_bindgen]
impl World
{
    pub fn new() -> World
    {
        World{
            balls: Some(vec![Ball::new()]), // fun 101: comment this line and uncomment the one below
            //balls: Some(vec![Ball::new(),Ball::new()]),
            /*
             now that you have done that you may think:
             okay, the length of the vector must have some other significance in the code, so it makes sense.
             you are absolutely right, get_ball_amount() returns the lenght of the vector.
             now go to that function and make it return 1 instead of len().
             see how fun? now the lenght of the vector has no effect on anything and it still behaves differently
             */
        }
    }

    pub fn tick(&mut self)
    {
        let mut balls = self.balls.take().unwrap();
        balls[0].x1 += 1.0;
        balls[0].y1 += 1.0;
            
        let (balls, _): (Vec<Ball>, Vec<Ball>) = // here's how to have fun: comment this block and uncomment the one below
            balls.into_iter().partition(|_| true );
        
        /* let balls:Vec<Ball> =
            balls.into_iter().filter(|_| true ).collect(); */


        self.balls = Some(balls);
    }

    pub fn ball_positions(&self) -> *const f64 {
        let balls = match &self.balls {
            Some(x) => x,
            None => panic!("a")
        };
        let mut v = Vec::new();
        for i in balls
        {
            v.push(i.x1);
            v.push(i.y1);
        }
        v.as_ptr()
    }

    pub fn get_ball_amount(&self) -> usize {
        let balls = match &self.balls {
            Some(x) => x,
            None => panic!("a")
        };
        balls.len()

        //1
    }
}

#[wasm_bindgen]
pub struct Ball
{
    pub y1: f64,
    pub x1: f64,
    y2: f64,// comment this for fun
    x2: f64, // comment this for even more fun
    //y3: f64,// want more fun? uncomment this
    //x3: f64 // uncomment this if you want absolute fun
}

#[wasm_bindgen]
impl Ball 
{
    /// angle is in degrees
    pub fn new() -> Ball
    {
        Ball {
            y1: 0.0,
            x1: 0.0,
            y2: 0.0, // comment/uncomment these accordingly so code compiles
            x2: 0.0, // comment/uncomment these accordingly so code compiles
            //y3: 0.0, // comment/uncomment these accordingly so code compiles
            //x3: 0.0 // comment/uncomment these accordingly so code compiles
        }
    }
}
