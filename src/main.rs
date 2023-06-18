use rustracer::math::Vector3;
use rustracer::math::Point3;
use rustracer::math::geometry::ParametricLine;
use rustracer::math::geometry::Plane3;

fn main() {
    let origin = Point3::new( 0.0, 0.0, 0.0 );
    let direction = Vector3::new( 0.0, 0.0, -1.0 ); 

    let _ray = ParametricLine::new(origin, direction);

    let anchor = Point3::new( 0.0, -1.0, 0.0 );
    let normal = Vector3::new( 0.0, 1.0, 0.0 );

    let _plane = Plane3::new(anchor, normal);

    let _x = direction * direction;


    println!("Hello, world!");
}
