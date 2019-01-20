# Useless makefile but asked in the subject

NAME=computer_v1

all: $(NAME)

$(NAME):
	cargo build

check:
	cargo check

test:
	cargo test
	./functionnal_tests.sh

clean:
	@echo "No objects files to clean"

fclean:
	cargo clean

re: fclean all
