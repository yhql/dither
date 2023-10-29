pub struct Filter<const N: usize> {
    pub name: &'static str,
    pub matrix: [[f32; N]; N],
    pub div: f32,
    pub size: usize,
}

impl<const N: usize> Filter<N> {
    const fn new(name: &'static str, div: f32, matrix: [[f32; N]; N]) -> Filter<N> {
        Filter {
            name,
            matrix,
            div,
            size: N,
        }
    }
}

pub const ATKINSON: Filter<5> = Filter::new(
    "atkinson",
    8.,
    [
        [0., 0., 0., 0., 0.],
        [0., 0., 0., 0., 0.],
        [0., 0., 0., 1., 1.],
        [0., 1., 1., 1., 0.],
        [0., 0., 1., 0., 0.],
    ],
);

pub const FS: Filter<3> = Filter::new(
    "floydsteinberg",
    16.,
    [[0., 0., 0.], [0., 1., 7.], [3., 5., 1.]],
);

pub const JJN: Filter<5> = Filter::new(
    "jjn",
    48.,
    [
        [0., 0., 0., 0., 0.],
        [0., 0., 0., 0., 0.],
        [0., 0., 0., 7., 5.],
        [3., 5., 7., 5., 3.],
        [1., 3., 5., 3., 1.],
    ],
);

pub const STUCKI: Filter<5> = Filter::new(
    "stucki",
    42.,
    [
        [0., 0., 0., 0., 0.],
        [0., 0., 0., 0., 0.],
        [0., 0., 0., 8., 4.],
        [2., 4., 8., 4., 2.],
        [1., 2., 4., 2., 1.],
    ],
);
