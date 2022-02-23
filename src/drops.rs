pub mod drops {
    use crate::logger;

    pub fn drops() {
        logger::info("Block1 Begin");
        {
            #[allow(unused_variables)]
            let c = CustomSmartPointer {
                data: String::from("my stuff"),
            };
            logger::info("Block2 Begin");
            {
                #[allow(unused_variables)]
                let d = CustomSmartPointer {
                    data: String::from("some other stuff: d"),
                };
                let e = CustomSmartPointer {
                    data: String::from("some other stuff: e"),
                };
                // e.drop(); // dropを明示的に呼ぶことは許可されていない
                drop(e); // 強制的にDropするときはstd::mem::drop関数を使用する
                logger::info("Custom Smart Pointer created");
            }
            logger::info("Block2 End");
        }
        logger::info("Block1 End");
    }

    pub struct CustomSmartPointer {
        pub data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            logger::info(&format!(
                "Dropping CustomSmartPointer with data `{}`!",
                self.data
            ));
        }
    }
}
