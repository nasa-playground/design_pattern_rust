//! # Facade pattern
//!
//! 目的: 既存のシステムの使用方法を簡略化するため、独自のインタフェースを定義する。
//! 因果関係:
//! Facadeはサブシステムの使用方法を簡略化して提供する。Facadeはすべての機能を提供するものではないので、ある機能がクライアントから使用できない可能性がある。
//! クライアントはどのようにサブシステムと協調するのか？: Facadeのインタフェースを通じて協調する
//! 通常はシステム全体にアクセスできるようになるか: 出来ない

struct Database;
impl Database {
    fn get_data() {}
}

struct ElasticSearch;
impl ElasticSearch {
    fn get_data() {}
}

// # Facade
struct Searcher;
impl Searcher {
    fn search() {
        let a = Database::get_data();
        let b = ElasticSearch::get_data();

        // return data
    }
}

pub fn main() {
    Searcher::search();
}
