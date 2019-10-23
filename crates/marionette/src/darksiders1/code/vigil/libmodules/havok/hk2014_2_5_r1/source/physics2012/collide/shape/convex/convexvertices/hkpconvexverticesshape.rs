use crate::darksiders1::hkpConvexShape;
use darksiders1_sys::target;
use na::Vector4;
use std::{
    arch::x86::{__m128, _mm_setzero_ps},
    convert::TryInto,
};

struct_wrapper!(
    /// A convex geometric shape, specified by a set of vertices in the shape's
    /// local space. An optional set of corresponding planes may be provided
    /// - if present, they will improve ray cast performance. Optional
    /// connectivity information may also be provided - this is required for
    /// certain algorithms, but not for most use cases.
    hkpConvexVerticesShape,
    target::hkpConvexVerticesShape,
);
struct_wrapper_super!(hkpConvexVerticesShape, hkpConvexShape);

impl hkpConvexVerticesShape {
    /// The original vertices passed during construction are stored in a packed
    /// form within the shape. This retrieves the original vertices into the
    /// provided array.
    ///
    /// A `Vec` of `__m128` is needed, to guarantee the buffer is 16-byte
    /// aligned. For convenience, this returns a slice transmuted to `Vector4`,
    /// which is the more natural type for the data.
    pub fn get_original_vertices(&self, vertices: &mut Vec<__m128>) -> &mut [Vector4<f32>] {
        // Hack: pre-allocate a buffer of the right size (so it won't need to be
        // resized), then pretend it's an hkArray, so I don't have to deal with
        // Havok memory management.
        let size = self.inner.m_rotatedVertices.m_size * 4;
        vertices.resize_with(size.try_into().unwrap(), || unsafe { _mm_setzero_ps() });
        let mut array = target::hkArray_hkVector4f_hkContainerHeapAllocator_ {
            m_data: vertices.as_mut_ptr().cast(),
            m_size: size,
            m_capacityAndFlags: size,
        };
        unsafe { target::hkpConvexVerticesShape__getOriginalVertices(self.as_ptr(), &mut array) }

        // The function will shrink `array` down to the needed size, so we should pass
        // the info along.
        vertices.resize_with(array.m_size.try_into().unwrap(), || unsafe {
            _mm_setzero_ps()
        });

        unsafe { &mut *(vertices.as_mut_slice() as *mut [__m128] as *mut [Vector4<f32>]) }
    }
}
