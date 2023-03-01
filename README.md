# calculator_gtk

[Python version](https://github.com/antonovmike/calculator_gtk_2)

![calculator_gtk](https://github.com/antonovmike/calculator_gtk/blob/main/calculator_gtk.gif)

**Install gtk-3-dev**
You need  libgtk-3-dev library to run the application
For Linux machines:
```bash
sudo apt install libgtk-3-dev build-essential
```
or
```bash
sudo apt-get install libgtk-3-dev
```

Build and run app:
```bash
cargo build --release
/target/release
./calculator_gtk
```

**Does not work with nightly**
```bash
rustc --version
rustup default stable
```

**Get text from entry**
```rust
entry.text()
```
Each time you click numeric button, method text() gives you the whole entry comtent, for example:
```bash
Click Button 0 -> 0
Click Button . -> 0.
Click Button 2 -> 0.2
Click Button + -> 0.2 +
```
etc.

**TODO:**
- Listen for keyboard events
- Scrollable Entry
- Set rounding precision (1.9869999999999999 -> 1,987)
- Documentation

**FIXED**
+ Tests
+ Negative numbers (works only if SUBTRACT is "-" and spaces after first number and before last insert manualy)
+ Fix wrong input bug (one operand or one operand and operator)
+ Application ID com.github.gtk-rs.repository_name
