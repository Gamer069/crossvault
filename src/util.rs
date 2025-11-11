#[derive(Clone, Copy, Debug)]
pub enum PlaceType {
    UpperLeftDiagonal,
    UpperRightDiagonal,

    LowerRightDiagonal,
    LowerLeftDiagonal,

    RightStraight,
    UpStraight,
    LeftStraight,
    DownStraight
}
