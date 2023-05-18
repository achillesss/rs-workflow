use {
    crate::factory::Factory,
    std::{thread::sleep, time},
};

#[test]
fn test_fatory() {
    let f = Factory::build(8);
    let now = time::SystemTime::now();
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
