#change this line if you're targeting some other architecture, e.g. x86_64-unknown-linux-gnu 
ARCH=armv7-unknown-linux-gnueabihf

DEBUG=target/$(ARCH)/debug/wcas
PROD=target/$(ARCH)/release/wcas

# change to $(PROD) for production
EXEC=$(DEBUG) 

NOT_ARCHIVED_PATH=target/package/
INST_DIR=$(NOT_ARCHIVED_PATH)wcas
PACKAGE=target/wcas.tar.xz

all: $(PACKAGE)

$(PACKAGE): $(EXEC)
	mkdir -p $(INST_DIR)
	cp $(EXEC) $(INST_DIR)/wcas
	cp makefile $(INST_DIR)
	cp install/wcas.service $(INST_DIR)
	tar -caf $(PACKAGE) --directory=$(NOT_ARCHIVED_PATH) wcas

$(DEBUG):
	cross build --target $(ARCH)
$(PROD):
	cross build --release --target $(ARCH)

BINDIR=/usr/bin/
SYSTEMD=/etc/systemd/system/

.PHONY: install
install:
	install wcas $(BINDIR)
	install wcas.service $(SYSTEMD)
	systemctl stop wcas
	systemctl start wcas
	systemctl enable wcas
