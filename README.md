# marioprompt

I've used [powerlevel10k](https://github.com/romkatv/powerlevel10k) for many
years now and am quite happy with it, however it has been put into maintenance
mode (with the warning the bugs will go unfixed...) and so I started looking
for alternatives. The most promising option that I saw is probably
[starship](https://starship.rs) but I like the way that my PL10K prompt looks
and I couldn't make starship behave the same way. I've also wanted to give
[fish](https://fishshell.com) (another) try for a while now, so I've decided
to see if I can just build my own prompt that does what I want and not what I
don't.

## usage

Add the following to `~/.config/fish/config.fish`:

```fish
mp init | source
```

## license

Licensed under the GNU GPL version 3.0 or later.

```
marioprompt: my fish shell prompt
Copyright 2025 Mario Finelli

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
```
