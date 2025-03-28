// use std::sync::mpsc;
// use std::thread;
//
// pub fn test_mpsc(){
//     mpsc_channel()
// }
//
//
// //多发送者 单接受者
// fn mpsc_channel(){
//     let ( tx,rx) = mpsc::channel();
//
//     for i in 0..10{
//         thread::spawn(move ||{
//             tx.send(i).unwrap();
//         });
//     }
//
//     for re in rx{
//         println!("receive {}",re);
//
//     }
//
// }
