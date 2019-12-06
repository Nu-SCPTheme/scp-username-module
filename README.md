# scp-username-module

<a href="https://travis-ci.org/Nu-SCPTheme/scp-username-module"><img src="https://travis-ci.org/Nu-SCPTheme/scp-username-module.svg?branch=master" alt="Travis Build" /></a>

This will be a library that generates the [[user]] module that should compile to Rust or WebASM

## JSON Format

The format for a username module will look like:

```
{
  userid: number;
  username: string;
  profile-picture-url: string | null;
}
```
