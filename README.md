# printr

Printr is the smarter `echo` alternative. It is meant as a drop-in replacement of `echo`
and has additional features like automatically guessing the sentiment of the string passed
to it and then outputting it in the corresponding color.

For example:

1. A positive statement will be colored green.

   ```bash
   printr This is quite awesome!!!
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

## Screenshot of printr in action

![printr-image](./assets/screenshot.png)

## Summary

- [printr](#printr)
  - [Screenshot of printr in action](#screenshot-of-printr-in-action)
  - [Summary](#summary)
  - [Getting Started](#getting-started)
    - [Prerequisites](#prerequisites)
    - [Installing](#installing)
  - [Running the tests](#running-the-tests)
    - [Break down into end to end tests](#break-down-into-end-to-end-tests)
    - [And coding style tests](#and-coding-style-tests)
  - [Deployment](#deployment)
  - [Built With](#built-with)
  - [Contributing](#contributing)
  - [Versioning](#versioning)
  - [Authors](#authors)
  - [License](#license)
  - [Others](#others)

## Getting Started

These instructions will get you a copy of the project up and running on
your local machine for development and testing purposes. See deployment
for notes on how to deploy the project on a live system.

### Prerequisites

What things you need to install the software and how to install them

    Give examples

### Installing

A step by step series of examples that tell you how to get a development
env running

Say what the step will be

    Give the example

And repeat

    until finished

End with an example of getting some data out of the system or using it
for a little demo

## Running the tests

Explain how to run the automated tests for this system

### Break down into end to end tests

Explain what these tests test and why

    Give an example

### And coding style tests

Explain what these tests test and why

    Give an example

## Deployment

Add additional notes about how to deploy this on a live system

## Built With

- [Contributor Covenant](https://www.contributor-covenant.org/) - Used
  for the Code of Conduct
- [Creative Commons](https://creativecommons.org/) - Used to choose
  the license

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code
of conduct, and the process for submitting pull requests to us.

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions
available, see the [tags on this
repository](https://github.com/PurpleBooth/a-good-readme-template/tags).

## Authors

See also the list of [contributors](contributors.md) who participated in this project.

## License

This project is licensed under the
[Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0) - see the
[LICENSE.md](LICENSE.md) file for details

## Others

Project bootstrapped using [cookiecutter](https://github.com/IgnisDa/project-cookiecutter)
by IgnisDa.
