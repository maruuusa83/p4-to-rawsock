// Copyright (c) 2018 Daichi Teruya @maruuusa83
// This project is released under the MIT license
// https://github.com/maruuusa83/p4-to-rawsock/blob/master/LISENCE
extern crate gcc;

fn main() {
    gcc::Config::new()
                .file("src/c/ioctl.c")
                .include("src")
                .compile("libioctl.a");
}

