# scp-username-module

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
