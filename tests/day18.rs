use itertools::Itertools;
use std::{
    collections::{HashSet, VecDeque},
    ops::Add,
};

type Delta = (i32, i32, i32);

const INPUT: &str = include_str!("res/18.txt");
const DIRECTIONS: [Delta; 6] = [
    (-1, 0, 0),
    (1, 0, 0),
    (0, -1, 0),
    (0, 1, 0),
    (0, 0, -1),
    (0, 0, 1),
];

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point(i32, i32, i32);

impl Add<Delta> for &Point {
    type Output = Point;

    fn add(self, rhs: Delta) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

fn parse(input: &str) -> Point {
    let coords = input
        .split(",")
        .map(str::parse)
        .filter_map(Result::ok)
        .collect_vec();
    Point(coords[0], coords[1], coords[2])
}

fn exposed(point: &Point, shape: &HashSet<Point>) -> usize {
    DIRECTIONS
        .iter()
        .map(|direction| point + *direction)
        .filter(|p| !shape.contains(p))
        .count()
}

fn surface_area(shape: &HashSet<Point>) -> usize {
    shape.iter().map(|point| exposed(point, shape)).sum()
}

#[derive(Debug)]
struct Bounds {
    low: Point,
    high: Point,
}

impl Bounds {
    fn new(shape: &HashSet<Point>) -> Self {
        // find the two corners that define the bounding box of this shape
        let mut points = shape.iter();
        let mut low = points.next().unwrap().clone();
        let mut high = low.clone();
        for p in points {
            low = Point(low.0.min(p.0), low.1.min(p.1), low.2.min(p.2));
            high = Point(high.0.max(p.0), high.1.max(p.1), high.2.max(p.2));
        }

        Self {
            low: &low + (-1, -1, -1),
            high: &high + (1, 1, 1),
        }
    }

    fn contains(&self, point: &Point) -> bool {
        if !(self.low.0 <= point.0 && point.0 <= self.high.0) {
            return false;
        }
        if !(self.low.1 <= point.1 && point.1 <= self.high.1) {
            return false;
        }
        if !(self.low.2 <= point.2 && point.2 <= self.high.2) {
            return false;
        }
        true
    }

    fn exterior_surface_area(&self) -> i32 {
        let delta_x = (self.high.0 - self.low.0) + 1;
        let delta_y = (self.high.1 - self.low.1) + 1;
        let delta_z = (self.high.2 - self.low.2) + 1;

        (2 * delta_x * delta_y) + (2 * delta_x * delta_z) + (2 * delta_y * delta_z)
    }
}

fn bfs_complement(shape: &HashSet<Point>, bounds: &Bounds) -> HashSet<Point> {
    // this will both let us know which points we've visited and let us compute
    // the final surface area when we're done
    let mut complement = HashSet::new();

    // queue for breadth-first search.
    let mut queue = VecDeque::new();
    queue.push_front(bounds.high.clone());

    // we have to mark the starting point as explored before entering the loop
    complement.insert(bounds.high.clone());

    // breadth-first search
    while let Some(point) = queue.pop_front() {
        // these are all the points to visit next. they must be inside our
        // bounding box and neither part of the original shape nor the
        // complement
        let candidates = DIRECTIONS
            .iter()
            .map(|direction| &point + *direction)
            .filter(|neighbor| bounds.contains(neighbor))
            .filter(|neighbor| !shape.contains(neighbor))
            .filter(|neighbor| !complement.contains(neighbor))
            .collect_vec();

        // once we've generated all of the new points, we can mark them, as
        // visited and then enqueue them to be explored later
        for point in &candidates {
            complement.insert(point.clone());
            queue.push_back(point.clone())
        }
    }

    complement
}

#[test]
fn part1() {
    let shape = INPUT.lines().map(parse).collect();
    let ans = surface_area(&shape);
    println!("Day 18, part 1: {ans}");
    assert_eq!(4536, ans);
}

#[test]
fn part2() {
    // parse the input
    let shape: HashSet<Point> = INPUT.lines().map(parse).collect();

    // create a cuboid which is slightly larger than the original shape
    let bounds = Bounds::new(&shape);

    // We're going to start "filling" this bounded cuboid of space with a
    // complementary set of points, starting at a position we know for certain
    // is outside of the shape. When we've completely filled this exterior
    // volume, we'll know that any remaining spaces which are in bound but
    // members of neither the original shape nor the complementary set must be
    // air pockets.
    let complement = bfs_complement(&shape, &bounds);

    // We can use the surface area of the complementary set to get the exterior
    // surface area of our original shape, but we first need to subtract the
    // exterior surface area of the *complement*. Thankfully, since it's
    // guaranteed to be some kind of cuboid we actually have an explicit
    // equation for it.
    let ans = surface_area(&complement) - bounds.exterior_surface_area() as usize;
    println!("Day 18, part 2: {ans}");
    assert_eq!(2606, ans);
}
