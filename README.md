# Easy Skia (WIP)
| Platform      | Status | Platform      | Status |
| ----------- | ----------- | ----------- | ----------- |
| OSX      | [![Build Status](https://api.cirrus-ci.com/github/M-Adoo/easy-skia.svg?task=osx_test)](https://cirrus-ci.com/github/M-Adoo/easy-skia)   | IOS | Not support now
| Linux    | [![Build Status](https://api.cirrus-ci.com/github/M-Adoo/easy-skia.svg?task=linux_test)](https://cirrus-ci.com/github/M-Adoo/easy-skia)   | Andorid |Not support now
| Windows  | Not support now | Web | Not support now

Rust idiomatic safe bindings to Google's Skia. The Most of C API is from [mono/skia](https://github.com/mono/skia).

### DEMO

clone this repo and run `cargo run  --example hello` get a demo like below:

![demo](./hello.png)

### TASK

- [ ] API
  - [x] basic 
  - [ ] advance
- [ ] more backend support
  - [x] cpu
  - [x] gpu
  - [ ] pdf
  - [ ] svg
  - [ ] xps
- [x] ci
- [ ] Animation Player

### I need help (Contibuting)

I am very happy the basic api of skia can work, and a simple demo can run on osx. Now I want to support more api and let it work on more platform. Because I can only do this repo in my spare time, so the progress is really not fast.

I'm very glad to see any kind of pr, issues are welcome.