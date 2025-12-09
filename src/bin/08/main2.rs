fn main() {
    let p: Vec<Vec<i32>> = std::fs::read_to_string("test.txt")
        .unwrap()
        .lines()
        .map(|l| l.split(',').map(|n| n.trim().parse().unwrap()).collect())
        .collect();

    let n = p.len();
    let mut edges = vec![];

    for i in 0..n {
        for j in i + 1..n {
            let dist = (0..3).map(|k| (p[i][k] - p[j][k]).pow(2)).sum::<i32>();
            edges.push((dist, i, j));
        }
    }
    edges.sort_unstable();

    let (mut parents, mut sizes) = ((0..n).collect::<Vec<_>>(), vec![1; n]);
    let mut ans = -1;

    for (_, i, j) in edges {
        let mut root_i = i;
        while root_i != parents[root_i] {
            parents[root_i] = parents[parents[root_i]];
            root_i = parents[root_i];
        }

        let mut root_j = j;
        while root_j != parents[root_j] {
            parents[root_j] = parents[parents[root_j]];
            root_j = parents[root_j];
        }

        if root_i != root_j {
            parents[root_i] = root_j;
            sizes[root_j] += sizes[root_i];
            sizes[root_i] = 0;

            ans = p[i][0] * p[j][0];
        }
    }

    println!("{}", ans);
}
