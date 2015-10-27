#![feature(no_std, lang_items)]
#![no_std]

#![feature(const_fn)]
#![feature(unique)]
#![feature(core_str_ext)]

extern crate rlibc;
extern crate spin;

#[macro_use]
mod vga_buffer;

#[no_mangle]
pub extern fn rust_main() {
    vga_buffer::clear_screen();

    println!("       _ ________      __   ____   _____");
    println!("      | |  ____\\ \\    / /  / __ \\ / ____|");
    println!("      | | |__   \\ \\  / /  | |  | | (___  ");
    println!("  _   | |  __|   \\ \\/ /   | |  | |\\___ \\ ");
    println!(" | |__| | |____   \\  /    | |__| |____) |");
    println!("  \\____/|______|   \\/      \\____/|_____/ ");

    loop{}
}

#[lang = "eh_personality"]
extern fn eh_personality() {}
#[lang = "panic_fmt"]
extern fn panic_fmt() -> ! {loop{}}
