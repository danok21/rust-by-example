use std::fs;//引入rust 标准库中的fs模块
extern crate hex;
// 一 、
/*
fn main() {

    let content = fs::read("./a.txt").unwrap();
    let str = String::from_utf8_lossy(&content);//将向量转换成字符串
    //let str = String::from_utf8(content).expect("Found invalid UTF-8");
    let input= &str[2..];
    assert!(input.len() % 2 == 0);
    println!("{:?}",input);
    let mut out = vec![0;input.len()/2];

    let decoded = hex::decode_to_slice(input,  &mut out).expect("Decoding failed");
    println!("{:X?}",out);
    fs::write("../test.txt", out).unwrap();
}
*/

//  二 、
/*
//090A0B0C
fn main() {
    let text = fs::read_to_string("./a.txt").unwrap();
    println!("{}", text);//读取指定文件，将内容以字符串形式显示
    let content = &text[2..text.len()];//字符串切片
    //let mut bar = text.as_bytes();//将字符串转化成u8字节数组
    //println!("{:x?}", content);//将切片后的字符串输出

    assert!(content.len() % 2 == 0);
    let mut out = vec![0; content.len()/2];//创建一定大小的向量
    let decoded =   hex::decode_to_slice(content, &mut out).expect("Decoding failed");//解码字符串
    println!("{:X?}",out);
    //println!("{:?}", decoded);

    fs::write("../test.txt", out).unwrap();
}
*/


//  三 、

fn main() {
    let text = fs::read_to_string("./a.txt").unwrap();//读取指定文件中的数据，
    println!("Source data is: {:?}", text);//读取指定文件，将内容以字符串形式显示
    let content = &text[2..text.len()];//将字符串进行切片处理
    //let mut bar = text.as_bytes();//将字符串转化成u8字节数组
    //let content = &bar[2..];
    //println!("Processed data is: {:?}", content);//将切片后的字符串输出

    //assert!(content.len() % 2 == 0);//断言
    if content.len() % 2 == 0 {
        let mut out = vec![0; content.len() / 2];//指定向量的长度
        let decoded = hex::decode_to_slice(content, &mut out).expect("Decoding failed");
        println!("{:X?}", out);//以16进制显示
        //println!("{:?}", decoded);
        fs::write("../test.txt", out).unwrap();//将处理后二进制数据写入文件中
        } else {
        print!("Data in the source file is purely a problem！");//错误打印提示信息
    }
}
