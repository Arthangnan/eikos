ENTRY(_start)
OUTPUT_FORMAT("elf32-i386")

SECTIONS
{
	. = 1M;

    .multiboot : ALIGN(0x1000)
    {
        KEEP(*(.multiboot))
    }

	.text : ALIGN(0x1000)
	{
		*(.text*)
	}

	.rodata : ALIGN(0x1000)
	{
		*(.rodata*)
	}

	.data : ALIGN(0x1000)
	{
		*(.data*)
	}

	.bss : ALIGN(0x1000)
	{
		*(COMMON)
		*(.bss*)
	}
}