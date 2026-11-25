mode ?= release
flags = $(if $(filter release,$(mode)),--release,)
efi = target/x86_64-unknown-uefi/$(mode)/arcadia-boot.efi
esp = build/esp
fw = /usr/share/edk2/x64
vars = build/vars.fd
iso = build/arcadia.iso
efiimg = build/efiboot.img

run: $(esp)/EFI/BOOT/BOOTX64.EFI $(vars)
	qemu-system-x86_64 -machine q35 -m 512M \
		-drive if=pflash,format=raw,readonly=on,file=$(fw)/OVMF_CODE.4m.fd \
		-drive if=pflash,format=raw,file=$(vars) \
		-drive format=raw,file=fat:rw:$(esp) \
		-display gtk -serial stdio -net none

# test the iso the way a real machine would see it
run-iso: $(iso) $(vars)
	qemu-system-x86_64 -machine q35 -m 512M \
		-drive if=pflash,format=raw,readonly=on,file=$(fw)/OVMF_CODE.4m.fd \
		-drive if=pflash,format=raw,file=$(vars) \
		-cdrom $(iso) \
		-display gtk -serial stdio -net none

iso: $(iso)

# always rebuild, cargo skips it if nothing changed
$(esp)/EFI/BOOT/BOOTX64.EFI: FORCE
	cargo build $(flags) -p arcadia-boot
	mkdir -p $(dir $@)
	cp $(efi) $@

# uefi cds boot from a fat image inside the iso
$(iso): $(esp)/EFI/BOOT/BOOTX64.EFI
	rm -rf build/iso $(efiimg)
	mkdir -p build/iso
	dd if=/dev/zero of=$(efiimg) bs=1k count=1440
	mkfs.fat $(efiimg)
	mmd -i $(efiimg) ::/EFI ::/EFI/BOOT
	mcopy -i $(efiimg) $(esp)/EFI/BOOT/BOOTX64.EFI ::/EFI/BOOT/
	cp $(efiimg) build/iso/
	xorriso -as mkisofs -R -J -V ARCADIA \
		-e efiboot.img -no-emul-boot -isohybrid-gpt-basdat \
		-o $@ build/iso

# ovmf wants its own writable vars file
$(vars):
	mkdir -p build
	cp $(fw)/OVMF_VARS.4m.fd $@

clean:
	rm -rf build target

FORCE:
.PHONY: run run-iso iso clean