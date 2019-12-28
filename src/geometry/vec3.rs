pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn dot_product(&self, vec: &Vec3) -> f32 {
        self.x * vec.x + self.y * vec.y  + self.z * vec.z
    }

    pub fn scale(&self, factor: f32) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        }
    }

    pub fn divide(&self, factor: f32) -> Self {
        Self {
            x: self.x / factor,
            y: self.y / factor,
            z: self.z / factor,
        }
    }

    pub fn minus(&self, vec: &Vec3) -> Self {
        Self {
            x: self.x - vec.x,
            y: self.y - vec.y,
            z: self.z - vec.z,
        }
    }

    pub fn plus(&self, vec: &Vec3) -> Self {
        Self {
            x: self.x + vec.x,
            y: self.y + vec.y,
            z: self.z + vec.z,
        }
    }

    pub fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        self.divide(self.length())
    }

    pub fn scale_by_matrix(&self, matrix: [[f32; 3]; 3]) -> Self {
        let mut result = [0., 0., 0.];
        let current = [self.x, self.y, self.z];

        for i in 0..3 {
            for j in 0..3 {
                result[i] += current[j] * matrix[i][j];
            }
        }

        Self { x: result[0], y: result[1], z: result[2] }
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::vec3::{Vec3};

    #[test]
    fn length() {
        let vec = Vec3::new(0., 3., 4.);
        let length = vec.length();

        assert_eq!(length, 5.);
    }

    #[test]
    fn normalize() {
        let vec = Vec3::new(0., 3., 4.);
        let normal = vec.normalize();

        let expected_x = 0.;
        let expected_y = 3. / 5.;
        let expected_z = 4. / 5.;

        assert_eq!(normal.x, expected_x);
        assert_eq!(normal.y, expected_y);
        assert_eq!(normal.z, expected_z);
    }

    #[test]
    fn plus() {
        let vec1 = Vec3::new(1., 1., 1.);
        let vec2 = Vec3::new(1., 2., 3.);
        let result = vec1.plus(&vec2);
        
        assert_eq!(result.x, 2.0);
        assert_eq!(result.y, 3.0);
        assert_eq!(result.z, 4.0);
    }

    #[test]
    fn minus() {
        let vec1 = Vec3::new(1., 1., 1.);
        let vec2 = Vec3::new(1., 2., 3.);
        let result = vec1.minus(&vec2);
        
        assert_eq!(result.x, 0.0);
        assert_eq!(result.y, -1.0);
        assert_eq!(result.z, -2.0);
    }

    #[test]
    fn dot_product() {
        let vec1 = Vec3::new(1., 2., 3.);
        let vec2 = Vec3::new(2., 3., 4.);
        let result = vec1.dot_product(&vec2);

        assert_eq!(result, 20.0);
    }

    #[test]
    fn scale() {
        let vec = Vec3::new(3., 2., 4.);
        let new_vec = vec.scale(2.);

        assert_eq!(new_vec.x, 6.);
        assert_eq!(new_vec.y, 4.);
        assert_eq!(new_vec.z, 8.);
    }

     #[test]
    fn divide() {
        let vec = Vec3::new(3., 2., 4.);
        let new_vec = vec.divide(2.);

        assert_eq!(new_vec.x, 1.5);
        assert_eq!(new_vec.y, 1.);
        assert_eq!(new_vec.z, 2.);
    }

    #[test]
    fn scale_by_matrix() {
        let vec = Vec3::new(2., 3., 4.);
        let matrix = [
            [1., 2., 3.],
            [3., 2., 1.],
            [4., 1., 3.],
        ];
        let result = vec.scale_by_matrix(matrix);

        assert_eq!(result.x, 20.);
        assert_eq!(result.y, 16.);
        assert_eq!(result.z, 23.);
    }
}