#![no_std]
#![no_main]

mod vga_buffer;

use core::{
    ops::{Add, Mul},
    panic::PanicInfo,
};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use vga_buffer::{Color, ColorCode, WRITER};

    const WIDTH: i32 = 80;
    const HEIGHT: i32 = 24;

    const COLOR_PALETTE: [Color; 16] = [
        Color::Black,
        Color::Blue,
        Color::LightBlue,
        Color::Cyan,
        Color::LightCyan,
        Color::Green,
        Color::LightGreen,
        Color::Brown,
        Color::Yellow,
        Color::DarkGray,
        Color::LightGray,
        Color::Magenta,
        Color::Pink,
        Color::LightRed,
        Color::Red,
        Color::White,
    ];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let point = Complex::new(
                x as f64 / WIDTH as f64 * 3.5 - 2.5,
                y as f64 / HEIGHT as f64 * 2.0 - 1.0,
            );
            let i = mandelbrot(point);
            if i == MAX_ITER {
                print!(" ");
            } else {
                WRITER.lock().write_byte_color(
                    b'#',
                    ColorCode::new(COLOR_PALETTE[i % COLOR_PALETTE.len()], Color::Black),
                );
            }
        }
        println!();
    }

    panic!("End of _start reached");
}

const MAX_ITER: usize = 128;

#[derive(Debug, Clone, Copy)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    fn new(real: f64, imag: f64) -> Complex {
        Complex { real, imag }
    }

    fn abs(self) -> f64 {
        self.real * self.real + self.imag * self.imag
    }
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex::new(self.real + other.real, self.imag + other.imag)
    }
}

impl Mul for Complex {
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        Complex::new(
            self.real * other.real - self.imag * other.imag,
            self.real * other.imag + self.imag * other.real,
        )
    }
}

fn mandelbrot(c: Complex) -> usize {
    let mut z = Complex::new(0.0, 0.0);
    for i in 0..MAX_ITER {
        z = z * z + c;
        if z.abs() > 4.0 {
            return i;
        }
    }
    MAX_ITER
}
