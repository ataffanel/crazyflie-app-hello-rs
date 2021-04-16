
# Name of the crate
CRATE_NAME=cfapprs

# enable app support
APP=1
APP_STACKSIZE=300

APP_OBJ += lib$(CRATE_NAME).a
LDFLAGS += -Wl,--allow-multiple-definition

CRAZYFLIE_BASE=crazyflie-firmware
include $(CRAZYFLIE_BASE)/Makefile

lib$(CRATE_NAME).a: FORCE
	cargo +nightly build --release --target thumbv7em-none-eabihf
	cp target/thumbv7em-none-eabihf/release/lib$(CRATE_NAME).a bin/

FORCE: 
