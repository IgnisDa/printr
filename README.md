# printr

`printr` is the smarter `echo` alternative. It is meant as a drop-in replacement of `echo`
and has additional features like automatically guessing the sentiment of the string passed
to it and then outputting it in the corresponding color.

Examples:

1. A positive statement will be colored green.

   ```bash
   printr This is quite awesome!
   ```

2. A negative statement will be colored red.

   ```bash
   printr Danger ahead!
   ```

3. A neutral statement will be colored blue.

   ```bash
   printr Hello World...
   ```

For a full list of options and flags available, run `printr --help`.

Behind the scenes, `printr` performs some light sentiment analysis to guess whether a
statement is positive, negative or neutral.

**NOTE**: _This program does not work well with Windows CMD and Powershell. It works with
[Git-Bash](https://git-scm.com/book/en/v2/Appendix-A%3A-Git-in-Other-Environments-Git-in-Bash)._

## A small demo

![printr-image](./assets/printr-demo.gif)

## Summary

- [printr](#printr)
  - [A small demo](#a-small-demo)
  - [Summary](#summary)
    - [Installing](#installing)
  - [Notes](#notes)
    - [Completions](#completions)
      - [Bash](#bash)
      - [Fish](#fish)
      - [Zsh](#zsh)
    - [Miscellaneous](#miscellaneous)
  - [Contributing](#contributing)
  - [Versioning](#versioning)
  - [Authors](#authors)
  - [License](#license)
  - [Others](#others)

### Installing

`printr` comes packaged with a convenience install script that can run on all bash systems.
Run the following command in your terminal (git-bash for windows users).

```bash
curl https://raw.githubusercontent.com/IgnisDa/printr/main/get-printr.sh -o get-printr.sh
# Warning: always examine scripts downloaded from the internet before running them locally.
bash get-printr.sh
```

The above command can also be used to update your current installation of `printr`.

## Notes

### Completions

The completions for `printr` are installed to `/usr/share/doc/printr` (or equivalent on
Windows). They can be copied to the correct directories to enable tab completions.

**NOTE**: You may need to restart your shell in order for the changes to take effect.

#### Bash

You should have [bash-completion](https://github.com/scop/bash-completion) installed.

```bash
cp /usr/share/doc/printr/printr.bash /etc/bash_completion.d/printr.bash-completion
```

#### Fish

```bash
cp /usr/share/doc/printr/printr.fish $HOME/.config/fish/completions/printr.fish
```

#### Zsh

```bash
cp /usr/share/doc/printr/_printr" "$HOME/.zfunc/_printr
```

You must then add the following line in your `$HOME/.zshrc` before `compinit`:

```bash
fpath+=~/.zfunc
```

For `oh-my-zsh`, you must then enable printr in your `$HOME/.zshrc` plugins

```bash
plugins(
   printr
   ...
   )
```

### Miscellaneous

The output of `printr -h` is different from `printr --help`. The installation script also
sets up man pages that can be accessed using `man printr` on non Windows systems.

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code
of conduct, and the process for submitting pull requests to us.

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions
available, see the [tags on this
repository](https://github.com/IgnisDa/printr/tags).

## Authors

See also the list of [contributors](contributors.md) who participated in this project.

## License

This project is licensed under the
[Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0) - see the
[LICENSE.md](LICENSE.md) file for details

## Others

Project bootstrapped using [cookiecutter](https://github.com/IgnisDa/project-cookiecutter)
by IgnisDa.
