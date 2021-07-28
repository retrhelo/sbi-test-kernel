# sbi-test-kernel
retrhelo <artyomliu@foxmail.com>

[中文](./README_zh.md)

## Introduction
**sbi-test-kernel** is a simple kernel implementation for testing given SBI 
implementation. It is meant to be lightweight but fully-functional. As written 
in Rust, it provides an organized way to modify codes and add/remove tests if needed.

## Todo List
- [x] Tests for Base Extension
- [ ] Support multi-hart tests, especially with sPI
- [ ] Tests for Legacy Extensions
- [ ] Tests for IPI Extension
- [ ] Some failure tests