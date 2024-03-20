use core::{marker, mem, ptr, slice};

pub struct Arena<'m> {
    ptr: *mut u8,
    len: usize,
    _marker: marker::PhantomData<&'m mut [u8]>,
}

impl<'m> Arena<'m> {
    pub fn new(mem: &'m mut [u8]) -> Self {
        Self {
            ptr: mem.as_mut_ptr(),
            len: mem.len(),
            _marker: marker::PhantomData,
        }
    }

    pub fn capacity(&self) -> usize {
        self.len
    }

    pub fn bytes(&mut self, n: usize) -> &'m mut [u8] {
        let ptr = self.alloc_raw::<u8>(n);
        unsafe { slice::from_raw_parts_mut(ptr, n) }
    }

    pub fn alloc<T: Sized>(&mut self, t: T) -> &'m mut T {
        self.alloc_with(|| t)
    }

    pub fn alloc_default<T: Sized + Default>(&mut self) -> &'m mut T {
        self.alloc_with(T::default)
    }

    pub fn alloc_with<T: Sized>(&mut self, f: impl FnOnce() -> T) -> &'m mut T {
        let ptr = self.alloc_raw::<T>(1);
        unsafe {
            ptr.write(f());
            ptr.as_mut().unwrap()
        }
    }

    pub fn alloc_slice<T: Sized + Clone>(&mut self, n: usize, t: T) -> &'m mut [T] {
        self.alloc_slice_with(n, move || t.clone())
    }

    pub fn alloc_slice_default<T: Sized + Default>(&mut self, n: usize) -> &'m mut [T] {
        self.alloc_slice_with(n, T::default)
    }

    pub fn alloc_slice_with<T: Sized>(
        &mut self,
        n: usize,
        mut f: impl FnMut() -> T,
    ) -> &'m mut [T] {
        let ptr = self.alloc_raw::<T>(n);
        unsafe {
            if mem::size_of::<T>() > 0 {
                for i in 0..n {
                    ptr.add(i).write(f());
                }
            }

            slice::from_raw_parts_mut(ptr, n)
        }
    }

    fn alloc_raw<T: Sized>(&mut self, n: usize) -> *mut T {
        let size = mem::size_of::<T>();
        let align = mem::align_of::<T>();
        let offset = self.ptr.align_offset(align);

        if n * size == 0 {
            return unsafe { ptr::NonNull::dangling().as_mut() };
        }

        let num_bytes = n * size + offset;

        self.len = self
            .len
            .checked_sub(num_bytes)
            .expect("alloc exceeded arena capacity");

        unsafe {
            let ptr = self.ptr.add(offset).cast::<T>();
            self.ptr = self.ptr.add(num_bytes);

            ptr
        }
    }
}
