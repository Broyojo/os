#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // display mandelbrot set
    const WIDTH: i32 = 80;
    const HEIGHT: i32 = 24;

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
                print!("*");
            }
        }
        println!("");
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
