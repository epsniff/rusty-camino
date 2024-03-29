# rusty-camino

rusty-camino - A project to play with Rust based distributed engineering and information retrieval.  

I'd like to say that someday we will morph it into a project like Microsoft's Helios paper, but that's just a goal to give us some focus at this point while we play with Rust.  Ref https://blog.acolyer.org/2020/10/26/helios-part-1/

## Requirements

It's using [tusk](https://github.com/rliebz/tusk) to run the automated scripts. Install it from the following:

```sh
brew install rliebz/tusk/tusk
```

## Development

Start the server:

```sh
cargo run --bin indexer
```

Create an index:

```sh
http POST localhost:3001/v1/foo2/_create
```

Index a document and query it back out:

```sh
echo '{ "a_foo": "lydia is great", "d_foo": 4171997 }' | http POST localhost:3001/v1/foo2/_upsert
echo 'a_foo:lydia' | http POST localhost:3001/v1/foo2/_search
```