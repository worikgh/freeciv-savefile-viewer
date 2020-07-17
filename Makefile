TARGET=target/debug/save_file_analysis
SRC=src/main.rs
${TARGET}: ${SRC}
	cargo build
