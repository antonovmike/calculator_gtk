# calculator_gtk

![calculator_gtk](https://github.com/antonovmike/calculator_gtk/blob/main/calculator_gtk.gif)

You need  libgtk-3-dev library to run the application
For Linux machines:
```bash
sudo apt-get install libgtk-3-dev
```
or
```bash
sudo apt install libgtk-3-dev build-essential
```

--------------------

Get text from entry
entry.text()
Each time you click numeric button, method text() gives you the whole entry comtent, for example:
```bash
Click Button 0 -> 0
Click Button 1 -> 01
Click Button 2 -> 012
```
etc.

--------------------

f64 wors

--------------------

ADD:

Listen for keyboard events

--------------------

FIX:

Get rid of data.txt

Try to get string from Entry::builder()

make Entry scrollable!

Set rounding precision

--------------------

Does not work with nightly
```bash
rustc --version
rustup default stable
```