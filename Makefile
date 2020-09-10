

default: build

build: fat.img

run: fat.img 
	qemu-system-x86_64 -bios /usr/share/OVMF/OVMF_CODE.fd -net none -drive file=fat.img,format=raw

netrun: cargo
	qemu-system-x86_64 -bios /usr/share/OVMF/OVMF_CODE.fd -device e1000,netdev=n1 -netdev user,bootfile=efi_adamos.exe,tftp=target/x86_64-pc-windows-msvc/release/,id=n1


cargo: 
	cargo build --release

fat.img: cargo
	dd if=/dev/zero of=fat.img bs=1k count=1440
	mformat -i fat.img -f 1440 ::
	mmd -i fat.img ::/EFI
	mmd -i fat.img ::/EFI/BOOT
	mcopy -i fat.img target/x86_64-pc-windows-msvc/release/efi_adamos.exe ::/