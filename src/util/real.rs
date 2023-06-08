use geng::prelude::{mat3, r32, vec2, Float, R32, R64};

pub trait RealConversions {
    fn as_r32(&self) -> R32;
}

impl<T: Float> RealConversions for T {
    fn as_r32(&self) -> R32 {
        r32(self.as_f32())
    }
}

pub trait Vec2RealConversions {
    fn as_f32(&self) -> vec2<f32>;
    fn as_r32(&self) -> vec2<R32>;
}

pub trait Mat3RealConversions {
    fn as_f32(&self) -> mat3<f32>;
    fn as_r32(&self) -> mat3<R32>;
}

// impl<T: Float> Vec2RealConversions for vec2<T> {
//     fn as_f32(&self) -> vec2<f32> {
//         self.map(|x| x.as_f32())
//     }
//     fn as_r32(&self) -> vec2<R32> {
//         self.map(|x| r32(x.as_f32()))
//     }
// }

macro_rules! impl_vec2_lossy {
    ($t:ident) => {
        impl Vec2RealConversions for vec2<$t> {
            fn as_f32(&self) -> vec2<f32> {
                self.map(|x| x as f32)
            }
            fn as_r32(&self) -> vec2<R32> {
                self.map(|x| r32(x as f32))
            }
        }
    };
}

macro_rules! impl_vec2_float {
    ($t:ident) => {
        impl Vec2RealConversions for vec2<$t> {
            fn as_f32(&self) -> vec2<f32> {
                self.map(|x| x.as_f32())
            }
            fn as_r32(&self) -> vec2<R32> {
                self.map(|x| r32(x.as_f32()))
            }
        }
    };
}

impl_vec2_lossy!(usize);
impl_vec2_lossy!(u8);
impl_vec2_lossy!(u16);
impl_vec2_lossy!(u32);
impl_vec2_lossy!(u64);
impl_vec2_lossy!(u128);
impl_vec2_lossy!(isize);
impl_vec2_lossy!(i8);
impl_vec2_lossy!(i16);
impl_vec2_lossy!(i32);
impl_vec2_lossy!(i64);
impl_vec2_lossy!(i128);

impl_vec2_float!(f32);
impl_vec2_float!(f64);
impl_vec2_float!(R32);
impl_vec2_float!(R64);

impl<T: Float> Mat3RealConversions for mat3<T> {
    fn as_f32(&self) -> mat3<f32> {
        self.map(|x| x.as_f32())
    }
    fn as_r32(&self) -> mat3<R32> {
        self.map(|x| r32(x.as_f32()))
    }
}
