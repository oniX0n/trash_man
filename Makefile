##
# Project Title
#
# @file
# @version 0.1



# end

install_maid: build_maid
	chmod +x target/release/maid
	cp target/release/maid ~/.local/bin

build_maid:
	cargo build --release --bin maid

install_trash: build_trash
	chmod +x target/release/trash_man
	sudo cp target/release/trash_man /usr/local/bin

build_trash:
	cargo build --release --bin trash_man
