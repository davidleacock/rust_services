use std::thread::sleep;
use std::time::Duration;

// fn main() {
//     println!("Starting...");
//
//     let thread_handle1 = thread::spawn(|| {
//         let file_contents = read_from_file1();
//         println!("Contents: {:?}", file_contents);
//     });
//
//     let thread_handle2 = thread::spawn(|| {
//         let file2_contents = read_from_file2();
//         println!("Contents: {:?}", file2_contents);
//     });
//
//     thread_handle1.join().unwrap();
//     thread_handle2.join().unwrap();
// }

#[tokio::main]
async fn main() {
    println!("Starting...");

    let handle = tokio::spawn(async {
        let file1_contents = read_from_file1().await;
        println!("{:?}", file1_contents)
    });

    let handle2 = tokio::spawn(async {
        let file2_contents = read_from_file2().await;
        println!("{:?}", file2_contents)

    });

    let _ = tokio::join!(handle, handle2);
}

async fn read_from_file1() -> String {
    sleep(Duration::new(4, 0));
    println!("{:?}", "Processing from 1");
    String::from("file 1 results.")
}

async fn read_from_file2() -> String {
    sleep(Duration::new(2, 0));
    println!("{:?}", "Processing from 2");
    String::from("file 2 results.")
}
