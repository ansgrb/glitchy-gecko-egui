APP_NAME       := glitchy_gecko
PACKAGE_NAME   := com.template.glitchygecko
ANDROID_SDK    ?= $(HOME)/Android/Sdk
BUILD_TOOLS    ?= $(shell ls -d $(ANDROID_SDK)/build-tools/* | sort -V | tail -n 1)
PLATFORM_JAR   ?= $(shell ls -d $(ANDROID_SDK)/platforms/android-* | sort -V | tail -n 1)/android.jar

AAPT2          := $(BUILD_TOOLS)/aapt2
ZIPALIGN       := $(BUILD_TOOLS)/zipalign
APKSIGNER      := $(BUILD_TOOLS)/apksigner
ADB            := $(ANDROID_SDK)/platform-tools/adb

CARGO_FLAGS    ?= --release
RUSTFLAGS      ?= -C link-arg=-Wl,-z,max-page-size=16384

BUILD_DIR      := build
APK_UNALIGNED  := $(BUILD_DIR)/$(APP_NAME).unaligned.apk
APK_ALIGNED    := $(BUILD_DIR)/$(APP_NAME).aligned.apk
APK_FINAL      := $(BUILD_DIR)/$(APP_NAME).apk
KEYSTORE       := $(BUILD_DIR)/debug.keystore

.PHONY: all clean rust-libs package align sign install logcat run

all: sign

rust-libs:
	@mkdir -p $(BUILD_DIR)/apk_staging/lib
	RUSTFLAGS="$(RUSTFLAGS)" cargo ndk \
		-t arm64-v8a \
		-t x86_64 \
		-o $(BUILD_DIR)/apk_staging/lib \
		build $(CARGO_FLAGS)

package: rust-libs
	$(AAPT2) link \
		-o $(APK_UNALIGNED) \
		--manifest AndroidManifest.xml \
		-I $(PLATFORM_JAR)
	cd $(BUILD_DIR)/apk_staging && zip -u -0 -r ../$(APP_NAME).unaligned.apk lib/

align: package
	$(ZIPALIGN) -f -p 4 $(APK_UNALIGNED) $(APK_ALIGNED)

$(KEYSTORE):
	@mkdir -p $(BUILD_DIR)
	keytool -genkeypair \
		-v -keystore $(KEYSTORE) \
		-alias androiddebugkey \
		-keyalg RSA -keysize 2048 -validity 10000 \
		-storepass android -keypass android \
		-dname "CN=Android Debug,O=Android,C=US"

sign: align $(KEYSTORE)
	$(APKSIGNER) sign \
		--ks $(KEYSTORE) \
		--ks-key-alias androiddebugkey \
		--ks-pass pass:android \
		--key-pass pass:android \
		--out $(APK_FINAL) \
		$(APK_ALIGNED)
	@echo "=================================================================="
	@echo "Built signed Pure-Rust APK: $(APK_FINAL)"
	@echo "=================================================================="

install: sign
	$(ADB) install -r $(APK_FINAL)
	$(ADB) shell am start -n $(PACKAGE_NAME)/android.app.NativeActivity

run: install

logcat:
	$(ADB) logcat -s RustStdoutStderr:D GlitchyGecko:D AndroidRuntime:E

clean:
	rm -rf $(BUILD_DIR) target
