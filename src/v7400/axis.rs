//! Axis and direction.

use std::fmt;

/// Direction (left, right, up, down, front, and back).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    /// Left.
    Left,
    /// Right.
    Right,
    /// Up.
    Up,
    /// Down.
    Down,
    /// Front; the direction from the screen toward the camera.
    ///
    /// > Vector with origin at the screen pointing toward the camera.
    /// >
    /// > --- [documentation for `EFrontVector`, FBX SDK 2017][fbxsdk-2017-efrontvector].
    ///
    /// [fbxsdk-2017-efrontvector]: https://help.autodesk.com/cloudhelp/2017/ENU/FBX-Developer-Help/cpp_ref/class_fbx_axis_system.html#a9b4b5a1c8cbc614be46eed206084f551
    Front,
    /// Back; the direction from the camera toward the screen.
    Back,
}

impl Direction {
    /// Returns the opposite direction.
    ///
    /// # Examples
    ///
    /// ```
    /// # use fbxcel_dom::v7400::Direction;
    /// assert_eq!(Direction::Left.opposite(), Direction::Right);
    /// assert_eq!(Direction::Right.opposite(), Direction::Left);
    ///
    /// assert_eq!(Direction::Up.opposite(), Direction::Down);
    /// assert_eq!(Direction::Down.opposite(), Direction::Up);
    ///
    /// assert_eq!(Direction::Front.opposite(), Direction::Back);
    /// assert_eq!(Direction::Back.opposite(), Direction::Front);
    /// ```
    #[inline]
    #[must_use]
    pub fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Front => Self::Back,
            Self::Back => Self::Front,
        }
    }

    /// Returns the third basis for the given two bases in a right-handed coordinate system.
    ///
    /// # Failures
    ///
    /// Returns `None` if the given two bases are same or opposite.
    ///
    /// # Examples
    ///
    /// ```
    /// # use fbxcel_dom::v7400::Direction;
    /// // Note that "front" is a direction from the screen toward the camera,
    /// // i.e. far to near.
    /// assert_eq!(
    ///     Direction::right_handed_third_axis(Direction::Right, Direction::Up),
    ///     Some(Direction::Front)
    /// );
    /// assert_eq!(
    ///     Direction::right_handed_third_axis(Direction::Right, Direction::Back),
    ///     Some(Direction::Up)
    /// );
    ///
    /// // Returns `None` for bases which are the same or the opposite.
    /// assert_eq!(
    ///     Direction::right_handed_third_axis(Direction::Right, Direction::Right),
    ///     None
    /// );
    /// assert_eq!(
    ///     Direction::right_handed_third_axis(Direction::Right, Direction::Left),
    ///     None
    /// );
    /// ```
    #[must_use]
    pub fn right_handed_third_axis(v1: Self, v2: Self) -> Option<Self> {
        use Direction::*;

        Some(match (v1, v2) {
            (Left, Up) => Back,
            (Left, Down) => Front,
            (Left, Front) => Up,
            (Left, Back) => Down,
            (Right, Up) => Front,
            (Right, Down) => Back,
            (Right, Front) => Down,
            (Right, Back) => Up,
            (Up, Left) => Front,
            (Up, Right) => Back,
            (Up, Front) => Right,
            (Up, Back) => Left,
            (Down, Left) => Back,
            (Down, Right) => Front,
            (Down, Front) => Left,
            (Down, Back) => Right,
            (Front, Left) => Down,
            (Front, Right) => Up,
            (Front, Up) => Left,
            (Front, Down) => Right,
            (Back, Left) => Up,
            (Back, Right) => Down,
            (Back, Up) => Right,
            (Back, Down) => Left,
            _ => {
                assert!((v1 == v2) || (v1 == v2.opposite()));
                return None;
            }
        })
    }
}

/// Signed axis (+X, -X, +Y, -Y, +Z, and -Z).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SignedAxis {
    /// +X.
    PosX,
    /// -X.
    NegX,
    /// +Y.
    PosY,
    /// -Y.
    NegY,
    /// +Z.
    PosZ,
    /// -Z.
    NegZ,
}

impl SignedAxis {
    /// Returns the signed axis as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// # use fbxcel_dom::v7400::SignedAxis;
    /// assert_eq!(SignedAxis::PosX.to_str(), "+X");
    /// assert_eq!(SignedAxis::NegX.to_str(), "-X");
    /// assert_eq!(SignedAxis::PosY.to_str(), "+Y");
    /// assert_eq!(SignedAxis::NegY.to_str(), "-Y");
    /// assert_eq!(SignedAxis::PosZ.to_str(), "+Z");
    /// assert_eq!(SignedAxis::NegZ.to_str(), "-Z");
    /// ```
    #[inline]
    #[must_use]
    pub fn to_str(self) -> &'static str {
        match self {
            Self::PosX => "+X",
            Self::NegX => "-X",
            Self::PosY => "+Y",
            Self::NegY => "-Y",
            Self::PosZ => "+Z",
            Self::NegZ => "-Z",
        }
    }

    /// Returns the opposite direction.
    ///
    /// # Examples
    ///
    /// ```
    /// # use fbxcel_dom::v7400::SignedAxis;
    /// assert_eq!(SignedAxis::PosX.opposite(), SignedAxis::NegX);
    /// assert_eq!(SignedAxis::NegX.opposite(), SignedAxis::PosX);
    ///
    /// assert_eq!(SignedAxis::PosY.opposite(), SignedAxis::NegY);
    /// assert_eq!(SignedAxis::NegY.opposite(), SignedAxis::PosY);
    ///
    /// assert_eq!(SignedAxis::PosZ.opposite(), SignedAxis::NegZ);
    /// assert_eq!(SignedAxis::NegZ.opposite(), SignedAxis::PosZ);
    /// ```
    #[inline]
    #[must_use]
    pub fn opposite(self) -> Self {
        match self {
            Self::PosX => Self::NegX,
            Self::NegX => Self::PosX,
            Self::PosY => Self::NegY,
            Self::NegY => Self::PosY,
            Self::PosZ => Self::NegZ,
            Self::NegZ => Self::PosZ,
        }
    }

    /// Returns whether the direction is positive.
    ///
    /// To know whether the direction is negative, you can also use
    /// [`is_negative`][`Self::is_negative`] method.
    ///
    /// # Examples
    ///
    /// ```
    /// # use fbxcel_dom::v7400::SignedAxis;
    /// assert!(SignedAxis::PosX.is_positive());
    /// assert!(SignedAxis::PosY.is_positive());
    /// assert!(SignedAxis::PosZ.is_positive());
    ///
    /// assert!(!SignedAxis::NegX.is_positive());
    /// assert!(!SignedAxis::NegY.is_positive());
    /// assert!(!SignedAxis::NegZ.is_positive());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_positive(self) -> bool {
        matches!(self, Self::PosX | Self::PosY | Self::PosZ)
    }

    /// Returns whether the direction is negative.
    ///
    /// To know whether the direction is positive, you can also use
    /// [`is_positive`][`Self::is_positive`] method.
    ///
    /// # Examples
    ///
    /// ```
    /// # use fbxcel_dom::v7400::SignedAxis;
    /// assert!(!SignedAxis::PosX.is_negative());
    /// assert!(!SignedAxis::PosY.is_negative());
    /// assert!(!SignedAxis::PosZ.is_negative());
    ///
    /// assert!(SignedAxis::NegX.is_negative());
    /// assert!(SignedAxis::NegY.is_negative());
    /// assert!(SignedAxis::NegZ.is_negative());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_negative(self) -> bool {
        !self.is_positive()
    }
}

impl fmt::Display for SignedAxis {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_str())
    }
}

/// Axes for Up, Front, Right.
// The number of variants is 48 (= 6*4*2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum AxisSystemRepr {
    /// Up: +X, Front: +Y, Right: +Z.
    PxPyPz = 0,
    /// Up: +X, Front: +Y, Right: -Z.
    PxPyNz,
    /// Up: +X, Front: -Y, Right: +Z.
    PxNyPz,
    /// Up: +X, Front: -Y, Right: -Z.
    PxNyNz,
    /// Up: -X, Front: +Y, Right: +Z.
    NxPyPz,
    /// Up: -X, Front: +Y, Right: -Z.
    NxPyNz,
    /// Up: -X, Front: -Y, Right: +Z.
    NxNyPz,
    /// Up: -X, Front: -Y, Right: -Z.
    NxNyNz,
    //
    /// Up: +X, Front: +Z, Right: +Y.
    PxPzPy,
    /// Up: +X, Front: +Z, Right: -Y.
    PxPzNy,
    /// Up: +X, Front: -Z, Right: +Y.
    PxNzPy,
    /// Up: +X, Front: -Z, Right: -Y.
    PxNzNy,
    /// Up: -X, Front: +Z, Right: +Y.
    NxPzPy,
    /// Up: -X, Front: +Z, Right: -Y.
    NxPzNy,
    /// Up: -X, Front: -Z, Right: +Y.
    NxNzPy,
    /// Up: -X, Front: -Z, Right: -Y.
    NxNzNy,
    //
    /// Up: +Y, Front: +X, Right: +Z.
    PyPxPz,
    /// Up: +Y, Front: +X, Right: -Z.
    PyPxNz,
    /// Up: +Y, Front: -X, Right: +Z.
    PyNxPz,
    /// Up: +Y, Front: -X, Right: -Z.
    PyNxNz,
    /// Up: -Y, Front: +X, Right: +Z.
    NyPxPz,
    /// Up: -Y, Front: +X, Right: -Z.
    NyPxNz,
    /// Up: -Y, Front: -X, Right: +Z.
    NyNxPz,
    /// Up: -Y, Front: -X, Right: -Z.
    NyNxNz,
    //
    /// Up: +Y, Front: +Z, Right: +X.
    PyPzPx,
    /// Up: +Y, Front: +Z, Right: -X.
    PyPzNx,
    /// Up: +Y, Front: -Z, Right: +X.
    PyNzPx,
    /// Up: +Y, Front: -Z, Right: -X.
    PyNzNx,
    /// Up: -Y, Front: +Z, Right: +X.
    NyPzPx,
    /// Up: -Y, Front: +Z, Right: -X.
    NyPzNx,
    /// Up: -Y, Front: -Z, Right: +X.
    NyNzPx,
    /// Up: -Y, Front: -Z, Right: -X.
    NyNzNx,
    //
    /// Up: +Z, Front: +X, Right: +Y.
    PzPxPy,
    /// Up: +Z, Front: +X, Right: -Y.
    PzPxNy,
    /// Up: +Z, Front: -X, Right: +Y.
    PzNxPy,
    /// Up: +Z, Front: -X, Right: -Y.
    PzNxNy,
    /// Up: -Z, Front: +X, Right: +Y.
    NzPxPy,
    /// Up: -Z, Front: +X, Right: -Y.
    NzPxNy,
    /// Up: -Z, Front: -X, Right: +Y.
    NzNxPy,
    /// Up: -Z, Front: -X, Right: -Y.
    NzNxNy,
    //
    /// Up: +Z, Front: +Y, Right: +X.
    PzPyPx,
    /// Up: +Z, Front: +Y, Right: -X.
    PzPyNx,
    /// Up: +Z, Front: -Y, Right: +X.
    PzNyPx,
    /// Up: +Z, Front: -Y, Right: -X.
    PzNyNx,
    /// Up: -Z, Front: +Y, Right: +X.
    NzPyPx,
    /// Up: -Z, Front: +Y, Right: -X.
    NzPyNx,
    /// Up: -Z, Front: -Y, Right: +X.
    NzNyPx,
    /// Up: -Z, Front: -Y, Right: -X.
    NzNyNx,
}

/// Coordinate system, which contains directions corresponding to each axis.
///
/// It is guaranteed that the size of `Option<AxisSystem>` is same as `AxisSystem` itself.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct AxisSystem {
    /// Internal representation.
    repr: AxisSystemRepr,
}

impl fmt::Debug for AxisSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AxisSystem")
            .field("x", &self.x_direction())
            .field("y", &self.y_direction())
            .field("z", &self.z_direction())
            .finish()
    }
}

impl AxisSystem {
    /// Returns directions of (X, Y, Z) axes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use fbxcel_dom::v7400::AxisSystem;
    /// use fbxcel_dom::v7400::Direction;
    ///
    /// // Note that "front" is a direction from the screen toward the camera.
    /// let y_up = AxisSystem::from_xyz(Direction::Right, Direction::Up, Direction::Front)
    ///     .expect("should never fail: valid axis system");
    /// assert_eq!(y_up.directions(), [Direction::Right, Direction::Up, Direction::Front]);
    ///
    /// let z_up = AxisSystem::from_xyz(Direction::Right, Direction::Back, Direction::Up)
    ///     .expect("should never fail: valid axis system");
    /// assert_eq!(z_up.directions(), [Direction::Right, Direction::Back, Direction::Up]);
    /// ```
    #[must_use]
    pub fn directions(self) -> [Direction; 3] {
        use Direction::*;

        /// Axes directions for each axis system.
        const DIRECTIONS: [[Direction; 3]; 48] = [
            [Up, Front, Right],
            [Up, Front, Left],
            [Up, Back, Right],
            [Up, Back, Left],
            [Down, Front, Right],
            [Down, Front, Left],
            [Down, Back, Right],
            [Down, Back, Left],
            //
            [Up, Right, Front],
            [Up, Left, Front],
            [Up, Right, Back],
            [Up, Left, Back],
            [Down, Right, Front],
            [Down, Left, Front],
            [Down, Right, Back],
            [Down, Left, Back],
            //
            [Front, Up, Right],
            [Front, Up, Left],
            [Back, Up, Right],
            [Back, Up, Left],
            [Front, Down, Right],
            [Front, Down, Left],
            [Back, Down, Right],
            [Back, Down, Left],
            //
            [Right, Up, Front],
            [Left, Up, Front],
            [Right, Up, Back],
            [Left, Up, Back],
            [Right, Down, Front],
            [Left, Down, Front],
            [Right, Down, Back],
            [Left, Down, Back],
            //
            [Front, Right, Up],
            [Front, Left, Up],
            [Back, Right, Up],
            [Back, Left, Up],
            [Front, Right, Down],
            [Front, Left, Down],
            [Back, Right, Down],
            [Back, Left, Down],
            //
            [Right, Front, Up],
            [Left, Front, Up],
            [Right, Back, Up],
            [Left, Back, Up],
            [Right, Front, Down],
            [Left, Front, Down],
            [Right, Back, Down],
            [Left, Back, Down],
        ];

        DIRECTIONS[self.repr as usize]
    }

    /// Returns the direction of positive X axis.
    ///
    /// # Examples
    ///
    /// ```
    /// # use fbxcel_dom::v7400::AxisSystem;
    /// use fbxcel_dom::v7400::Direction;
    ///
    /// // Note that "front" is a direction from the screen toward the camera.
    /// let y_up = AxisSystem::from_xyz(Direction::Right, Direction::Up, Direction::Front)
    ///     .expect("should never fail: valid axis system");
    /// assert_eq!(y_up.x_direction(), Direction::Right);
    ///
    /// let z_up = AxisSystem::from_xyz(Direction::Right, Direction::Back, Direction::Up)
    ///     .expect("should never fail: valid axis system");
    /// assert_eq!(z_up.x_direction(), Direction::Right);
    /// ```
    #[inline]
    #[must_use]
    pub fn x_direction(self) -> Direction {
        self.directions()[0]
    }

    /// Returns the direction of positive Y axis.
    ///
    /// # Examples
    ///
    /// ```
    /// # use fbxcel_dom::v7400::AxisSystem;
    /// use fbxcel_dom::v7400::Direction;
    ///
    /// // Note that "front" is a direction from the screen toward the camera.
    /// let y_up = AxisSystem::from_xyz(Direction::Right, Direction::Up, Direction::Front)
    ///     .expect("should never fail: valid axis system");
    /// assert_eq!(y_up.y_direction(), Direction::Up);
    ///
    /// let z_up = AxisSystem::from_xyz(Direction::Right, Direction::Back, Direction::Up)
    ///     .expect("should never fail: valid axis system");
    /// assert_eq!(z_up.y_direction(), Direction::Back);
    /// ```
    #[inline]
    #[must_use]
    pub fn y_direction(self) -> Direction {
        self.directions()[1]
    }

    /// Returns the direction of positive Z axis.
    ///
    /// # Examples
    ///
    /// ```
    /// # use fbxcel_dom::v7400::AxisSystem;
    /// use fbxcel_dom::v7400::Direction;
    ///
    /// // Note that "front" is a direction from the screen toward the camera.
    /// let y_up = AxisSystem::from_xyz(Direction::Right, Direction::Up, Direction::Front)
    ///     .expect("should never fail: valid axis system");
    /// assert_eq!(y_up.z_direction(), Direction::Front);
    ///
    /// let z_up = AxisSystem::from_xyz(Direction::Right, Direction::Back, Direction::Up)
    ///     .expect("should never fail: valid axis system");
    /// assert_eq!(z_up.z_direction(), Direction::Up);
    /// ```
    #[inline]
    #[must_use]
    pub fn z_direction(self) -> Direction {
        self.directions()[2]
    }

    /// Creates the axis system with the given (X, Y, Z) axes directions.
    ///
    /// # Failures
    ///
    /// Returns `None` if two or more of the given directions are same or opposite.
    ///
    /// # Examples
    ///
    /// ```
    /// # use fbxcel_dom::v7400::AxisSystem;
    /// use fbxcel_dom::v7400::Direction;
    ///
    /// // Note that "front" is a direction from the screen toward the camera.
    /// let y_up = AxisSystem::from_xyz(Direction::Right, Direction::Up, Direction::Front)
    ///     .expect("should never fail: valid axis system");
    /// assert_eq!(y_up.x_direction(), Direction::Right);
    /// assert_eq!(y_up.y_direction(), Direction::Up);
    /// assert_eq!(y_up.z_direction(), Direction::Front);
    /// assert!(y_up.is_right_handed());
    ///
    /// let z_up = AxisSystem::from_xyz(Direction::Right, Direction::Back, Direction::Up)
    ///     .expect("should never fail: valid axis system");
    /// assert_eq!(z_up.x_direction(), Direction::Right);
    /// assert_eq!(z_up.y_direction(), Direction::Back);
    /// assert_eq!(z_up.z_direction(), Direction::Up);
    /// assert!(z_up.is_right_handed());
    ///
    /// // `None` is returned for invalid axis system.
    /// assert!(AxisSystem::from_xyz(Direction::Right, Direction::Left, Direction::Up).is_none());
    /// ```
    pub fn from_xyz(x: Direction, y: Direction, z: Direction) -> Option<Self> {
        use AxisSystemRepr::*;
        use Direction::*;

        let repr = match (x, y, z) {
            (Up, Front, Right) => PxPyPz,
            (Up, Front, Left) => PxPyNz,
            (Up, Back, Right) => PxNyPz,
            (Up, Back, Left) => PxNyNz,
            (Down, Front, Right) => NxPyPz,
            (Down, Front, Left) => NxPyNz,
            (Down, Back, Right) => NxNyPz,
            (Down, Back, Left) => NxNyNz,
            //
            (Up, Right, Front) => PxPzPy,
            (Up, Left, Front) => PxPzNy,
            (Up, Right, Back) => PxNzPy,
            (Up, Left, Back) => PxNzNy,
            (Down, Right, Front) => NxPzPy,
            (Down, Left, Front) => NxPzNy,
            (Down, Right, Back) => NxNzPy,
            (Down, Left, Back) => NxNzNy,
            //
            (Front, Up, Right) => PyPxPz,
            (Front, Up, Left) => PyPxNz,
            (Back, Up, Right) => PyNxPz,
            (Back, Up, Left) => PyNxNz,
            (Front, Down, Right) => NyPxPz,
            (Front, Down, Left) => NyPxNz,
            (Back, Down, Right) => NyNxPz,
            (Back, Down, Left) => NyNxNz,
            //
            (Right, Up, Front) => PyPzPx,
            (Left, Up, Front) => PyPzNx,
            (Right, Up, Back) => PyNzPx,
            (Left, Up, Back) => PyNzNx,
            (Right, Down, Front) => NyPzPx,
            (Left, Down, Front) => NyPzNx,
            (Right, Down, Back) => NyNzPx,
            (Left, Down, Back) => NyNzNx,
            //
            (Front, Right, Up) => PzPxPy,
            (Front, Left, Up) => PzPxNy,
            (Back, Right, Up) => PzNxPy,
            (Back, Left, Up) => PzNxNy,
            (Front, Right, Down) => NzPxPy,
            (Front, Left, Down) => NzPxNy,
            (Back, Right, Down) => NzNxPy,
            (Back, Left, Down) => NzNxNy,
            //
            (Right, Front, Up) => PzPyPx,
            (Left, Front, Up) => PzPyNx,
            (Right, Back, Up) => PzNyPx,
            (Left, Back, Up) => PzNyNx,
            (Right, Front, Down) => NzPyPx,
            (Left, Front, Down) => NzPyNx,
            (Right, Back, Down) => NzNyPx,
            (Left, Back, Down) => NzNyNx,
            //
            _ => return None,
        };

        Some(Self { repr })
    }

    /// Creates the axis system from up, front, right axes.
    ///
    /// Note that "front" is the direction from the screen toward the camera, i.e. far to near.
    ///
    /// # Failures
    ///
    /// Returns `None` if two or more of the given directions are same or opposite.
    ///
    /// # Examples
    ///
    /// ```
    /// # use fbxcel_dom::v7400::AxisSystem;
    /// use fbxcel_dom::v7400::{Direction, SignedAxis};
    ///
    /// // Note that "front" is a direction from the screen toward the camera.
    /// let y_up =
    ///     AxisSystem::from_up_front_right(SignedAxis::PosY, SignedAxis::PosZ, SignedAxis::PosX)
    ///         .expect("should never fail: valid axis system");
    /// assert_eq!(y_up.x_direction(), Direction::Right);
    /// assert_eq!(y_up.y_direction(), Direction::Up);
    /// assert_eq!(y_up.z_direction(), Direction::Front);
    /// assert!(y_up.is_right_handed());
    ///
    /// let z_up =
    ///     AxisSystem::from_up_front_right(SignedAxis::PosZ, SignedAxis::NegY, SignedAxis::PosX)
    ///         .expect("should never fail: valid axis system");
    /// assert_eq!(z_up.x_direction(), Direction::Right);
    /// assert_eq!(z_up.y_direction(), Direction::Back);
    /// assert_eq!(z_up.z_direction(), Direction::Up);
    /// assert!(z_up.is_right_handed());
    ///
    /// // `None` is returned for invalid axis system.
    /// assert!(
    ///     AxisSystem::from_up_front_right(SignedAxis::PosX, SignedAxis::NegX, SignedAxis::PosY)
    ///         .is_none()
    /// );
    /// ```
    pub fn from_up_front_right(
        up: SignedAxis,
        front: SignedAxis,
        right: SignedAxis,
    ) -> Option<Self> {
        let [x, y, z] = axes_to_directions(up, front, right)?;
        let asys = Self::from_xyz(x, y, z);
        assert!(
            asys.is_some(),
            "should be valid axis system since `axes_to_directions` already validated them"
        );
        asys
    }

    /// Returns whether the axis system is right-handed.
    ///
    /// To know whether the axis is **left**-handed, you can also use
    /// [`is_left_handed`][`Self::is_left_handed`] method.
    ///
    /// # Examples
    ///
    /// ```
    /// # use fbxcel_dom::v7400::AxisSystem;
    /// use fbxcel_dom::v7400::Direction;
    ///
    /// // Note that "front" is a direction from the screen toward the camera.
    /// let y_up = AxisSystem::from_xyz(Direction::Right, Direction::Up, Direction::Front)
    ///     .expect("should never fail: valid axis system");
    /// assert!(y_up.is_right_handed());
    ///
    /// let z_up = AxisSystem::from_xyz(Direction::Right, Direction::Back, Direction::Up)
    ///     .expect("should never fail: valid axis system");
    /// assert!(z_up.is_right_handed());
    ///
    /// let directx = AxisSystem::from_xyz(Direction::Right, Direction::Up, Direction::Back)
    ///     .expect("should never fail: valid axis system");
    /// assert!(!directx.is_right_handed());
    /// ```
    #[must_use]
    pub fn is_right_handed(self) -> bool {
        use AxisSystemRepr::*;

        // Coordinate system (Up, Front, Right) = (+X, +Y, +Z) is the right-handed system.
        // (Note that "front" is a direction from the screen toward the camera, i.e. far to near.)
        matches!(
            self.repr,
            PxPyPz
                | PxNyNz
                | NxPyNz
                | NxNyPz
                | PxPzNy
                | PxNzPy
                | NxPzPy
                | NxNzNy
                | PyPxNz
                | PyNxPz
                | NyPxPz
                | NyNxNz
                | PyPzPx
                | PyNzNx
                | NyPzNx
                | NyNzPx
                | PzPxPy
                | PzNxNy
                | NzPxNy
                | NzNxPy
                | PzPyNx
                | PzNyPx
                | NzPyPx
                | NzNyNx
        )
    }

    /// Returns whether the axis system is left-handed.
    ///
    /// To know whether the axis is **right**-handed, you can also use
    /// [`is_right_handed`][`Self::is_right_handed`] method.
    ///
    /// # Examples
    ///
    /// ```
    /// # use fbxcel_dom::v7400::AxisSystem;
    /// use fbxcel_dom::v7400::Direction;
    ///
    /// // Note that "front" is a direction from the screen toward the camera.
    /// let y_up = AxisSystem::from_xyz(Direction::Right, Direction::Up, Direction::Front)
    ///     .expect("should never fail: valid axis system");
    /// assert!(!y_up.is_left_handed());
    ///
    /// let z_up = AxisSystem::from_xyz(Direction::Right, Direction::Back, Direction::Up)
    ///     .expect("should never fail: valid axis system");
    /// assert!(!z_up.is_left_handed());
    ///
    /// let directx = AxisSystem::from_xyz(Direction::Right, Direction::Up, Direction::Back)
    ///     .expect("should never fail: valid axis system");
    /// assert!(directx.is_left_handed());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_left_handed(self) -> bool {
        !self.is_right_handed()
    }
}

/// Converts the axes for directions to directions for axes.
#[must_use]
fn axes_to_directions(
    up: SignedAxis,
    front: SignedAxis,
    right: SignedAxis,
) -> Option<[Direction; 3]> {
    use Direction::*;
    use SignedAxis::*;

    let mut axes = [(Up, up), (Front, front), (Right, right)];
    // Make axes positive.
    for (dir, axis) in &mut axes {
        if !axis.is_positive() {
            *dir = dir.opposite();
            *axis = axis.opposite();
        }
    }
    match axes {
        [(x, PosX), (y, PosY), (z, PosZ)]
        | [(x, PosX), (z, PosZ), (y, PosY)]
        | [(y, PosY), (x, PosX), (z, PosZ)]
        | [(y, PosY), (z, PosZ), (x, PosX)]
        | [(z, PosZ), (x, PosX), (y, PosY)]
        | [(z, PosZ), (y, PosY), (x, PosX)] => Some([x, y, z]),
        axes => {
            assert!(
                axes.iter().all(|(_dir, axis)| axis.is_positive()),
                "all axes should have been made positive"
            );
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashSet;
    use std::mem::size_of;

    #[test]
    fn axis_system_size() {
        assert_eq!(size_of::<AxisSystem>(), 1);
        assert_eq!(size_of::<AxisSystem>(), size_of::<Option<AxisSystem>>());
    }

    fn all_axis_systems() -> impl Iterator<Item = AxisSystem> {
        use Direction::*;

        const BASES: [(Direction, Direction, Direction); 6] = [
            (Left, Up, Front),
            (Left, Front, Up),
            (Up, Left, Front),
            (Up, Front, Left),
            (Front, Left, Up),
            (Front, Up, Left),
        ];

        (&BASES[..])
            .iter()
            .flat_map(|base: &(_, _, _)| {
                std::array::IntoIter::new([
                    *base,
                    (base.0, base.1, base.2.opposite()),
                    (base.0, base.1.opposite(), base.2),
                    (base.0, base.1.opposite(), base.2.opposite()),
                    (base.0.opposite(), base.1, base.2),
                    (base.0.opposite(), base.1, base.2.opposite()),
                    (base.0.opposite(), base.1.opposite(), base.2),
                    (base.0.opposite(), base.1.opposite(), base.2.opposite()),
                ])
            })
            .filter_map(|(x, y, z)| AxisSystem::from_xyz(x, y, z))
    }

    #[test]
    fn axis_system_basis_directions() {
        for asys in all_axis_systems() {
            let [x, y, z] = asys.directions();
            assert_eq!(asys.x_direction(), x);
            assert_eq!(asys.y_direction(), y);
            assert_eq!(asys.z_direction(), z);
        }
    }

    #[test]
    fn axis_system_decompose_then_compose() {
        for asys in all_axis_systems() {
            let [x, y, z] = asys.directions();
            let composed = AxisSystem::from_xyz(x, y, z);
            assert_eq!(composed, Some(asys));
        }
    }

    #[test]
    fn axis_system_completeness() {
        let all = all_axis_systems().collect::<HashSet<_>>();
        assert_eq!(all.len(), 6 * 4 * 2);
    }

    #[test]
    fn axis_system_right_handedness() {
        for asys in all_axis_systems() {
            let [x, y, z] = asys.directions();
            assert_eq!(
                asys.is_right_handed(),
                Direction::right_handed_third_axis(x, y) == Some(z)
            );
        }
    }
}
