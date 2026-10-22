# 🦎 Glitchy Gecko — Pure-Rust Immediate-Mode Android Template

> **Architecture:** Pure Rust Immediate-Mode GUI (`egui` + `eframe` + `glow`) on Android `NativeActivity` (`hasCode=false`).  
> **Build System:** Zero-Gradle Raw Android SDK Pipeline (`cargo-ndk` + `aapt2` + `zipalign` + `apksigner`).

---

## 🎯 What Is This?

`glitchy-gecko-egui` is a minimal, blazing-fast boilerplate for writing **100% Rust apps on Android** with **zero Java, zero Kotlin, and zero Gradle**. 

It runs an immediate-mode UI rendered via OpenGL ES (`glow`) inside Android's native `NativeActivity`, achieving **solid 60 FPS** performance with an APK size of only **~11 MB**.

### Highlights
- **Zero Gradle / JVM Overhead:** Builds in seconds using standard Android SDK CLI tools.
- **Android 15 Ready:** Compiles with `-C link-arg=-Wl,-z,max-page-size=16384` for 16 KB page-size compliance.
- **Multi-Architecture:** Automatically builds both `arm64-v8a` (modern phones) and `x86_64` (emulators/Chromebooks).
- **Interactive Boilerplate:** Includes a custom animated sine-wave canvas, button counter, speed slider, and text input.

---

## 🚀 Quick Start

### 1. Prerequisites
Ensure you have the following installed:
```bash
# Rust targets
rustup target add aarch64-linux-android x86_64-linux-android

# cargo-ndk
cargo install cargo-ndk

# Android SDK Build-Tools & Platform
# Make sure $ANDROID_HOME or $HOME/Android/Sdk is set
```

### 2. Build the Signed APK
```bash
make all
```
The resulting APK will be located at:
`build/glitchy_gecko.apk`

### 3. Install & Run on Device
Connect your Android phone with USB debugging enabled, then:
```bash
make install
# or
make run
```

### 4. Live Logs
Stream Rust logs directly from your device:
```bash
make logcat
```

---

## 📁 Project Structure

```
glitchy-gecko-egui/
├── AndroidManifest.xml   # Declares NativeActivity with hasCode="false"
├── Cargo.toml            # Rust dependencies (egui, eframe, android-activity)
├── Makefile              # Clean SDK packaging script (aapt2, zipalign, apksigner)
├── README.md             # This documentation
└── src/
    └── lib.rs            # Egui app logic & android_main entrypoint
```

---

## 🛠️ How to Customize

1. **Add UI Widgets:** Open `src/lib.rs` and modify `eframe::App::update()`.
2. **Change Package Name:** Update `PACKAGE_NAME` in `Makefile` and `package` in `AndroidManifest.xml`.
3. **Add Permissions:** Add `<uses-permission android:name="..." />` to `AndroidManifest.xml`.
