


pub struct RenderableGeometry<G, M, T> {
    pub geometry: G,
    pub material: M,
    pub transform: T,
}

impl<G, M, T> RenderableGeometry<G, M, T> {
    pub fn new(geometry: G, material: M, transform: T) -> RenderableGeometry<G, M, T> {
        RenderableGeometry {
            geometry,
            material,
            transform,
        }
    }
}

/*
pub struct Node<T: Length, C: Color, E> {
    pub transform: Transform3<<T as Div>::Output>,
    pub elements: Vec<
        Box<
            dyn E
/*            dyn Renderable<
                T,
                ScalarType = <T as Div>::Output,
                LengthType = T,
                PointType = Point3<T>,
                VectorType = Vector3<T>,
                NormalType = <Vector3<T> as NormalizableVector>::NormalType,
                ColorType = C,
            >,*/
        >,
    >,
}

impl<T: Length, C: Color, E> Node<T, C, E> {
    pub fn new(
        transform: Transform3<<T as Div>::Output>,
        elements: Vec<
            Box<
                dyn E
/*                dyn Renderable<
                    T,
                    ScalarType = <T as Div>::Output,
                    LengthType = T,
                    PointType = Point3<T>,
                    VectorType = Vector3<T>,
                    NormalType = <Vector3<T> as NormalizableVector>::NormalType,
                    ColorType = C,
                >,*/
            >,
        >,
    ) -> Node<T, C, E> {
        Node {
            transform,
            elements,
        }
    }
}*/
