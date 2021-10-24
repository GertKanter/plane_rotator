# plane_rotator

This program calculates the rotation matrix to rotate the normal of a plane to point in the z-axis direction `(x=0, y=0, z=1)`.

## Installation

* Install Rust
* Build the package using `cargo build && cargo install --path .`.

## Usage

```
Usage:
  plane_rotator [OPTIONS]

This program calculates the rotation matrix to rotate a plane's normal to the
desired vector (default is {0, 0, 1}).

Optional arguments:
  -h,--help             Show this help message and exit
  -e,--equation EQUATION
                        Plane equation coefficients (a, b, c, d) in the formula
                        ax + by + cz + d = 0
  -p,--points POINTS    Plane points "(x1, y1, z1), (x2, y2, z2), (x3, y3, z3)"
                        For example: plane_rotator -p (0, 0, 0), (1, 1, 0),
                        (-1, 1, 0)
```

## Example

For the plane equation `1x + 0y + 0z + 0 = 0`:

```
# plane_rotator -e 1,0,0,0
rotation_matrix = Matrix { data: [[0.0, 0.0, 1.0], [0.0, 1.0, 0.0], [-1.0, 0.0, 0.0]] }
```

For the plane equation `0.05x + 0.96y + 0.28z + -2.24 = 0`:

```
# plane_rotator -e 0.05,0.96,0.28,-2.24
rotation_matrix = Matrix { data: [[0.9980512, -0.0374167, 0.04993762], [-0.0374167, 0.28159946, 0.9588022], [-0.04993762, -0.9588022, 0.2796507]] }
```
