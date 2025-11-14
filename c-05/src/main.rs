fn main() {
    // println!("Hello, world!");

    // fn f1() {
    //     use std::collections::HashMap;
    //
    //     type Table = HashMap<String, Vec<String>>;
    //
    //     fn show(table: Table) {
    //         for (artist, works) in table {
    //             println!("works by {}:", artist);
    //
    //             for work in works {
    //                 println!("    {}", work);
    //             }
    //         }
    //     }
    //     let mut table: Table = HashMap::new();
    //     table.insert(
    //         "fruits".to_string(),
    //         vec!["apple".to_string(), "banana".to_string()],
    //     );
    //     table.insert(
    //         "vegetables".to_string(),
    //         vec!["carrot".to_string(), "lettuce".to_string()],
    //     );
    //     show(table);
    // }
    //
    // f1();

    // fn f2() {
    //     use std::collections::HashMap;
    //
    //     type Table = HashMap<String, Vec<String>>;
    //
    //     fn show(table: &Table) {
    //         for (artist, works) in table {
    //             println!("works by {}:", artist);
    //
    //             for work in works {
    //                 println!("    {}", work)
    //             }
    //         }
    //     }
    //     let mut table: Table = HashMap::new();
    //     table.insert(
    //         "fruits".to_string(),
    //         vec!["apple".to_string(), "banana".to_string()],
    //     );
    //     table.insert(
    //         "vegetables".to_string(),
    //         vec!["carrot".to_string(), "lettuce".to_string()],
    //     );
    //     show(&table);
    // }
    //
    // f2();

    // fn f3() {
    //     use std::collections::HashMap;
    //
    //     type Table = HashMap<String, Vec<String>>;
    //     fn sort_works(table: &mut Table) {
    //         for (_, works) in table {
    //             works.sort();
    //         }
    //     }
    //     let mut table: Table = HashMap::new();
    //     table.insert(
    //         "fruits".to_string(),
    //         vec!["apple".to_string(), "banana".to_string()],
    //     );
    //     table.insert(
    //         "vegetables".to_string(),
    //         vec!["carrot".to_string(), "lettuce".to_string()],
    //     );
    //     sort_works(&mut table);
    // }
    //
    // f3();

    fn f4() {
        let x = 10;
        let r = &x;
        assert!(*r == 10);

        let mut y = 32;
        let m = &mut y;
        *m += 32;
        assert!(*m == 64);
    }

    f4();

    fn f5() {
        struct Anime {
            name: &'static str,
            bechdel_pass: bool,
        }
        let aria = Anime {
            name: "Aria: The Animation",
            bechdel_pass: true,
        };
        let anime_ref = &aria;
        assert_eq!(anime_ref.name, "Aria: The Animation");
        assert_eq!((*anime_ref).name, "Aria: The Animation");
    }

    fn f6() {
        let mut v = vec![1973, 1968];
        v.sort();
        // (&mut v).sort();
    }

    fn f7() {
        let x = 10;
        let y = 20;
        let mut r = &x;

        if true {
            r = &y;
        }

        assert!(*r == 10 || *r == 20);
    }

    fn f8() {
        struct Point {
            x: i32,
            y: i32,
        }

        let point = Point { x: 1000, y: 729 };
        let r: &Point = &point;
        let rr: &&Point = &r;
        let rrr: &&&Point = &rr;

        assert_eq!(rrr.y, 729);
    }

    fn f9() {
        let x = 10;
        let y = 10;

        let rx = &x;
        let ry = &y;

        let rrx = &rx;
        let rry = &ry;

        assert!(rrx <= rry);
        assert!(rrx == rry);

        assert!(rx == ry);
        assert!(!std::ptr::eq(rx, ry));
        assert!(rx == *rrx);
    }

    fn f10() {
        fn factorial(n: usize) -> usize {
            (1..n + 1).product()
        }

        let r = &factorial(6);
        assert_eq!(r + &1009, 1729);
    }

    fn f11() {
        {
            let r;
            {
                let x = 1;
                r = &x;
                assert_eq!(*r, 1)
            }
        }
    }

    fn f12() {
        let v = vec![1, 2, 3];
        let r = &v[1];
    }

    fn f13() {
        static mut STASH: &i32 = &128;
        fn f(p: &'static i32) {
            unsafe {
                // ここで、p と STASH の生存期間が異なる
                STASH = p;
            }
        }

        static WORTH_POINTING_AT: i32 = 1000;
        f(&WORTH_POINTING_AT);

        fn g<'a>(p: &'a i32) {}
        let x = 10;
        g(&x);

        // fn h(p: &'static i32) {}
        // let x = 10;
        // h(&x);

        fn smallest(v: &[i32]) -> &i32 {
            let mut s = &v[0];

            for r in &v[1..] {
                if *r < *s {
                    s = r;
                }
            }
            s
        }

        // let s;
        // {
        //     let parabola = [9, 4, 1, 0, 1, 4, 9];
        //     s = smallest(&parabola);
        // }
        // assert_eq!(*s, 0);

        {
            let s;
            let parabola = [9, 4, 1, 0, 1, 4, 9];
            s = smallest(&parabola);
            assert_eq!(*s, 0);
        }
    }

    fn f14() {
        #[derive(Debug)]
        struct S<'a> {
            r: &'a i32,
        }

        // let s;
        // {
        //     let x = 10;
        //     s = S { r: &x };
        //     println!("{:?}", s);
        // }
        // assert_eq!(*s.r, 10);

        struct D<'a> {
            s: S<'a>,
        }
    }

    fn f15() {
        struct S<'a, 'b> {
            x: &'a i32,
            y: &'b i32,
        }

        let x = 10;
        let r;
        {
            let y = 20;
            {
                let s = S { x: &x, y: &y };
                r = s.x;
            }
        }
        println!("{}", r);
    }
}
