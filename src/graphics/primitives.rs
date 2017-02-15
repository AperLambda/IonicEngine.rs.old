#![allow(dead_code)]
use na::{ Point3, Point2 };

struct Quad3d
{
    p1: Point3<f64>,
    p2: Point3<f64>,
    p3: Point3<f64>,
    p4: Point3<f64>
}
impl Quad3d
{
    fn new(p1: Point3<f64>, p2: Point3<f64>, p3: Point3<f64>, p4: Point3<f64>) -> Quad3d
    {
        Quad3d
            {
                p1: p1,
                p2: p2,
                p3: p3,
                p4: p4
            }
    }
}

struct Quad2d
{
    p1: Point2<f64>,
    p2: Point2<f64>,
    p3: Point2<f64>,
    p4: Point2<f64>
}
impl Quad2d
{
    fn new(p1: Point2<f64>, p2: Point2<f64>, p3: Point2<f64>, p4: Point2<f64>) -> Quad2d
    {
        Quad2d
            {
                p1: p1,
                p2: p2,
                p3: p3,
                p4: p4
            }
    }

    /// p1-----( )  =>  p1-----p2
    ///  |      |   =>  |       |
    ///  |      |   =>  |       |
    /// ( )-----p2  =>  p4-----p3
    fn new_rect(p1: Point2<f64>, p2: Point2<f64>) -> Quad2d
    {
        Quad2d
            {
                p1: p1,
                p2: Point2::<f64>::new(p1.x, p2.y),
                p3: p2,
                p4: Point2::<f64>::new(p2.x, p1.y)
            }
    }
}