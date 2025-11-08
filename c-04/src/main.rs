fn main() {
    // fn f1() {
    //     let s = vec!["udon".to_string()];
    //     let _t = s.clone();
    //     let _u = s.clone();
    // }
    // f1();
    //
    // fn f2() {
    //     let mut _s = "Govinda".to_string();
    //     _s = "Siddhartha".to_string();
    // }
    // f2();
    //
    // fn f3() {
    //     let mut _s = "Govinda".to_string();
    //     let _t = _s;
    //     _s = "Siddhartha".to_string();
    // }
    // f3();
    //
    // fn f4() {
    //     struct Person {
    //         name: String,
    //         birth: i32,
    //     }
    //     let mut composers = Vec::new();
    //     composers.push(Person {
    //         name: "Palestrina".to_string(),
    //         birth: 1525,
    //     })
    // }
    // f4();
    //
    // fn f5() {
    //     let x = vec![10, 20, 30];
    //     let c = true;
    //     fn f(x: Vec<i32>) {
    //         println!("{:?}", x);
    //     }
    //     fn g(x: Vec<i32>) {
    //         println!("{:?}", x);
    //     }
    //     fn _h(x: Vec<i32>) {
    //         println!("{:?}", x);
    //     }
    //     if c { f(x) } else { g(x) }
    //     // h(x)
    // }
    // f5();
    //
    // fn f6() {
    //     let mut v = Vec::new();
    //     for i in 101..106 {
    //         v.push(i.to_string());
    //     }
    //
    //     let _third = &v[2];
    //     let _fifth = &v[4];
    // }
    // f6();
    // fn f7() {
    //     let v = vec![
    //         "liberte".to_string(),
    //         "egalite".to_string(),
    //         "fraternite".to_string(),
    //     ];
    //
    //     for mut s in v {
    //         s.push('!');
    //         println!("{}", s);
    //     }
    // }
    // f7();

    // fn f8() {
    //     struct Person {
    //         name: Option<String>,
    //         birth: i32,
    //     }
    //     let mut composers = Vec::new();
    //
    //     composers.push(Person {
    //         name: Some("Palestrina".to_string()),
    //         birth: 1523,
    //     });
    //
    //     // let first_name = std::mem::replace(&mut composers[0].name, None);
    //     let first_name = composers[0].name.take();
    //     println!("{:?}", first_name);
    // }
    // f8();

    // fn f9() {
    //     struct Label {
    //         number: u32,
    //     }
    //     fn print(l: Label) {
    //         println!("STAMP: {}", l.number);
    //     }
    //     let l = Label { number: 3 };
    //     print(l);
    //     // println!("My label number is: {}", l.number);
    // }
    // f9();

    use std::rc::Rc;
    fn f10() {
        let s: Rc<String> = Rc::new("shirataki".to_string());
        let t: Rc<String> = s.clone();
        let u: Rc<String> = s.clone();

        assert!(s.contains("shira"));
        assert_eq!(t.find("taki"), Some(5));
        println!("{} are quite chewy, almost bouncy, but lack flavor", u);
    }
    f10()
}
