use std::time::Duration;

fn main() {
    trpl::block_on(async {
        // An async channel. recv returns a future, which resolves to None once the sending end has been dropped.
        let (tx, mut rx) = trpl::channel();

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        // async move takes ownership of tx. When this block finishes, tx is dropped, and the channel closes.
        let tx_fut = async move {
            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        // while let keeps going as long as recv gives Some. It stops when the channel closes.
        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        trpl::join(tx_fut, rx_fut).await;
    });

    // Try: change the 500 to 1000 milliseconds. Does the program still finish, and how long does it take?
}
