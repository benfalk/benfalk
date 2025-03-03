---
CREATED_AT: 2025-03-03T00:46:54.397763984+00:00
TOPIC: snippet
lang: rust
title: playing with git2
---

## Playing With Git2

{{#aa (snippet) ../../../crates/git2_learning/src/lib.rs#mod?name=test}}

This snippet opens the repository for this journal, fetches the
"HEAD" revision.  From this you can get the id of the commit, which
is the sha of that particular commit.  Once you have a commit it
becomes a drill-down to a particular file, which is what I'm interested
in.  Because the files can be anything it appears, if I want to get
the contents as a string the "blob" it needs to be converted.

I've been trying to learn how to use the [git2-rs] bindings for my
[anchors aweigh] project.  It has been extremely slow going as the
documentation is extremely sparse and hard to follow.  The biggest
breakthroughs I've made so far is these:

- [examples] have been the best source so far
- a lot of online resources are very out of date
- `Repository::discover` is handy for opening a repo

[git2-rs]: https://github.com/rust-lang/git2-rs/tree/master
[anchors aweigh]: https://github.com/benfalk/anchors-aweigh
[examples]: https://github.com/rust-lang/git2-rs/tree/master/examples
