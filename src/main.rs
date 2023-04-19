#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

use crate::vga_buffer::WRITER;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    const WIDTH: i32 = 80;
    const HEIGHT: i32 = 24;

    const COLOR_PALETTE: [vga_buffer::Color; 16] = [
        vga_buffer::Color::Black,
        vga_buffer::Color::Blue,
        vga_buffer::Color::Green,
        vga_buffer::Color::Cyan,
        vga_buffer::Color::Red,
        vga_buffer::Color::Magenta,
        vga_buffer::Color::Brown,
        vga_buffer::Color::LightGray,
        vga_buffer::Color::DarkGray,
        vga_buffer::Color::LightBlue,
        vga_buffer::Color::LightGreen,
        vga_buffer::Color::LightCyan,
        vga_buffer::Color::LightRed,
        vga_buffer::Color::Pink,
        vga_buffer::Color::Yellow,
        vga_buffer::Color::White,
    ];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let point = Complex {
                real: x as f64 / WIDTH as f64 * 3.5 - 2.5,
                imag: y as f64 / HEIGHT as f64 * 2.0 - 1.0,
            };
            let i = mandelbrot(point);
            if i == MAX_ITER {
                print!(" ");
            } else {
                WRITER.lock().write_string_color(
                    "*",
                    vga_buffer::ColorCode::new(
                        COLOR_PALETTE[i % COLOR_PALETTE.len()],
                        vga_buffer::Color::Black,
                    ),
                )
            }
        }
        println!();
    }

    panic!("End of _start reached");
}

const MAX_ITER: usize = 100;

struct Complex {
    real: f64,
    imag: f64,
}

fn mandelbrot(point: Complex) -> usize {
    let mut z = Complex {
        real: 0.0,
        imag: 0.0,
    };
    for i in 0..MAX_ITER {
        let z2 = Complex {
            real: z.real * z.real - z.imag * z.imag,
            imag: 2.0 * z.real * z.imag,
        };
        z = Complex {
            real: z2.real + point.real,
            imag: z2.imag + point.imag,
        };
        if z.real * z.real + z.imag * z.imag > 4.0 {
            return i;
        }
    }
    MAX_ITER
}
