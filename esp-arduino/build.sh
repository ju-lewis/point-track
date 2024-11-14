

if [ "$1" == "flash" ]; then
    echo "Flashing code to board"
    arduino-cli upload --fqbn esp32:esp32:esp32da --port=/dev/ttyUSB0
elif [ "$1" == "scan" ]; then
    echo "Starting serial monitor"
    arduino-cli monitor --port=/dev/ttyUSB0
else 
    echo "Compiling only - Input command was: $1"
    arduino-cli compile --fqbn=esp32:esp32:esp32da
fi

