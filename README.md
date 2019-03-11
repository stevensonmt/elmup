I noticed updating to the `http 2.0` package was modestly trickier than expected.

You just have to delete your old `elm.json` and then run `elm install` for each dependency. This tool automates that process slightly on Linux systems only. 

I hope to make it a little more sophisticated in time, providing more security checks and only re-installing packages for which doing so is necessary. Right now if you run the `elmup` binary and then refuse to allow `elm init` to make a new elm.json, you'll have to manually move the `elm.json.bak` file to `elm.json` to get back your old setup. Currently `elmup` overwrites any existing `elm.json.bak`, so any interruption in the `elm install` process causing you to restart the program could lead to loss of your original `elm.json`. 

IANAL, so whatever license is inherited by the dependencies I used in this project can be assumed to be the license under which you may use/modify/distribute `elmup`. Questions, comments, feature requests all welcome. Thanks!
