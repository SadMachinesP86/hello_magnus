# Hello, Magnus!

Yet another sample project exploring the use of Ruby and Rust code in tandem, this time entrusting the exposure of Rust code to Ruby with the 'magnus' crate, and utilizing the 'rake-compiler' gem to automate compilation.

# Usage

Upon entering the project directory, the presence of `.ruby-version` and `.ruby-gemset` will cause RVM to switch the Ruby interpreter to the specified version (prompting you to install it if it isn't already) and generate the Gemset.

Requires `clang`:

```
sudo apt-get install -y clang
```

Bundle, compile, and enter the console:

```
bundle install
bundle exec rake compile
bin/console
```

By inspecting the Rust code you'll see that it defines a global function `hello_magnus`, which simply prints a message:

```
$ bin/console
3.1.3 :001 > hello_magnus
Hello, Magnus!
 => nil
```

# Magnus

See [matsadler](https://github.com/matsadler)'s [repo for the Magnus crate](https://github.com/matsadler/magnus).

# VS Code Workspace

A `.code-workspace` has been added which includes the root directory, and also includes the `ext/hello_magnus` directory housing the Rust code. While this has the odd side effect of displaying the Rust source folder twice, it allows the `rust-analyzer` extension to find the Rust root when it's not at the root of the project directory.
