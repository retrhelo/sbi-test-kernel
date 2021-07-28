# sbi-test-kernel
retrhelo <artyomliu@foxmail.com>

[English](./README.md)

## 简介
**sbi-test-kernel**是一个用于测试给定SBI实现的简单内核。我们希望能将其设计得轻量但功能齐全。
使用Rust进行编写的代码提供了良好的代码组织能力，有助于根据需要增加/删除测试。

## Todo清单
- [x] 针对Base拓展的测试
- [ ] 多核情况下的测试，尤其是关于S态核间中断（sPI）的测试
- [ ] 针对Legacy拓展集的测试
- [ ] 针对IPI拓展的测试
- [ ] 一些失败情况下的测试用例