use queue::Queue;

mod queue;

fn main() {
    let mut q = Queue {
        older: Vec::new(),
        younger: Vec::new(),
    };
    q.push('P');
    q.push('D');
    assert_eq!(q.pop(), Some('P'));
    q.push('X');

    let (older, younger) = q.split();
    assert_eq!(older, vec!['D']);
    assert_eq!(younger, vec!['X']);

    let mut q2 = Queue::new();
    let mut q3 = Queue::new();

    q2.push("CAD");
    q3.push(0.74);

    q2.push("BTC");
    q3.push(13764.0);
}

struct Broom {
    name: String,
    height: u32,
    health: u32,
    position: (f32, f32, f32),
    intent: BroomIntent,
}

#[derive(Copy, Clone)]
enum BroomIntent {
    FetchWater,
    DumpWater,
}

fn chop(b: Broom) -> (Broom, Broom) {
    let mut boom1 = Broom {
        height: b.height / 2,
        ..b
    };

    let mut boom2 = Broom {
        name: boom1.name.clone(),
        ..boom1
    };

    boom1.name.push_str(" I");
    boom2.name.push_str(" II");

    (boom1, boom2)
}
