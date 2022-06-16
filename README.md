> :warning: **I've switched back to [Starship](https://github.com/starship/starship) since my PR to fix vi mode detection for Fish shell released.**

# vifi-prompt

`vifi` is a portmandeau of 'Vi' and 'Fish', because it's a prompt for Fish shell,
primarily focused around showing proper indicators when using Vi key bindings.

![demo](https://github.com/mrjones2014/vifi-prompt/raw/master/demo.gif)

## Install

Requires a [NerdFont](https://github.com/ryanoasis/nerd-fonts).

```
cargo install vifi-prompt
```

Then add this line to your `config.fish`:

```fish
vifi init | source
```

## Notes

This is just something I made for myself. I'm not going to take feature requests,
but will accept well-written PRs.
