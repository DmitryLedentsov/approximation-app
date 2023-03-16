#!derive[(Debug)]
pub enum AppError{
    IOError,

    InvalidFormat,
    UnableApproximate,
    OnePoint
}