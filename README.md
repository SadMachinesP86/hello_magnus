# Hello, Magnus!

Yet another sample project exploring the use of Ruby and Rust code in tandem, this time entrusting the exposure of Rust code to Ruby with the 'magnus' crate, and utilizing the 'rake-compiler' gem to automate compilation.

# Usage

Requires `clang`: 

```
sudo apt-get install -y clang
```

Bundle gems with `bundle install`. Then compile with `bundle exec rake compile`. Compilation places the dynamic library `hello_magnus.so` at `lib/hello_magnus`, which can then be required with `require_relative 'hello_magnus/hello_magnus'`.

Enter the console - with the library required for you - with `bin/console`. By inspecting the Rust code you'll see that it defines a global function `hello_magnus`, which simply prints a message:

```
bin/console
3.1.3 :001 > hello_magnus
Hello, Magnus!
 => nil
```

# VS Code Workspace

A `.code-workspace` has been added which includes the root directory, and also includes the `ext/hello_magnus` directory housing the Rust code. While this has the odd side effect of displaying the Rust source folder twice, it allows the `rust-analyzer` extension to find the Rust root when it's not at the root of the project directory.
