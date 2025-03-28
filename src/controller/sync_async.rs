trait Sequencer {
    fn generate(&self) -> Vec<i32>;
}

struct PlainSequencer {
    bound: i32,
}

impl PlainSequencer {
    async fn generate_async(&self) -> Vec<i32> {
        let mut res = vec![];
        for i in 0..self.bound {
            res.push(i);
            // tokio::time::sleep(Duration::from_millis(100)).await;
        }
        res
    }
}

impl Sequencer for PlainSequencer {
    fn generate(&self) -> Vec<i32> {
        // 第一种
        futures::executor::block_on(async { self.generate_async().await })

        //第二种
        // tokio::task::spawn_blocking(|| {
        //     self.generate_async().await
        // } )
    }
}

pub async fn test_async() {
    let sequencer = PlainSequencer { bound: 3 };
    println!("{:#?}", 111);

    let vec = sequencer.generate();
    println!("{:#?}", vec);
}
