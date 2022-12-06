echo "Building binary for multiple targets"

version=$( awk /^version/ Cargo.toml | cut -d '"' -f2 )
version="${version//./_}"

echo "Apple"
cross build -r --target aarch64-apple-darwin && cp target/aarch64-apple-darwin/release/how_many_days_until how_many_days_until && tar -zcvf how_many_days_until_"$version"_darwin_aarch64.tar.gz how_many_days_until && rm how_many_days_until
# cross build -r --target x86_64-apple-darwin && cp target/x86_64-apple-darwin/release/how_many_days_until how_many_days_until && tar -zcvf how_many_days_until_"$version"_darwin_x86_64.tar.gz how_many_days_until && rm how_many_days_until

echo "Linux"
# cross build -r --target x86_64-unknown-linux-gnu && cp target/x86_64-unknown-linux-gnu/release/how_many_days_until how_many_days_until && tar -zcvf how_many_days_until_"$version"_linux_x86_64.tar.gz how_many_days_until && rm how_many_days_until
# cross build -r --target aarch64-unknown-linux-gnu && cp target/aarch64-unknown-linux-gnu/release/how_many_days_until how_many_days_until && tar -zcvf how_many_days_until_"$version"_linux_aarch64.tar.gz how_many_days_until && rm how_many_days_until

echo "Windows"
# cross build -r --target x86_64-pc-windows-gnu && cp target/x86_64-pc-windows-gnu/release/how_many_days_until.exe how_many_days_until.exe && tar -zcvf how_many_days_until_"$version"_windows_gnu_x86_64.tar.gz how_many_days_until.exe && rm how_many_days_until.exe
# cross build -r --target x86_64-pc-windows-msvc

echo "Calculate and save checksums"
shasum -a 256 how_many_days_until_"$version"_darwin_aarch64.tar.gz > checksums.txt
shasum -a 256 how_many_days_until_"$version"_darwin_x86_64.tar.gz >> checksums.txt
shasum -a 256 how_many_days_until_"$version"_linux_x86_64.tar.gz >> checksums.txt
shasum -a 256 how_many_days_until_"$version"_linux_aarch64.tar.gz >> checksums.txt
shasum -a 256 how_many_days_until_"$version"_windows_gnu_x86_64.tar.gz >> checksums.txt
