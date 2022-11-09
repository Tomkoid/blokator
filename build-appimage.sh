# Prepare

if [ ! -f "packages/Blokator.AppDir/usr" ]
then
  mkdir -p packages/Blokator.AppDir/usr/bin
fi

# Build Blokator
cargo build --release

# Copy the binary into packages/Blokator.AppDir/usr/bin/blokator
cp target/release/blokator packages/Blokator.AppDir/usr/bin/blokator

# Build the AppImage using appimagetool
ARCH=x86_64 appimagetool packages/Blokator.AppDir
