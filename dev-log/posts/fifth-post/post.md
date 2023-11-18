
# Chess board pieces with FEN 
<br>

I have implemented the board config and the pieces, represented in FEN. And have added two unit tests for the FEN parser, I find it quite interesting having to add the tests in the same file that the code is and I think I prefer it to the way unit tests are handled in Java.<br>  
When I first attempted to run my tests, they weren't being picked up by cargo, after a brief Google, I learned that if the code is not referenced in the lib.rs or main.rs files, they won't be compiled. And when i did reference the code, red lines galore, here i was thinking it was smooth sailing but a lot of my code wasn't compiling and i had to rework a bit of stuff.<br>  
But I have come out of this with more knowledge and a better understanding of Rust. A lot to learn still but I am having fun
