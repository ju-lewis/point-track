

if ("$args" -eq "flash") {
    echo "Flashing code to board"
    arduino-cli upload --fqbn esp32:esp32:esp32da --port=COM3
} else {
    echo "Compiling only - Input command was: $args"
    arduino-cli compile --fqbn=esp32:esp32:esp32da
}


