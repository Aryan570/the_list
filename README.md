
# The List

It's a TODO list application but you use it from the Terminal.
Currently, it is fairly slow for a CLI tool. I am working to speed it up. One way might be to move up to the paid version of mongoDB, but I'm sure there are some optimizations I can do.


## Get Started
Create new account as :

```bash
  cargo run -- name <your_name>
```

List all the items

```bash
  cargo run -- list
```

Add new item to the list

```bash
  cargo run -- add "<item>"
```

Delete an item from list

```bash
  cargo run -- delete <item_number>
```
## Contributing

Contributions are always welcome!

See `contributing.md` for ways to get started.

Please adhere to this project's `code of conduct`.
