FLAGS="-C prefer-dynamic"
all:
	RUSTFLAGS=${FLAGS} cargo b -r
