/* 
 notes critical user :
 - You may not change, modify or even put this script into your personal project without written permission from the script creator (official author) 
 
 Telegarm    : @UnixeID | Chenel : @Yeye_PID
 Github      : Betrix-ID
 Consultasi  : betrix322@gmail.com
 
 the date this script was written : 11 - Mei - 2025
*/

use std::{process::Command, thread::sleep, time::Duration};


fn shell(message: &str) {
    let command = format!(
        "cmd notification post -S bigtext -t '♨️ Render Priority ' 'Tag' '{}' > /dev/null 2>&1",
        message
    );

    if let Err(e) = Command::new("sh").arg("-c").arg(command).status() {
        eprintln!("Failed to execute shell command: {}", e);
    }
}

fn cmd(prop: &str, value: &str) {
    let command = format!("setprop {} {}", prop, value);
    let output = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .output();

    if let Err(e) = output {
        eprintln!("Failed to set {} {}: {}", prop, value, e);
    }
}

fn get_vulkan() -> i32 {
    let output = Command::new("sh")
        .arg("-c")
        .arg("pm list features | awk -F'=' '/feature:android.hardware.vulkan.level/ {print $2}'")
        .output()
        .expect("Failed to read Vulkan feature");

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.trim().parse().unwrap_or(-1)
}

pub fn monitor_auto() {
    let get = get_vulkan();

    if get >= 1 {
        println!(
            "\tDescription:
    Applies system properties to enable Vulkan as the primary graphics renderer.
    These settings optimize rendering performance, improve UI responsiveness,
    and ensure full utilization of hardware acceleration with Vulkan support."
        );

        let props = [
            ("debug.hwui.renderer", "vulkan"),
            ("debug.composition.type", "dyn"),
            ("debug.hwui.render_dirty_regions", "true"),
            ("debug.hwui.use_partial_updates", "true"),
            ("debug.sf.protect_mm", "1"),
            ("debug.sf.hw", "1"),
            ("debug.sf.swaprect", "1"),
            ("debug.hwc.dynThreshold", "2.00"),
            ("debug.mdpcomp.maxlayer", "5"),
            ("debug.compbypass.enable", "1"),
        ];

        for (prop, value) in props {
            cmd(prop, value);
        }
        sleep(Duration::from_secs(2));
        shell("Success: Applying Use Vulkan Renderer..");
    } else {
        println!(
            "\tDescription:
    Applies system properties to switch the graphics renderer to OpenGL.
    This configuration is ideal for devices that lack proper Vulkan support
    or need compatibility with legacy rendering paths. It can also help
    resolve stability issues on certain hardware setups."
        );

        let props = [
            ("debug.hwui.renderer", "opengl"),
            ("debug.composition.type", "dyn"),
            ("debug.hwui.render_dirty_regions", "true"),
            ("debug.hwui.use_partial_updates", "true"),
            ("debug.sf.protect_mm", "1"),
            ("debug.sf.hw", "1"),
            ("debug.sf.swaprect", "1"),
            ("debug.hwc.dynThreshold", "2.00"),
            ("debug.compbypass.enable", "1"),
            ("debug.mdpcomp.maxlayer", "3"),
        ];

        for (prop, value) in props {
            cmd(prop, value);
        }
        sleep(Duration::from_secs(2));
        shell("Success: Applying Use Opengl Renderer..");
    }
}

pub fn vulkan_use() {
    println!(
        "\tDescription:
    Applies system properties to enable Vulkan as the primary graphics renderer.
    These settings optimize rendering performance, improve UI responsiveness,
    and ensure full utilization of hardware acceleration with Vulkan support."
    );

    let props = [
        ("debug.hwui.renderer", "vulkan"),
        ("debug.composition.type", "dyn"),
        ("debug.hwui.render_dirty_regions", "true"),
        ("debug.hwui.use_partial_updates", "true"),
        ("debug.sf.protect_mm", "1"),
        ("debug.sf.hw", "1"),
        ("debug.sf.swaprect", "1"),
        ("debug.hwc.dynThreshold", "2.00"),
        ("debug.mdpcomp.maxlayer", "5"),
        ("debug.compbypass.enable", "1"),
    ];

    for (prop, value) in props {
        cmd(prop, value);
    }
    sleep(Duration::from_secs(2));
    shell("Success: Applying Use Vulkan Renderer..");
}

pub fn opengl_use() {
    println!(
        "\tDescription:
    Applies system properties to switch the graphics renderer to OpenGL.
    This configuration is ideal for devices that lack proper Vulkan support
    or need compatibility with legacy rendering paths. It can also help
    resolve stability issues on certain hardware setups."
    );

    let props = [
        ("debug.hwui.renderer", "opengl"),
        ("debug.composition.type", "dyn"),
        ("debug.hwui.render_dirty_regions", "true"),
        ("debug.hwui.use_partial_updates", "true"),
        ("debug.sf.protect_mm", "1"),
        ("debug.sf.hw", "1"),
        ("debug.sf.swaprect", "1"),
        ("debug.hwc.dynThreshold", "2.00"),
        ("debug.compbypass.enable", "1"),
        ("debug.mdpcomp.maxlayer", "3"),
    ];

    for (prop, value) in props {
        cmd(prop, value);
    }
    sleep(Duration::from_secs(2));
    shell("Success: Applying Use Opengl Renderer..");
}
