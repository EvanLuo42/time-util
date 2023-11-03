use std::any::TypeId;
use std::ops::{Add, Div, Mul, Sub};

pub struct Time<Unit, ValueType>
where
    Unit: TimeUnit,
    ValueType: Add + Sub + Div + Mul
{
    pub unit: Unit,
    pub value: ValueType
}

impl<Unit, ValueType> Time<Unit, ValueType>
where
    Unit: TimeUnit,
    ValueType: Add<Output = ValueType> + Sub<Output = ValueType> + Div<Output = ValueType> + Mul<Output = ValueType>
{
    pub fn new(value: ValueType, unit: Unit) -> Self {
        Time {
            unit,
            value
        }
    }
}

impl<Unit, ValueType> Add<ValueType> for Time<Unit, ValueType>
where
    Unit: TimeUnit,
    ValueType: Add + Sub + Div + Mul
{
    type Output = Time<Unit, ValueType>;

    fn add(self, rhs: ValueType) -> Self::Output {
        Time::new(self.value + rhs, self.unit)
    }
}

impl<Unit, ValueType> Sub<ValueType> for Time<Unit, ValueType>
where
    Unit: TimeUnit,
    ValueType: Add + Sub + Div + Mul
{
    type Output = Time<Unit, ValueType>;

    fn sub(self, rhs: ValueType) -> Self::Output {
        Time::new(self.value - rhs, self.unit)
    }
}

impl<Unit, ValueType> Div<ValueType> for Time<Unit, ValueType>
    where
        Unit: TimeUnit,
        ValueType: Add + Sub + Div + Mul
{
    type Output = Time<Unit, ValueType>;

    fn div(self, rhs: ValueType) -> Self::Output {
        Time::new(self.value - rhs, self.unit)
    }
}

impl<Unit, ValueType> Mul<ValueType> for Time<Unit, ValueType>
    where
        Unit: TimeUnit,
        ValueType: Add + Sub + Div + Mul
{
    type Output = Time<Unit, ValueType>;

    fn mul(self, rhs: ValueType) -> Self::Output {
        Time::new(self.value - rhs, self.unit)
    }
}

pub trait TimeUnit {
    fn convert<FromUnit, ValueType, ToUnit>(&self, from: ValueType) -> ValueType
    where
        FromUnit: TimeUnit,
        ToUnit: TimeUnit,
        ValueType: Add + Sub + Div + Mul;
}

#[derive(PartialEq, Debug)]
pub struct Second;

#[derive(PartialEq, Debug)]
pub struct Minute;

impl TimeUnit for Second {
    fn convert<FromUnit, ValueType, ToUnit>(&self, from: ValueType) -> ValueType
    where
        ToUnit: TimeUnit,
        FromUnit: TimeUnit,
        ValueType: Add + Sub + Div + Mul
    {
        match TypeId::of::<ToUnit>() {
            TypeId::of::<Minute>() => {
                from / 60
            },
            TypeId::of::<Second>() => {
                from
            }
        }
    }
}

impl TimeUnit for Minute {
    fn convert<FromUnit, ValueType, ToUnit>(&self, from: ValueType) -> ValueType
        where
            ToUnit: TimeUnit,
            FromUnit: TimeUnit,
            ValueType: Add + Sub + Div + Mul
    {
        match TypeId::of::<ToUnit>() {
            TypeId::of::<Second>() => {
                from * 60
            },
            TypeId::of::<Minute>() => {
                from
            }
        }
    }
}

macro_rules! time {
    ($value:expr => $unit:expr) => {
        Time::new($value, $unit)
    };
}