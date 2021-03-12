# Oxid OS
# This is the makefile used for building and managing builds of Oxid OS.]
#
# Author:  Ardalan Ahanchi
# Date:    Spring 2020
# License: GPLv2

# Constants and Variables ######################################################

# Path for the build directory
BUILD_DIR = build

# Path for the source directory
SRC_DIR = src

# Path for the configurations directory
CONFIG_DIR = config

# Directory for the cargo output (traditionally target)
CARGO_BUILD_DIR = $(BUILD_DIR)/cargo_build

# The name of the target file (used since it's a custom enviornment)
CARGO_TARGET_FILE = $(CONFIG_DIR)/x86_64-unknown-oxid.json

# Path for the cargo built binary (output of cargo). Depending on the current
# target, the build sub directory will be different. So keep that in mind.
boot: CARGO_BIN_SUBDIR = release
build: CARGO_BIN_SUBDIR = release
iso: CARGO_BIN_SUBDIR = release
test: CARGO_BIN_SUBDIR = debug
debug: CARGO_BIN_SUBDIR = debug
CARGO_BIN_DIR = $(CARGO_BUILD_DIR)/$(notdir $(basename $(CARGO_TARGET_FILE)))/$(CARGO_BIN_SUBDIR)

# The name of the cargo binary
CARGO_BIN_NAME = liboxid_os.a

# The unit test feature flag as defined by the Cargo.toml and used in the code.
test: CARGO_UNIT_TEST_FLAG = --features "unit-test"
debug: CARGO_UNIT_TEST_FLAG = --features "unit-test"

# The following targets don't include the debug symbols.
boot: CARGO_RELEASE_FLAG = --release
build: CARGO_RELEASE_FLAG = --release
iso: CARGO_RELEASE_FLAG = --release
test: CARGO_RELEASE_FLAG =
debug: CARGO_RELEASE_FLAG =

# The default port for the launched remote GDB instance
GDB_PORT = 1996

# Path (full name) of the output ISO
ISO_PATH = $(BUILD_DIR)/oxid.iso

# Path for the script used for linking
LINKER_SCRIPT = $(CONFIG_DIR)/linker.ld

# Directory for the compiled assembly output.
ASM_BUILD_DIR = $(BUILD_DIR)/asm_build

# All the assembly source code (find all files which have a .asm extension).
ASM_SRC = $(shell find $(SRC_DIR) -type f -name '*.asm')

# Find the output
ASM_OBJ = $(patsubst $(SRC_DIR)/%.asm, $(ASM_BUILD_DIR)/%.o, $(ASM_SRC))

# The names for all the rust source files.
RUST_SRC += $(shell find $(SRC_DIR) -type f -name '*.rs')

# Targets ######################################################################

# Run the cargo build and create the binary (dependencies are all rs files)
$(CARGO_BIN_DIR)/$(CARGO_BIN_NAME): $(RUST_SRC)
	@mkdir -p $(BUILD_DIR)
	@mkdir -p $(CARGO_BUILD_DIR)

    # Build the actual rust code.
	cargo build --target $(CARGO_TARGET_FILE) \
				--target-dir $(CARGO_BUILD_DIR) \
				-Z build-std=core,alloc,compiler_builtins \
				$(CARGO_RELEASE_FLAG) \
				$(CARGO_UNIT_TEST_FLAG)

# Build the assembly source files with NASM for all assembly files
$(ASM_OBJ): $(ASM_SRC)
	# Create the directory for this assembled object if needed.
	mkdir -p $(shell echo $@ | rev | cut -d "/" -f 2- | rev)

	# Actually compile the assembly code.
	nasm -f elf64 $(patsubst $(ASM_BUILD_DIR)/%.o, $(SRC_DIR)/%.asm, $@) -o $@

# Link the object files using ld and the given config file to make a binary
$(BUILD_DIR)/oxid.bin: $(ASM_OBJ) $(CARGO_BIN_DIR)/$(CARGO_BIN_NAME)
	ld --nmagic \
	    --output $@ \
	    --script $(LINKER_SCRIPT) \
	    $(ASM_OBJ) $(CARGO_BIN_DIR)/$(CARGO_BIN_NAME)

# Create the GRUB ISO file from the binary (also generate the grub config file)
$(ISO_PATH): $(BUILD_DIR)/oxid.bin $(CONFIG_DIR)/grub.cfg
	# Make some directories required for creating an ISO.
	@mkdir -p $(BUILD_DIR)
	@mkdir -p $(BUILD_DIR)/iso_temp
	@mkdir -p $(BUILD_DIR)/iso_temp/boot
	@mkdir -p $(BUILD_DIR)/iso_temp/boot/grub

	# Copy the linked oxid kernel to the correct directory.
	cp $< $(BUILD_DIR)/iso_temp/boot

	# Copy the grub configuration file to the correct folder.
	cp $(CONFIG_DIR)/grub.cfg $(BUILD_DIR)/iso_temp/boot/grub/

	# Build the actual bootable image using grub-mkrescue
	grub-mkrescue --output=$@ $(BUILD_DIR)/iso_temp

	# Remove the temporary build files.
	@rm -r $(BUILD_DIR)/iso_temp

# Actions ######################################################################

# Make the following targets phony so it doesn't mix it with files
.PHONY: clean clean_bins build boot test

# For when there are no arguments provded, just compile.
.DEFAULT_GOAL: build

# Build the actual kernel (without making the iso).
build: clean_bins $(BUILD_DIR)/oxid.bin $(CONFIG_DIR)/grub.cfg

# Build the ISO file.
iso: build $(ISO_PATH)

# Boot from the ISO in qemu virtual machine
boot: iso
	qemu-system-x86_64 -cdrom $(ISO_PATH)

# Create a test ISO (with unit tests), and run in virtual machine.
# It will also launch a debugger and connect it to qemu.
# It additionally runs eqmu in monitor mode to facilitate debugging.
# https://en.wikibooks.org/wiki/QEMU/Monitor
test: clean_bins $(ISO_PATH)
	qemu-system-x86_64 -monitor stdio -d int,cpu_reset -no-reboot -cdrom $(ISO_PATH)

debug: clean_bins $(ISO_PATH)
	qemu-system-x86_64 -S -gdb tcp::$(GDB_PORT) -monitor stdio -d int,cpu_reset -no-reboot -cdrom $(ISO_PATH)


# Remove the build file (and target if it was made by an IDE).
clean:
	@rm -R -f $(BUILD_DIR)
	@rm -R -f target

# Remove the built binaries and object files so we can rebuild.
clean_bins:
	@rm -R -f $(BUILD_DIR)/oxid.bin
	@rm -R -f $(ISO_PATH)
	@rm -R -f $(ASM_BUILD_DIR)
	@rm -R -f $(CARGO_BIN_DIR)/$(CARGO_BIN_NAME)
