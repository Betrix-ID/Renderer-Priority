pub fn usage() {
    let script_version = "1.0.1"; 
    println!(r#"♨️ Renderer Priority {} - Automatic GPU Renderer Utility

Usage:
  RendererPriority [OPTION]

Options:
 -O               Auto-detect and apply the best GPU renderer based on device Vulkan support.
 -d               Force-enable Vulkan renderer for enhanced performance and modern rendering pipeline.
  -l               Force-enable OpenGL renderer for compatibility and legacy device support.
  -h, --help   Show this help message and exit.

Description:
  Renderer Priority is a command-line utility designed to manage GPU rendering modes on Android devices.
  It allows users to configure critical system properties to switch between Vulkan and OpenGL backends,
  optimizing rendering performance, UI responsiveness, and hardware acceleration features.
The automatic mode intelligently selects the best renderer based on Vulkan availability on the device.
  If Vulkan is supported and stable, it is prioritized for smoother and more efficient rendering.
  For devices lacking full Vulkan support, the tool will gracefully fall back to OpenGL to ensure
  maximum compatibility and stability.

Examples:
  Auto-detect and apply optimal renderer based on Vulkan support:
      RendererPriority -O

  Force-enable Vulkan renderer regardless of device auto-detection:
      RendererPriority -d

  Force-enable OpenGL renderer for legacy support or troubleshooting:
      RendererPriority -l

  Display help and usage information:
      RendererPriority -h --help

Requirements:
  - Root access is required to apply GPU system properties.
  - The device must support 'adb shell' and necessary shell utilities such as setprop, pm, and cmd.

More info:
  - SurfaceFlinger property flags: https://cs.android.com/search?q=debug.sf.swaprect&sq=&ss=android%2Fplatform%2Fsuperproject      
  - Vulkan system feature reference: https://cs.android.com/search?q=debug.hwui.renderer&ss=android%2Fplatform%2Fsuperproject
  - setprop system usage:
      https://cs.android.com/android/platform/superproject/+/master:system/core/init/property_service.cpp
"#,
    script_version);
}