macro_rules! rotation_from_equation {
    ($a:expr) => {
        rotation_from_equation!($a, nalgebra::Vector3::new(0.0, 0.0, 1.0));
    };
    ($a:expr, $b:expr) => {
        rotation_from_equation($a, $b);
    }
}

fn rotation_from_equation(normal: nalgebra::Vector4<f32>, goal: nalgebra::Vector3<f32>) -> nalgebra::Matrix3<f32> {
    let normalized = normal.normalize();
    println!("Parsed normal vector as: {:?}", normalized);
    let normal = nalgebra::Vector3::new(normalized[0], normalized[1], normalized[2]);
    let identity: nalgebra::Matrix3<f32> =  nalgebra::Matrix3::identity();
    let rotation_matrix;
    if normal == goal {
        rotation_matrix = identity;
    }
    else {
        let v = normal.cross(&goal);
        let s = v.norm();
        let c = goal.dot(&normal);
        let skew = nalgebra::Matrix3::new(0.0, -v[2], v[1],
                                            v[2], 0.0, -v[0],
                                            -v[1], v[0], 0.0);
        rotation_matrix = identity + skew + ((skew * skew) * ((1.0 - c) / (s*s)));
    }
    println!("rotation_matrix = {:?}", rotation_matrix);
    
    let rotated = rotate_vector(normal, rotation_matrix);
    println!("rotated = {:?}", rotated);
    rotation_matrix
}

fn is_rotation_matrix(matrix: nalgebra::Matrix3<f32>) -> bool {
    let transposed = matrix.transpose();
    let identity: nalgebra::Matrix3<f32> =  nalgebra::Matrix3::identity();
    let should_be_identity = transposed * matrix;
    should_be_identity.metric_distance(&identity) < f32::powf(0.1, 10.0)
}

fn rotate_vector(vector: nalgebra::Vector3<f32>, rotation: nalgebra::Matrix3<f32>) -> nalgebra::Vector3<f32> {
    rotation * vector
}

fn tokenize(input: String) -> Vec<f32> {
    let trimmed = input.replace(" ", "");
    let tokens: Vec<&str> = trimmed.split(",").collect();
    let mut values: Vec<f32> = Vec::new();
    for token in tokens {
        if token.parse::<f32>().is_ok() {
            values.push(token.parse().unwrap());
        }
    }
    values
}

fn main() {
    let mut equation_string = String::new();
    let mut points_string = String::new();
    {
        let mut ap = argparse::ArgumentParser::new();
        ap.set_description("This program calculates the rotation matrix to rotate a plane's normal to the desired vector (default is {0, 0, 1}).");
        ap.refer(&mut equation_string)
          .add_option(&["-e", "--equation"], argparse::Store,
                      "Plane equation coefficients (a, b, c, d) in the formula ax + by + cz + d = 0");
        ap.refer(&mut points_string)
          .add_option(&["-p", "--points"], argparse::Store,
                      "Plane points \"(x1, y1, z1), (x2, y2, z2), (x3, y3, z3)\"\n
                       For example: plane_rotator -p (0, 0, 0), (1, 1, 0), (-1, 1, 0)");
        ap.parse_args_or_exit();
    }
    if equation_string.len() > 0 {
        let coefficients = tokenize(equation_string);
        if coefficients.len() < 4 {
            println!("Equation has to be in the form \"a,b,c,d\" in the formula ax + by + cz + d = 0");
            println!("Example: plane_rotator -e \"1, 0, 0, 0\"");
        }
        else {
            // Find the normal
            // Normal is just the coefficients
            let equation = nalgebra::Vector4::new(coefficients[0], coefficients[1], coefficients[2], 0.0);
            is_rotation_matrix(rotation_from_equation!(equation));
        }
    }
    if points_string.len() > 0 {
        let points = tokenize(points_string.replace("(", "").replace(")", ""));
        if points.len() == 9 {
            // p1 = (points[0], points[1], points[2])
            // p2 = (points[3], points[4], points[5])
            // p3 = (points[6], points[7], points[8])
            let v1 = nalgebra::Vector3::new(points[3] - points[0], points[4] - points[1], points[5] - points[2]); // v1 = p2 - p1
            let v2 = nalgebra::Vector3::new(points[6] - points[0], points[7] - points[1], points[8] - points[2]); // v2 = p3 - p1
            let normal = v1.cross(&v2);
            is_rotation_matrix(rotation_from_equation!(nalgebra::Vector4::new(normal[0], normal[1], normal[2], 0.0)));
            
        }
        else {
            println!("Points have to be in the form \"(x1, y1, z1), (x2, y2, z2), (x3, y3, z3)\"");
            println!("For example: plane_rotator -p (0, 0, 0), (1, 1, 0), (-1, 1, 0)");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_vector() {
        assert_eq!(rotate_vector(nalgebra::Vector3::new(1.0, 0.0, 0.0), nalgebra::Matrix3::new(1.0, 0.0, 0.0,
                                                                                               0.0, 1.0, 0.0,
                                                                                               0.0, 0.0, 1.0)),
                   nalgebra::Vector3::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_normalization() {
        assert_eq!(rotation_from_equation!(nalgebra::Vector4::new(0.0, 1.0, 0.0, 0.0)),
                   rotation_from_equation!(nalgebra::Vector4::new(0.0, 2.0, 0.0, 0.0)));
    }

    #[test]
    fn test_rotation_from_equation() {
        assert_eq!(rotation_from_equation!(nalgebra::Vector4::new(0.0, 1.0, 0.0, 0.0)),
                   nalgebra::Matrix3::new(1.0, 0.0, 0.0,
                                          0.0, 0.0, -1.0,
                                          0.0, 1.0, 0.0));
    }

    #[test]
    fn test_rotation_matrix() {
        assert_eq!(is_rotation_matrix(rotation_from_equation!(nalgebra::Vector4::new(0.0, 1.0, 0.0, 0.0))), true);
    }

}

