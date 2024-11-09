# Build with release profile 
cargo build --release

# Go to the dir where the build executable file is located
cd ./target/debug/

# Give Permission rights to the executable for manage memory, data writes etc.
sudo chmod 777 Burst 

# Run the executable
./Burst
