use futures::future::join_all;

use {
    crate::factory::Factory,
    std::{
        thread::sleep,
        time::{self as stdTime, Duration},
    },
    tokio::time,
};

async fn count_down(n: u64) {
    for i in 0..10 {
        time::sleep(Duration::from_millis(100 * n)).await;
        println!("[{}] count {}", n, i);
    }
    println!("[{}] count finish", n);
}

#[test]
fn test_tokio() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let mut v = Vec::new();
        for i in 0..10 {
            // v.push(tokio::task::spawn(count_down(i)));
            v.push(count_down(i));
        }

        join_all(v).await;
    });
}

#[test]
fn test_fatory() {
    let f = Factory::build(8);
    let now = stdTime::SystemTime::now();
    let max_num = 1_000_000_u64;
    for i in 0..max_num {
        f.deliver(move || {
            if (i + 1) % 100000 == 0 {
                println!("deliver job {}", i + 1);
            }
            // sleep(time::Duration::from_millis(10));
        });
    }

    f.close();
    println!("==========");
    println!("cost: {:?}", now.elapsed().unwrap());
    println!("==========");

    sleep(time::Duration::from_secs(1));
    f.close();
    f.close();
    sleep(time::Duration::from_secs(1));
}
