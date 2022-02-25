use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crate::logger;

pub fn message_passing() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();

        // logger::info(&format!("send message: {}", val));
        // valの所有権はsend関数によって奪われているので再使用できない
        // これにより、誤って値を書き換えたり再使用するのを防ぐことができる
    });

    // recv()は、メッセージを受信するまでメインスレッドの処理を待機する
    let recieved = rx.recv().unwrap();

    logger::info(&format!("Got: {}", recieved));

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec!["hi", "from", "the", "thread"];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
    });

    for received in rx {
        logger::info(&format!("Got: {}", received));
    }

    let (tx, rx) = mpsc::channel();

    // 転送機をクローンできる
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec!["hi", "from", "the", "thread"];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    let tx2 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec!["more", "messages", "for", "you"];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });
    // この時点だとtxが使用されていないので、このブロックを終了することができない。
    // ので複製したtxを使用するだけでなく、最終的に複製前の元のtxtも使用する必要がある。
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        tx.send("final message").unwrap();
    });

    // tx.send("final message'").unwrap(); // スレッド外でsendしても何も起こらない

    for received in rx {
        logger::info(&format!("Got: {}", received));
    }
}
