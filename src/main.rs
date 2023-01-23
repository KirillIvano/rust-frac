#![allow(clippy::pedantic)]

use clap::{App, Arg};
use std::{process::Command, thread::sleep, time::Duration};

use num::{traits::Pow, Complex};

const ITERATIONS_CAP: i32 = 1000;

struct Fractal {
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
    width: i32,
    height: i32,
}

type HeatMatrix = Vec<Vec<i32>>;
const DENSITY: &[u8] = "1234567890".as_bytes();

impl Fractal {
    fn get_symb_from_iter(iter: i32) -> char {
        let byte_val = match iter {
            1 => DENSITY[9],
            2..=5 => DENSITY[8],
            6..=10 => DENSITY[7],
            11..=20 => DENSITY[6],
            21..=40 => DENSITY[5],
            41..=60 => DENSITY[4],
            61..=100 => DENSITY[3],
            101..=200 => DENSITY[2],
            201..=400 => DENSITY[1],
            _ => DENSITY[0],
        };

        char::from(byte_val)
    }

    fn render_matrix(&self, vec: &HeatMatrix) {
        for row in vec {
            let frac_row = row
                .iter()
                .map(|&x| Self::get_symb_from_iter(x))
                .collect::<String>();

            println!("{}", frac_row)
        }
    }

    fn calculate_matrix(&self) -> HeatMatrix {
        let initial_w = self.x_max - self.x_min;
        let initial_h = self.y_max - self.y_min;

        let mut res: HeatMatrix = vec![];

        for row in 0..self.height {
            let mut row_vec: Vec<i32> = vec![];

            for col in 0..self.width {
                let curr_x = (col as f32 / self.width as f32) * initial_w + self.x_min;
                let curr_y = (row as f32 / self.height as f32) * initial_h + self.y_min;

                let val = Self::get_point_iterations(curr_x, curr_y);

                row_vec.push(val);
            }

            res.push(row_vec);
        }

        res
    }

    fn get_point_iterations(x: f32, y: f32) -> i32 {
        let mut z = Complex::new(0f32, 0f32);
        let c = Complex::new(x, y);

        for iter in 0..ITERATIONS_CAP {
            z = z.pow(2.0) + c;

            if z.norm() > 2.0 {
                return iter;
            }
        }

        ITERATIONS_CAP
    }
}

fn cls() {
    Command::new("cls");
}

struct Args {
    zoom: f32,
    dx: f32,
    dy: f32,
}

fn get_args() -> Args {
    let matches = App::new("simple fractal")
        .arg(
            Arg::with_name("zoom")
                .default_value("1.0")
                .long("zoom")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("dx")
                .default_value("0.0")
                .long("dx")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("dy")
                .default_value("0.0")
                .long("dy")
                .required(false)
                .takes_value(true),
        )
        .get_matches();

    return Args {
        zoom: matches.value_of("zoom").unwrap().parse().unwrap(),
        dx: matches.value_of("dx").unwrap().parse().unwrap(),
        dy: matches.value_of("dy").unwrap().parse().unwrap(),
    };
}

fn main() {
    let args = get_args();

    let target: (f32, f32) = (args.dx, args.dy);
    let initial: (f32, f32) = (0.0, 0.0);

    let target_zoom: f32 = args.zoom;

    const ITERATIONS: i32 = 100;

    for i in 0..ITERATIONS {
        let diff_part = i as f32 / ITERATIONS as f32;

        let current_zoom: f32 = 1.0 + (target_zoom - 1.0) * diff_part;

        let diff_x = (target.0 - initial.0) * diff_part;
        let diff_y = (target.1 - initial.1) * diff_part;

        cls();

        let frac = Fractal {
            x_min: -2.0 / current_zoom + diff_x,
            x_max: 2.0 / current_zoom + diff_x,
            y_min: -2.0 / current_zoom + diff_y,
            y_max: 2.0 / current_zoom + diff_y,
            width: 100,
            height: 50,
        };
        let field = frac.calculate_matrix();
        frac.render_matrix(&field);

        let millis = Duration::from_millis(20);
        sleep(millis);
    }
}
