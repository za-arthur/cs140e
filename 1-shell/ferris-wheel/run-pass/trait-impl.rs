// FIXME: Make me pass! Diff budget: 25 lines.
#[derive(Debug)]
enum Duration {
    MilliSeconds(u64),
    Seconds(u32),
    Minutes(u16)
}

impl PartialEq for Duration {
    fn eq(&self, other: &Duration) -> bool {
        let mut a: u64 = 0;
        let mut b: u64 = 0;

        match self {
            &Duration::MilliSeconds(ms) => a = ms,
            &Duration::Seconds(s) => a = s as u64 * 1000,
            &Duration::Minutes(m) => a = m as u64 * 1000 * 60,
        };

        match (other) {
            &Duration::MilliSeconds(ms) => b = ms,
            &Duration::Seconds(s) => b = s as u64 * 1000,
            &Duration::Minutes(m) => b = m as u64 * 1000 * 60,
        };

        a == b
    }
}

fn main() {
    assert_eq!(Duration::Seconds(120), Duration::Minutes(2));
    assert_eq!(Duration::Seconds(420), Duration::Minutes(7));
    assert_eq!(Duration::MilliSeconds(420000), Duration::Minutes(7));
    assert_eq!(Duration::MilliSeconds(43000), Duration::Seconds(43));
}
