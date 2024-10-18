## Huffman Compression

This package uses the Huffman Compression algorithm to compress the size of files. The compressed file can later be decompressed to get back the initial file. The compression generates a *.huf* file.

It accepts 3 compulsory arguments:
* The mode which accepts 2 possible values **C** and **D**. (Compress and Decompress).
* The path to the input file
* The path to the output file

---
To compress a .txt file run

```bash
cargo run -- C test.txt compressed
```

To decompress the compressed file, simply run

```bash
cargo run -- D compressed.huf output
```
