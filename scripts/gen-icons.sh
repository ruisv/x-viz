#!/bin/bash
# Generate all app icon sizes from icon.svg using sharp-cli + iconutil (macOS)
set -e

ICONS_DIR="$(cd "$(dirname "$0")/../src-tauri/icons" && pwd)"
SVG="$ICONS_DIR/icon.svg"
ICONSET="$ICONS_DIR/icon.iconset"

echo "Generating PNGs from SVG..."

# Tauri required sizes
npx --yes sharp-cli -i "$SVG" -o "$ICONS_DIR/icon.png"         resize 1024 1024
npx --yes sharp-cli -i "$SVG" -o "$ICONS_DIR/32x32.png"        resize 32 32
npx --yes sharp-cli -i "$SVG" -o "$ICONS_DIR/128x128.png"      resize 128 128
npx --yes sharp-cli -i "$SVG" -o "$ICONS_DIR/128x128@2x.png"   resize 256 256

# Windows Store logos
npx --yes sharp-cli -i "$SVG" -o "$ICONS_DIR/Square30x30Logo.png"    resize 30 30
npx --yes sharp-cli -i "$SVG" -o "$ICONS_DIR/Square44x44Logo.png"    resize 44 44
npx --yes sharp-cli -i "$SVG" -o "$ICONS_DIR/Square71x71Logo.png"    resize 71 71
npx --yes sharp-cli -i "$SVG" -o "$ICONS_DIR/Square89x89Logo.png"    resize 89 89
npx --yes sharp-cli -i "$SVG" -o "$ICONS_DIR/Square107x107Logo.png"  resize 107 107
npx --yes sharp-cli -i "$SVG" -o "$ICONS_DIR/Square142x142Logo.png"  resize 142 142
npx --yes sharp-cli -i "$SVG" -o "$ICONS_DIR/Square150x150Logo.png"  resize 150 150
npx --yes sharp-cli -i "$SVG" -o "$ICONS_DIR/Square284x284Logo.png"  resize 284 284
npx --yes sharp-cli -i "$SVG" -o "$ICONS_DIR/Square310x310Logo.png"  resize 310 310
npx --yes sharp-cli -i "$SVG" -o "$ICONS_DIR/StoreLogo.png"          resize 50 50

# macOS .icns via iconutil
echo "Generating .icns..."
rm -rf "$ICONSET"
mkdir -p "$ICONSET"
npx --yes sharp-cli -i "$SVG" -o "$ICONSET/icon_16x16.png"        resize 16 16
npx --yes sharp-cli -i "$SVG" -o "$ICONSET/icon_16x16@2x.png"     resize 32 32
npx --yes sharp-cli -i "$SVG" -o "$ICONSET/icon_32x32.png"        resize 32 32
npx --yes sharp-cli -i "$SVG" -o "$ICONSET/icon_32x32@2x.png"     resize 64 64
npx --yes sharp-cli -i "$SVG" -o "$ICONSET/icon_128x128.png"      resize 128 128
npx --yes sharp-cli -i "$SVG" -o "$ICONSET/icon_128x128@2x.png"   resize 256 256
npx --yes sharp-cli -i "$SVG" -o "$ICONSET/icon_256x256.png"      resize 256 256
npx --yes sharp-cli -i "$SVG" -o "$ICONSET/icon_256x256@2x.png"   resize 512 512
npx --yes sharp-cli -i "$SVG" -o "$ICONSET/icon_512x512.png"      resize 512 512
npx --yes sharp-cli -i "$SVG" -o "$ICONSET/icon_512x512@2x.png"   resize 1024 1024
iconutil -c icns "$ICONSET" -o "$ICONS_DIR/icon.icns"
rm -rf "$ICONSET"

# Windows .ico (embed multiple sizes via sharp)
echo "Generating .ico..."
# sharp-cli doesn't do .ico natively, use a small Node script
node -e "
const sharp = require('sharp');
const fs = require('fs');
const svg = '$SVG';
(async () => {
  // ICO can embed 256x256 max as PNG
  const buf = await sharp(svg).resize(256, 256).png().toBuffer();
  // Minimal ICO container for a single 256x256 PNG entry
  const dir = Buffer.alloc(6 + 16);
  dir.writeUInt16LE(0, 0);     // reserved
  dir.writeUInt16LE(1, 2);     // type: icon
  dir.writeUInt16LE(1, 4);     // count
  dir.writeUInt8(0, 6);        // width (0 = 256)
  dir.writeUInt8(0, 7);        // height (0 = 256)
  dir.writeUInt8(0, 8);        // palette
  dir.writeUInt8(0, 9);        // reserved
  dir.writeUInt16LE(1, 10);    // planes
  dir.writeUInt16LE(32, 12);   // bits per pixel
  dir.writeUInt32LE(buf.length, 14); // size
  dir.writeUInt32LE(22, 18);   // offset (6 + 16)
  fs.writeFileSync('$ICONS_DIR/icon.ico', Buffer.concat([dir, buf]));
  console.log('icon.ico created');
})();
"

echo "All icons generated!"
ls -lh "$ICONS_DIR"
