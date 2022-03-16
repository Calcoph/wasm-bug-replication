//wasm-pack build in /www
//npm run start in /www
import { World } from "dodge-the-ball";
import { memory } from "dodge-the-ball/dodge_the_ball_bg"

const canvas = document.getElementById("dodge-the-ball-canvas");
const world = World.new()
const length = 1000
const height = 300

canvas.height = height
canvas.width = length

const ctx = canvas.getContext("2d")

const draw_balls = () => {
    const ball_amount = world.get_ball_amount()
    const balls = new Float64Array(memory.buffer, world.ball_positions(), 2*ball_amount)
    console.log(balls)
    ctx.beginPath()
    var x = 0.0
    for (let i = 0; i<2*ball_amount;i++) {
        if (i%2 == 1) {
            ctx.fillRect(x, balls[i], 3, 3)
        } else {
            x = balls[i]
        }
    }
}

draw_balls()
world.tick()
draw_balls()
world.tick()
draw_balls()
world.tick()
draw_balls()
world.tick()
draw_balls()
world.tick()
draw_balls()
world.tick()
draw_balls()
world.tick()
draw_balls()
world.tick()
draw_balls()
world.tick()
draw_balls()
world.tick()
draw_balls()
world.tick()
draw_balls()
world.tick()
draw_balls()
world.tick()
draw_balls()
world.tick()
draw_balls()
