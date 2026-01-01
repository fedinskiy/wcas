#change this line if you're targeting some other architecture, e.g. x86_64-unknown-linux-gnu 
ARCH=aarch64-unknown-linux-gnu

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
	cargo build --target $(ARCH)
$(PROD):
	cargo build --release --target $(ARCH)

BINDIR=/usr/bin/
SYSTEMD=/etc/xdg/systemd/user/

.PHONY: install
install:
	sudo install wcas $(BINDIR)
	sudo install wcas.service $(SYSTEMD)
	systemctl --user stop wcas
	systemctl --user start wcas
	systemctl --user enable wcas

.PHONY: uninstall
uninstall:
	systemctl --user stop wcas
	systemctl --user disable wcas
	sudo rm $(BINDIR)wcas
	sudo rm $(SYSTEMD)wcas.service
