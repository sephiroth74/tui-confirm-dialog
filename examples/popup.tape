# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.
# To run this script, install vhs and run `vhs ./examples/barchart.tape`
Output "examples/popup.gif"
Set Theme "Whimsy"
Set WindowBar Colorful
Set Width 1200
Set Height 800
Hide
Type "cargo run --example=popup"
Enter
Sleep 1s
Show
Sleep 2s
Type "p"
Sleep 3s
Type "p"
Sleep 1s
