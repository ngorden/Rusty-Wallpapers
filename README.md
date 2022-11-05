# WPAPERCLI

This is a cli tool written in Rust to get various wallpapers from [Wallhaven](https://wallhaven.cc).
By default, the API key is blank. This is ok because most wallpapers are able to be downloaded without a key.

## Usage

This CLI tool assumes a couple of things in regards to it's usage, mainly because I wrote it for my system.

1. I wrote it for a *nix system; specifically Linux, but I don't see why it wouldn't work on a Mac
2. It utilizes dmenu to gather user input to determine things like search queries and sorting order

## Disclaimer

I didn't come up with this on my own. [Bugswriter](https://www.youtube.com/channel/UCngn7SVujlvskHRvRKc1cTw) wrote it first, I just converted it to Rust.
