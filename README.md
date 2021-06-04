# blog-protocol-decoder

This is a simple message reader/decoder implemented in Rust. The aim is to illustrate the use of a state machine for decoding, over more naive approaches.

This is the code for a blog post I wrote, which can be found here:

https://boringadv.com/2021/06/04/using-state-machines-for-simplified-message-processing/

The test directory contains 2 binary files with message data. The first file contains a clean message, the second contains two messages mixed with some noise. To run the application (on Unix systems):

$ blog-protocol-decoder < input_file

or 

$ cat input_file | blog-protocol-decoder


