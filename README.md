# Digidecs
Digidecs is the online form for committee- and board members to submit their receipts.

# Prerequesites
- [Yarn](https://classic.yarnpkg.com/en/)
- [Rust](https://www.rust-lang.org/)
- [Node](https://nodejs.org/en), = ^22.0 (with the strong advice of using nvm :).


# Local Installation
## start the server
1. move to the `server` folder
2. run `cargo build`.
3. run `cargo run -- --config config.json --dry-run`. if on production, remove the `--dry-run` part.

## start the front-end
1. Move to the `frontend` folder.
2. There, run `yarn install`
3. After this, run `yarn run dev`
You now have a hot-reloadable dev enviroment. Happy hacking!

