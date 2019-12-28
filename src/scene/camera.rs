use crate::geometry::vec3::{Vec3};

pub struct Camera {
    pub position: Vec3,
    pub rotation_matrix: [[f32; 3]; 3],
    pub rotation: Vec3,
}

impl Camera {
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            rotation_matrix: [
                [0.7071, 0., -0.7071],
                [0., 1., 0.],
                [0.7071, 0., 0.7071],
            ],
            rotation: Vec3::new(0., 0., 0.),
        }
    }

    pub fn move_by(&mut self, distance: &Vec3) {
        self.position = self.position.plus(distance);
    }

    pub fn rotate_by(&mut self, rotation: &Vec3) {
        self.rotation = self.rotation.plus(rotation);

        let alpha = self.rotation.x * 3.14 / 180.;
        let beta = self.rotation.y * 3.14 / 180.;
        let gamma = self.rotation.z * 3.14 / 180.;

        let x_matrix = [
            [1., 0., 0.,],
            [0., gamma.cos(), -gamma.sin()],
            [0., gamma.sin(), gamma.cos()]
        ];
        let y_matrix = [
            [ beta.cos(), 0., beta.sin(),],
            [0., 1., 0.],
            [0., beta.sin(), beta.cos()]
        ];
        let z_matrix = [
            [alpha.cos(), -alpha.sin(), 0.,],
            [alpha.sin(), alpha.cos(), 0.],
            [0., 0., 1.]
        ];

        let result = multiply_matrixes(z_matrix, multiply_matrixes(y_matrix, x_matrix));

        self.rotation_matrix = result;
    }
}

fn multiply_matrixes(mat1: [[f32; 3]; 3], mat2: [[f32; 3]; 3]) -> [[f32; 3]; 3] {
    let mut result: [[f32; 3]; 3] = [[0., 0., 0.], [0., 0., 0.], [0., 0., 0.]];

    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                result[i][j] += mat1[i][k] * mat2[k][j];
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::scene::camera::Camera;
    use crate::geometry::vec3::Vec3;

    #[test]
    fn move_by() {
        let mut camera = Camera::new(Vec3::new(0., 0., 0.));

        camera.move_by(&Vec3::new(1., 2., 3.));
        camera.move_by(&Vec3::new(3., 2., 2.));

        assert_eq!(camera.position.x, 4.);
        assert_eq!(camera.position.y, 4.);
        assert_eq!(camera.position.z, 5.);
    }

    #[test]
    fn multiply_matrixes() {
        let mat1 = [
            [1., 2., 3.],
            [4., 5., 6.],
            [3., 2., 2.],
        ];

        let mat2 = [
            [2., 3., 3.],
            [1., 3., 2.],
            [1., 1., 3.],
        ];

        let result = crate::scene::camera::multiply_matrixes(mat1, mat2);
        let actual_result = [
            [7., 12., 16.],
            [19., 33., 40.],
            [10., 17., 19.],
        ];

        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(result[i][j], actual_result[i][j]);
            }
        }
    }
}