use crate::Vertex;

pub fn make_cube(lenth: f32) -> ([Vertex; 8], [u32; 36]) {
    let vertecies = [
        Vertex::new(
            [-lenth / 2.0, -lenth / 2.0, -lenth / 2.0], // = = 0
            [1.0, 0.0, 0.0],                              //
            [0.0, 0.0],                                       // # =
        ),
        Vertex::new(
            [-lenth / 2.0, -lenth / 2.0, lenth / 2.0],  // = = 1
            [0.0, 1.0, 0.0],                              //
            [0.0, 0.0],                                       // - =
        ), 
        Vertex::new(
            [lenth / 2.0, -lenth / 2.0, -lenth / 2.0],  // = = 2
            [0.0, 0.0, 1.0],                              //
            [1.0, 0.0],                                       // = #
        ),
        Vertex::new(
            [lenth / 2.0, -lenth / 2.0, lenth / 2.0],   // = = 3
            [1.0, 0.0, 0.0],                              //
            [1.0, 0.0],                                       // = -
        ), 
        Vertex::new( 
            [-lenth / 2.0, lenth / 2.0, -lenth / 2.0],  // # = 4 0 1
            [0.0, 0.0, 1.0],                              //
            [0.0, 1.0],                                       // = =
        ),
        Vertex::new(
            [-lenth / 2.0, lenth / 2.0, lenth / 2.0],   // - = 5
            [1.0, 0.0, 0.0],                              //
            [0.0, 1.0],                                       // = = 0 1
        ),  
        Vertex::new(  
            [lenth / 2.0, lenth / 2.0, -lenth / 2.0],   // = # 6
            [0.0, 0.0, 1.0],                              //
            [1.0, 1.0],                                       // = =
        ),
        Vertex::new(
            [lenth / 2.0, lenth / 2.0, lenth / 2.0],    // = - 7
            [1.0, 0.0, 0.0],                              //
            [1.0, 1.0],                                       // = =
        ),
    ];

    let indecies = [
        0, 1, 2, /**/ 2, 1, 3, /**/ 4, 5, 6, /**/ 6, 5, 7, /**/ 0, 1, 5,
        /**/ 0, 5, 4, /**/ 2, 3, 6, /**/ 6, 3, 7, /**/ 0, 2, 4, /**/ 4, 6,
        2, /**/
        1, 3, 5, /**/ 3, 5, 7,
    ];
    return (vertecies, indecies);
}
