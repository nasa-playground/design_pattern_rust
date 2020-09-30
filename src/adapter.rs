//! # Adapter pattern
//!
//! 目的: 修正できない既存のStructを特定のインタフェースに適合させる。
//!
//! 因果関係: 既存のオブジェクトをインタフェースに制限されることなく、新たなクラス構造に取り込める

trait Interface {
    fn hoge(&self);
    fn fuga(&self);
}

struct A {}

impl Interface for A {
    fn hoge(&self) {}

    fn fuga(&self) {}
}

struct B {}

impl Interface for B {
    fn hoge(&self) {}

    fn fuga(&self) {}
}

fn function(i: &Interface) {
    // do something
}

fn main() {
    let a = A {};
    let a = B {};

    function(a);
    function(b);
}
