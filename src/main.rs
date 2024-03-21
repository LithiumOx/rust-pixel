// mod clear_canvas;
// use http::response;
// use rand::Rng;
use reqwest::Client;
use serde_json::json;
use serde_json::{Map, Value};
use std::str::FromStr;
// use std::thread::sleep;
// use std::time::Duration;

// const KEY: &str = "XNDLPUHU";
const KEY: &str = "joppe";

struct Pixel {
    x: i32,
    y: i32,
    color: [i32; 3],
}

struct Canvas {
    width: i32,
    height: i32,
    pixels: Vec<Pixel>,
}

struct JSONCanvas {
    status: bool,
    canvas: Canvas,
}

#[tokio::main]
async fn get_canvas(client: &Client) -> Canvas {
    let response: Value = client
        .get("https://canvas.pixelcorp.nl/api/canvas")
        .send()
        .await
        .unwrap()
        .json::<Value>()
        .await
        .unwrap();

    let status: bool = response["succes"].as_bool().unwrap();
    let canvas_map: &Map<String, Value> = response["canvas"].as_object().unwrap();

    let mut pixels: Vec<Pixel> = Vec::new();

    for (key, value) in canvas_map {
        let coords: Vec<i32> = key.split(',').map(|s| i32::from_str(s).unwrap()).collect();
        let color: Vec<i32> = value
            .as_str()
            .unwrap()
            .split(',')
            .map(|s| i32::from_str(s).unwrap())
            .collect();

        if !(color[0] == 255 && color[1] == 255 && color[2] == 255) {
            pixels.push(Pixel {
                x: coords[0],
                y: coords[1],
                color: [color[0], color[1], color[2]],
            });
        }
    }

    let canvas = Canvas {
        width: 200,  // Replace with actual width
        height: 200, // Replace with actual height
        pixels,
    };

    let json_canvas = JSONCanvas { status, canvas };

    let canvas: Canvas = Canvas {
        width: 200,
        height: 200,
        pixels: json_canvas.canvas.pixels,
    };

    canvas
}

async fn clear_canvas(client: &Client, canvas: &Canvas) {
    for pixel in canvas.pixels.iter() {
        if pixel.color != [255, 255, 255] {
            send_pixel(create_pixel(pixel.x, pixel.y, [255, 255, 255]), &client).await;
        }
    }
}

async fn send_pixel(p: Pixel, client: &Client) {
    let pixel = json!({
        "x": p.x,
        "y": p.y,
        "color": p.color,
        "key": KEY
    });
    let _body = client
        .post("https://canvas.pixelcorp.nl/api/single")
        .json(&pixel)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    // sleep(Duration::from_millis(120));
    println!("{}", _body);
}

fn create_pixel(x: i32, y: i32, color: [i32; 3]) -> Pixel {
    Pixel { x, y, color }
}

#[tokio::main]
async fn main() {
    let client = Client::new();
    let height = 200;
    let width = 200;
    // let mut rng = rand::thread_rng();
    let mut x = 0;
    let mut y = 153;
    loop {
        if x % width == 0 && x != 0 {
            x = 0;
            y += 1;
        }
        if y == height {
            break;
        }
        send_pixel(create_pixel(x, y, [255, 255, 255]), &client).await;
        // sleep(Duration::from_millis(80));
        x += 1;
    }

    // let mut rng = rand::thread_rng();
    // use a sine wave to create a pattern and use it for the color
    // let canvas: Canvas = get_canvas(&client);
    // loop {
    //     for pixel in canvas.pixels.iter() {
    //         send_pixel(create_pixel(pixel.x, pixel.y, [255, 255, 255]), &client).await;
    //         // sleep(Duration::from_millis(120));
    //     }
    //     if canvas.pixels.len() == 0 {
    //         println!("Canvas is clear");
    //         break;
    //     }
    // }

    // let mut rng = rand::thread_rng();
    // let mut radius: f32 = 0.0;
    // let mut i: i32 = 0;
    // loop {
    //     // have a modifier for the x and y
    //     let t: f32 = i as f32 / 10.0;
    //     x = (t.sin() * radius) as i32 + 100;
    //     y = (t.cos() * radius) as i32 + 100;
    //     let color: [i32; 3] = [
    //         (rng.gen_range(50..255) as f32 * (i as f32).sin()).abs() as i32,
    //         (rng.gen_range(50..255) as f32 * (i as f32).sin()).abs() as i32,
    //         (rng.gen_range(50..255) as f32 * (i as f32).sin()).abs() as i32,
    //     ];
    //     send_pixel(create_pixel(x, y, color), &client).await;
    //     // send_pixel(create_pixel(x + 1, y - 1, color), &client).await;
    //     // sleep(Duration::from_millis(120));
    //     if radius < 0.0 {
    //         radius = 100.0;
    //     } else {
    //         radius -= 0.1;
    //     }
    //     i += 1;
    // }
}
