.PHONY: all desktop kernel iso package-example clean
all: desktop

desktop:
	cmake -S . -B build -G Ninja
	cmake --build build

kernel:
	$(MAKE) -f boot/Makefile kernel

iso:
	$(MAKE) -f boot/Makefile iso

package-example:
	python package-manager/src/bloxpm.py verify package-manager/examples/BloxOS-Calculator-1.0.0.blox

clean:
	$(MAKE) -f boot/Makefile clean
	rm -rf build
