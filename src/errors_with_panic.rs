use crate::logger;

pub fn errors_with_panic() {

    logger::info("errors_with_panic begin");

    // panic!("crash and burn"); // クラッシュして炎上する

    // C言語では、バッファ外呼び出しになり予定外のデータが読み込まれることがあるが、
    // Rustは実行を中止し、継続を拒みます
    // let v = vec![1, 2, 3];
    // v[99]; // クラッシュして炎上する
    // RUST_BACKTRACE=1 cargo run でバックトレース(Stack Trace情報)を表示する

    logger::info("errors_with_panic end");
}