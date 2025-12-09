fn main() {
    let p: Vec<Vec<i32>> = std::fs::read_to_string("test.txt")
        .unwrap()
        .lines()
        .map(|l| l.split(',').map(|n| n.trim().parse().unwrap()).collect())
        .collect();
    let n = p.len();
    let mut e = vec![];
    for i in 0..n {
        for j in i + 1..n {
            e.push((
                (0..3).map(|k| (p[i][k] - p[j][k]).pow(2)).sum::<i32>(),
                i,
                j,
            ));
        }
    }
    e.sort_unstable();

    let (mut g, mut s) = ((0..n).collect::<Vec<_>>(), vec![1; n]);
    for k in 0..1000 {
        let (_, mut i, mut j) = e[k];
        while i != g[i] {
            g[i] = g[g[i]];
            i = g[i];
        }
        while j != g[j] {
            g[j] = g[g[j]];
            j = g[j];
        }
        if i != j {
            g[i] = j;
            s[j] += s[i];
            s[i] = 0;
        }
    }

    s.sort_unstable();
    println!("{}", s.iter().rev().take(3).product::<usize>());
}
