mod frame;
mod help;

use frame::*;
use std::{env, process::exit, thread::sleep, time::Duration};
use help::usage;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        usage();
        exit(1);
    }

    match args[1].as_str() {
        "-d" => {           
            vulkan_use();
            println!("\n- Apply use Vulkan Render priority");
        }
        "-l" => {            
            opengl_use();
            println!("\n- Apply use OpenGL Render priority");
        }
        "-O" => {           
            monitor_auto();
            println!("\n- Apply automatically adjusts to renderer ");
        }
        "-h" | "--help" => usage(),
        other => {
            println!("Unknown option: {}", other);
            exit(1);
        }
    }

    sleep(Duration::from_secs(1));
    println!("\n⚠️ This module is protected by copyright and is\n\
              intended for use by regular users only. Any use of\n\
              this module, including its code, design, or features,\n\
              by other developers without written permission from\n\
              the copyright owner is strictly prohibited.\n\
              ________________________________________________(+)\n");
}