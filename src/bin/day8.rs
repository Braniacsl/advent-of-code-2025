use advent_of_code_2025::aoc_main;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    // Calculate square distance without square root
    // Since were just comparing distances the squares suffice (as opposed to if we wanted to know
    // the real distance)
    fn sq_dist(&self, other: &Point) -> usize {
        self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2)
    }
}

struct Edge {
    u: usize,
    v: usize,
    dist: usize,
}

// We use a disjoint set union to store circuits and to easily merge them (as well as easily
// checking if they're already in the set)
struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>,
    num_components: usize,
}

impl DSU {
    fn new(n: usize) -> Self {
        DSU {
            parent: (0..n).collect(),
            size: vec![1; n],
            num_components: n,
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] != i {
            self.parent[i] = self.find(self.parent[i]);
        }
        self.parent[i]
    }

    fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);

        if root_i != root_j {
            if self.size[root_j] < self.size[root_i] {
                self.parent[root_i] = root_j;
                self.size[root_j] += self.size[root_i];
            } else {
                self.parent[root_j] = root_i;
                self.parent[root_j] = root_i;
                self.size[root_i] += self.size[root_j];
            }

            self.num_components -= 1;
            true
        } else {
            false
        }
    }
}

fn parse(coords: &str) -> Vec<Point> {
    coords
        .lines()
        .map(|line| {
            let parts: Vec<usize> = line.split(',').map(|s| s.parse().unwrap()).collect();
            Point {
                x: parts[0],
                y: parts[1],
                z: parts[2],
            }
        })
        .collect()
}

fn create_edges(points: &Vec<Point>) -> Vec<Edge> {
    let n = points.len();
    // We can pre-allocate the vec size based on the number of edges
    // n * (n - 1): every edge connects to eachother except itself
    // / 2: (x, y) = (y, x)
    let mut edges = Vec::with_capacity(n * (n - 1) / 2);

    // Generate all edges between circuits and sq dist
    for i in 0..n {
        for j in (i + 1)..n {
            edges.push(Edge {
                u: i,
                v: j,
                dist: points[i].sq_dist(&points[j]),
            })
        }
    }

    edges
}

fn solve_p1(coords: &str) -> usize {
    let points = parse(coords);
    let n = points.len();
    let mut edges = create_edges(&points);

    // We combine circuits 1000 times
    let limit = 1000;

    // in case small input
    let actual_limit = limit.min(edges.len());

    // Sort edges by distance so we can just loop through when joining shortest distance
    // We first select_nth_unstable_by_key and then truncate so that we can quickly get the 1000
    // largest items and then fully sort only those
    if actual_limit < edges.len() {
        edges.select_nth_unstable_by_key(actual_limit, |e| e.dist);
        edges.truncate(actual_limit);
    }
    edges.sort_unstable_by_key(|e| e.dist);

    let mut dsu = DSU::new(n);

    for edge in edges {
        // We can simply union without checks because we only attempt to union points in edges
        // (which we already truncated and sorted to 1000)
        dsu.union(edge.u, edge.v);
    }

    let mut circuit_sizes = Vec::new();
    let mut visited_roots = Vec::new();

    for i in 0..n {
        let root = dsu.find(i);
        if !visited_roots.contains(&root) {
            circuit_sizes.push(dsu.size[root]);
            visited_roots.push(root);
        }
    }

    circuit_sizes.sort_unstable_by(|a, b| b.cmp(a));

    circuit_sizes.iter().take(3).product()
}

fn solve_p2(coords: &str) -> usize {
    let points = parse(coords);
    let n = points.len();
    let mut edges = create_edges(&points);

    // Now we sort all bceause we dont have the first 1000 limit
    edges.sort_unstable_by_key(|e| e.dist);

    let mut dsu = DSU::new(n);

    for edge in edges {
        if dsu.union(edge.u, edge.v) {
            // Once theres 1 component left (e.g. solved) we multiply the points used to make the
            // last union
            if dsu.num_components == 1 {
                return points[edge.u].x * points[edge.v].x;
            }
        }
    }

    0
}

aoc_main!(solve_p1, solve_p2);

