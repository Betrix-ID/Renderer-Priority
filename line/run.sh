#!/system/bin/sh
# Checking ID shell
if [ "$(id -u)" -ne 0 ] && [ "$(id -u)" -ne 2000 ]; then
	printf "[ Eror |@Yeye_nat(Yeye)]\n"
   exit 1
fi
#Chking cpu.abi
     if [ ! -f /sdcard/line/system/target ]; then
	    architecture=$(getprop ro.product.cpu.abi)
	  if [ "$architecture" = "arm64-v8a" ]; then
		cp /sdcard/line/system/target/release/arm64 /sdcard/line/use
	elif [ "$architecture" = "armeabi-v7a" ]; then
		cp /sdcard/line/system/target/release/arm /sdcard/line/use
	fi
  fi
#smart notifications
shell() {
    sor="$1"
    cmd notification post -S bigtext -t '♨️ Renderer Priority' 'Tag' "$sor" > /dev/null 2>&1
}
	
# Style display Terminal
set +x
    echo
    echo "     ☆================================☆"
    echo
    echo "       ~ Description. Renderer Priority...... "
    echo
    echo "       - Author                 :  @UnixeID"
    echo "       - Point                    :  1.0 "
    echo "       - Release               :  12 - Mei - 2025"
    echo "       - Name Shell         :  Renderer Priority"
    echo
    echo "    |_______________________________________|"
    echo "    \______________________________________/"
    echo
    echo "   Priority Renderer Priority Custem. "
    echo
    sleep 2
     rm -rf /data/local/tmp/use
     cp /sdcard/line/use /data/local/tmp
     chmod +x /data/local/tmp/use
     if [ "$1" = "-d" ]; then
          shell "Applying Renderer Priority for Vulkan use 1-2 seconds..."
          /data/local/tmp/use -d
     elif [ "$1" = "-L" ]; then
          shell "Applying Renderer Priority for OpenGL 1-2 seconds..."
          /data/local/tmp/use -l
     elif [ "$1" = "-O" ]; then
          shell "Applying Renderer Priority adjust use Renderer 1-2 seconds..."
          /data/local/tmp/use -O
     elif [ "$1" = "-h" ] || [ "$1" = "--help" ]; then
           /data/local/tmp/use --help
        else
          printf "Failed to apply requested profile. Unknown option: %s\n" "$1"
         fi
set +x

