cargo build
cargo bootimage
qemu-system-x86_64 -drive   format=raw,file=target\x86_64-blog_os\debug\bootimage-blog_os.bin  -L "d:\program\qemu" 
