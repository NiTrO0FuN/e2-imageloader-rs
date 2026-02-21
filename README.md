# E2 Image Loader

A desktop application that converts images into a format compatible with Garry's Mod's WireMod E2 digital screens.

## Installation

1. Visit the [GitHub releases page](https://github.com/NiTrO0FuN/e2-imageloader-rs/releases)
2. Download the latest installer for your operating system:
   - `.msi` for Windows
   - `.dmg` for macOS
   - `.AppImage` for Linux
3. Run the installer and follow the setup wizard for windows and macOS, for linux just make the AppImage executable and run it.

## Usage

### Desktop Application

1. **Set Game Path**

   The app will auto-detect the game if it is installed in standard Steam directories.

   If the <img src="public/gmod_logo.svg" height=15px> icon flashes then:
   - Click the Garry's Mod logo in the top-right corner
   - Select your `GarrysMod` installation folder

2. **Add Images**
   - Click "Add image(s)" in the sidebar
   - Select one or more images (PNG, JPEG, WebP, etc.)

3. **Select the desired options**
   - **Quality**: Desired resolution (lower means quicker to load)
   - **Chunk Size**: Represents the amount of files generated (small chunk size means more file, this can help when a server limit the file size a user can load)

4. **Generate**
   - Click the "Generate" button on your selected image

### In Garry's Mod

1. Copy the [E2 script](src/assets/e2-imageloader.txt)
2. Create a new E2 chip in Garry's Mod
3. Paste the script code
4. Click upload
5. Connect a Digital Screen to the "Screen" input
6. Run the command `!load` in chat to load and display the image
