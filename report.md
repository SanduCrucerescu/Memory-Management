<h1 align= center>Report</h1>
<h4 align= center>1DV512 - Lab assignment 1 </h4>
<h4 align= center>Sandu Crucerescu</h4>

<font size=2 > **Instructions on how to run the program can be fond in the README.md** </font> 

<font size=4> 1. Task objective </font> \
The objective of this assignment was to simulate contiguous memory allocation using all the first-fit, best-fit, and worst-fit strategies. Ath the end of the of the allocation external fragmentation shall be computed.

<font size=4> 2. Completing the task </font> \
Before starting to write the code I started to think on what type of objects I might need, so I decided I need block and each block has an operation for example (allocation), then I had to create the file_api so we can read the input file to read all of the operation and add them into a vector and pass it to the memory management to complete the operation, also in the file_api I wrote the write function to write the output of the algorithms. \
At the beginning of any of the algorithms I first create a block of the size specified in the input file. For the first fit algorithms the program iterated over the operations vector and march the operation to the corresponding function for example if it is `A` then the program iterates of the blocks vector and searches for a block where we can place this allocated block if we find one we place the block with the corresponding start and end addresses and then we refactor the block that we took memory from with the new start address, if we cannot find a empty block then we generate a error.
For the best-fit and worst-fit the idea is similar but we also check the free block size to be equal of larger than the required block size.\
If the operation command is `D` then we have to deallocate a block first I remove the block and then I check if there is a free one next to it and if there is one then I merge them. If we try to deallocate a block than is not allocated then we also generate an error.

<font size=4> 3. Input file structure </font>

1000 // Max number of bytes in memory \
A;0;100 // Allocate 100 bytes to a block identified by 0 \
A;1;100 \
A;2;500 \
D;1 // Deallocate memory for block 1 \
A;3;200 \
D;2 \
O // Produce an intermediate output file \
A;4;1500 \
D;4 \
D;5
