# rusty-camino

rusty-camino - A project to play with Rust based distributed engineering and information retrieval.  

I'd like to say that someday we will morph it into a project like Microsoft's Helios paper, but that's just a goal to give us some focus at this point while we play with Rust.  Ref https://blog.acolyer.org/2020/10/26/helios-part-1/

## Requirements

It's using [tusk](https://github.com/rliebz/tusk) to run the automated scripts. Install it from the following:

```sh
brew install rliebz/tusk/tusk
```

## Development

Setup the project by installing all the required dev tools:

```sh
tusk setup
```

Start the dev server:

```sh
tusk dev
```

Start the prod server:

```sh
tusk start
```

Deploy the server:

```sh
tusk deploy
```

Please refer to `tusk.yml` file for more commands.

