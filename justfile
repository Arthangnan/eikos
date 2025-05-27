name := "eikos"
target := "i686-eikos-teleia"
grub_cfg := "grub.cfg"
iso := name + ".iso"
iso_dir := "iso"
boot_dir := iso_dir + "/boot"
grub_dir := boot_dir + "/grub"

default: build iso

build:
    cargo build --target {{ target }}.json

iso:
    mkdir -p {{ grub_dir }}
    cp {{ grub_cfg }} {{ grub_dir }}/grub.cfg
    cp target/{{ target }}/debug/{{ name }} {{ boot_dir }}/{{ name }}.elf
    grub-mkrescue -o {{ iso }} {{ iso_dir }}

clean:
    cargo clean
    rm -rf {{ iso }} {{ iso_dir }}

run:
    just build iso
    qemu-system-i386 -cdrom {{ iso }} -no-reboot