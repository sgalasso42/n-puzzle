# n-puzzle
A n-puzzle solver, made in Rust, working with an IDA* using admissible heuristics<br/>
This is a 42 school project <strong>[Final grade: 121%]</strong>

---
## Demo
<img src="demo.gif" height="500"/>

---
## Setup
If you do not have Rust
```
> sh install-rust.sh
```
Then you can build the project
```
> cargo build --release
```

---
## Usage
```
> ./target/release/n-puzzle [FLAGS] [OPTIONS]
```
### Flags
```
-h, --help          Prints help information
-s, --solvable      Generates a solvable puzzle
-u, --unsolvable    Generates an unsolvable puzzle
-V, --version       Prints version information
-v, --visual        Make a visualisation of the result
```
### Options
```
-f, --file <file>         Path to the file to read from
-c, --heuristic <name>    Heuristic selection, choose from 'manhattan', 'euclidian', 'hamming' and 'conflict'
-i, --iterations <nb>     The number of iterations
-n, --size <nb>           The size of the puzzle
-t, --type <type>         Alternative g(x) and f(x), choose from 'greedy' and 'uniform'
```

---
## Contributors
<table>
  <tr>
    <td align="center"><a href="https://github.com/sgalasso42"><img src="https://avatars2.githubusercontent.com/u/38636967?v=4" width="100px;" alt=""/><br /><sub><b>Simon Galasso</b></sub></a><br />
    <td align="center"><a href="https://fr.linkedin.com/in/nicolasvienot"><img src="https://avatars0.githubusercontent.com/u/44903069?v=4" width="100px;" alt=""/><br /><sub><b>Nicolas Viénot</b></sub></a><br />
  </tr>
</table>
