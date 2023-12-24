# passgen
Customizable password generator with rust 

### Compiling and Running
Compiling the code:
```bash
cargo build -r
```
Run the code:
```bash
cd target/release
./passgen
```
or just
```bash
cargo run
```

### Giving the arguments:
After running the program you will see
```
Enter password lenght(max i16):
```
Type in the password lenght as long as itsin i16 boundires (32,767)
Example:
```
10
```

After setting the lenght you will see
```
Customize the password.
Type 1 to include lowercase letters
Type 2 to include uppercase letters
Type 3 to include numbers
Type 4 to include special characters
Usage: 124 includes everything except numbers
```
In this part you specify what characters to include.
Example:
```
124
```

Example output:
```
ujA)?XVD+k
```
