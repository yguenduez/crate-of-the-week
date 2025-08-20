# Just - 20th of August

Just is just more than a makefile. It's a tool that helps you automate tasks.

## Installation

To install Just, you can use the following command:

```sh
cargo install just
```

## Usage

To use Just, you can create a `justfile` in your project directory and define recipes in it. For example:

File: `justfile`

```just
my_recipe:
    echo "Hello, world!"
```

You can then execute the recipe by running:

```sh
just my_recipe
```

And you can list all your recipes by running:

```sh
just --list
```

### Arguments

You can also have arguments, where you can use the argument with `{{argument}}`

```text
my_recipe my_arg:
    echo "Hello, {{my_arg}}!"
```

### Importing other recipes files

Have another recipe file called `other_recipes.just`:

File: `other_recipes.just`

```just
another_recipe:
    echo "Hello, from another recipe!"
```

You can import this file in your main `justfile`:

File: `justfile`

```just
import 'other_recipes.just'

my_recipe:
    just another_recipe
```

### Advanced things

For example, if you want to read you `.env` file and declare variables, you can do this within your `justfile`:

File: `justfile`

```just
# This will load the .env file, it contains IP_ADDRESS and USERNAME
set dotenv-load

# This is a variable
DEPLOY_PATH := "/my-path"

copy_artifacts:
  rsync -avz -e "ssh -i ../ssh/id_rsa" my_db.db $USERNAME@$IP_ADDRESS:{{ DEPLOY_PATH }}
```

## More

You can not only write shell recipes. Just supports shell/powershell commands. You can write recipes in [different languages](https://just.systems/man/en/shebang-recipes.html):
Just have a look at the [official documentation](https://https://just.systems/).
