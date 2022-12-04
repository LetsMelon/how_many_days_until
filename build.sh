echo "Building binary for multiple targets"

echo "Apple"
cross build -r --target aarch64-apple-darwin
cross build -r --target x86_64-apple-darwin

echo "Linux"
cross build -r --target x86_64-unknown-linux-gnu
cross build -r --target aarch64-unknown-linux-gnu

echo "Windows"
cross build -r --target x86_64-pc-windows-gnu
# cross build -r --target x86_64-pc-windows-msvc
