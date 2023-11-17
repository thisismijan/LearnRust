
# Chess board pieces with FEN 
<br>

I have started to work on modelling the chess pieces and my research has led me to the [**Forsyth-Edwards Notation**](https://www.chess.com/terms/fen-chess) or FEN for short. This is a single string that can describe the positions of all the pieces on a chess board<br>  
Additionally, it says which players turn it is, can indicate whether a player can castle or has a valid en passant target. And also I've learnt of the Halfmove clock rule, which enforces a 50-move draw rule, and the fullmove clock which counts the total number of moves in a game, i have created new issues for the rules<br>  
I have opened a [**draft PR**](https://github.com/thisismijan/LearnRust/pull/14), still needs work as I am yet to implement castling, enpassant and the newly discovered halfmove and fullmove clock which are required to have a functioning FEN parser.<br>  
This will also be a good time to figure out how unit tests work in Rust
