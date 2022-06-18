simple password generator app, works from command line. built with rust & clap.

available parameters:

- -h or --help -> help menu
- -V or --version -> print version info
- --min-length {x} -> minimum length of the password
- --max-length {x} -> maximum length of the password
- --no-digit -> don't use digits in password (not recommended)
- --no-punc -> don't use punctuations in password (not recommended)

your password will be generated of length between min_length and max_length and printed out to the console.

you can use the given dockerfile for building a small docker image. it's just a rust standalone binary. you can follow those instructions like me:

https://bxbrenden.github.io

i'm new to rust, so feel free to contribute if you find any errors. suggestions and improvements are always welcome, too.
