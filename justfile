name := "eikos"
target := "i686-eikos-teleia"
iso := name + ".iso"
iso_dir := "iso"
boot_dir := iso_dir + "/boot"
grub_dir := boot_dir + "/grub"
profile := "debug"
# Thanks rust for having the debug profile called "dev" but the output folder called "debug"
r_profile := if profile == "debug" { "dev" } else { profile }
kernel := "target/" + target + "/" + profile + "/" + name
grub_mk := if os() == "macos" { "i686-elf-grub-mkrescue" } else { "grub-mkrescue" }

default: build iso

build:
    @echo "{{ GREEN }}██████████████████████ Compiling kernel... ██████████████████████{{ NORMAL }}"
    cargo build --profile {{ r_profile }}

iso:
    @echo "{{ YELLOW }}██████████████████████ Building GRUB iso... ██████████████████████{{ NORMAL }}"
    @mkdir -p {{ grub_dir }}
    @cp grub.cfg {{ grub_dir }}/grub.cfg
    @cp {{ kernel }} {{ boot_dir }}/{{ name }}.elf
    {{ grub_mk }} -o {{ iso }} {{ iso_dir }}

clean:
    @echo "{{ RED }}██████████████████████ Deleting built files & iso... ██████████████████████{{ NORMAL }}"
    cargo clean
    @rm -rf {{ iso }} {{ iso_dir }}

run: build iso
    qemu-system-i386 -cdrom {{ iso }} -no-reboot -no-shutdown
