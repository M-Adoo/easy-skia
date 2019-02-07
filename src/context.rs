pub use bindings::gr_backend_t as GrBackend;
pub use bindings::gr_gl_framebufferinfo_t as FrameBufferInfo;
use bindings::*;
use std::ffi::CString;
use ColorType;

pub struct GrContext {
  pub(crate) raw_pointer: *mut gr_context_t,
}

impl GrContext {
  pub fn make_gl(glinterface: Option<GrGlInterface>) -> GrContext {
    let ptr = if let Some(glinterface) = glinterface {
      glinterface.raw_pointer
    } else {
      std::ptr::null()
    };
    GrContext {
      raw_pointer: unsafe { gr_context_make_gl(ptr) },
    }
  }

  pub fn abandon_context(&self) {
    unsafe { gr_context_abandon_context(self.raw_pointer) };
  }

  pub fn release_resources_and_abandon_context(&self) {
    unsafe { gr_context_release_resources_and_abandon_context(self.raw_pointer) };
  }

  pub fn get_resource_cache_limits(&self) -> (i32, usize) {
    let mut max_resources: i32 = 0;
    let mut max_resources_bytes: usize = 0;
    unsafe {
      gr_context_get_resource_cache_limits(
        self.raw_pointer,
        &mut max_resources,
        &mut max_resources_bytes,
      )
    };
    (max_resources, max_resources_bytes)
  }

  pub fn set_resource_cache_limit(&self, max_resources: i32, max_resources_bytes: usize) {
    unsafe {
      gr_context_set_resource_cache_limits(self.raw_pointer, max_resources, max_resources_bytes)
    };
  }

  pub fn get_resource_cache_usage(&self) -> (i32, usize) {
    let mut max_resources: i32 = 0;
    let mut max_resources_bytes: usize = 0;
    unsafe {
      gr_context_get_resource_cache_usage(
        self.raw_pointer,
        &mut max_resources,
        &mut max_resources_bytes,
      )
    };
    (max_resources, max_resources_bytes)
  }

  pub fn get_max_surface_sample_count_for_color_type(
    &self,
    colorType: ColorType,
  ) -> i32 {
    unsafe { gr_context_get_max_surface_sample_count_for_color_type(self.raw_pointer, colorType) }
  }

  pub fn flush(&self) {
    unsafe { gr_context_flush(self.raw_pointer) }
  }

  pub fn get_backend(&self) -> GrBackend {
    unsafe { gr_context_get_backend(self.raw_pointer) }
  }
}

impl Drop for GrContext {
  fn drop(&mut self) {
    unsafe { gr_context_unref(self.raw_pointer) };
  }
}

/// GrInterface wrap
pub struct GrGlInterface {
  pub(crate) raw_pointer: *const gr_glinterface_t,
}

impl GrGlInterface {
  pub fn create_native_interface() -> GrGlInterface {
    GrGlInterface {
      raw_pointer: unsafe { gr_glinterface_create_native_interface() },
    }
  }

  pub fn is_valid(&self) -> bool {
    unsafe { gr_glinterface_validate(self.raw_pointer) }
  }

  pub fn has_extension(&self, extension: &str) -> bool {
    let cstr = CString::new(extension).unwrap();
    unsafe { gr_glinterface_has_extension(self.raw_pointer, cstr.as_ptr()) }
  }
}

impl Drop for GrGlInterface {
  fn drop(&mut self) {
    unsafe { gr_glinterface_unref(self.raw_pointer) };
  }
}

/// Backend Texture
pub struct GrBackendTexture {
  pub(crate) raw_pointer: *mut gr_backendtexture_t,
}

// SK_C_API gr_backendtexture_t* gr_backendtexture_new_gl(int width, int height, bool mipmapped, const gr_gl_textureinfo_t* glInfo);
// SK_C_API void gr_backendtexture_delete(gr_backendtexture_t* texture);
//
// SK_C_API bool gr_backendtexture_is_valid(const gr_backendtexture_t* texture);
// SK_C_API int gr_backendtexture_get_width(const gr_backendtexture_t* texture);
// SK_C_API int gr_backendtexture_get_height(const gr_backendtexture_t* texture);
// SK_C_API bool gr_backendtexture_has_mipmaps(const gr_backendtexture_t* texture);
// SK_C_API gr_backend_t gr_backendtexture_get_backend(const gr_backendtexture_t* texture);
// SK_C_API bool gr_backendtexture_get_gl_textureinfo(const gr_backendtexture_t* texture, gr_gl_textureinfo_t* glInfo);

/// Backend Render Target
pub struct GrBackendRenderTarget {
  pub(crate) raw_pointer: *mut gr_backendrendertarget_t,
}

impl GrBackendRenderTarget {
  pub fn new_gl(
    width: i32,
    height: i32,
    samples: i32,
    stencils: i32,
    gl_info: &FrameBufferInfo,
  ) -> GrBackendRenderTarget {
    GrBackendRenderTarget {
      raw_pointer: unsafe {
        gr_backendrendertarget_new_gl(width, height, samples, stencils, gl_info)
      },
    }
  }

  pub fn is_valid(&self) -> bool {
    unsafe { gr_backendrendertarget_is_valid(self.raw_pointer) }
  }

  pub fn get_width(&self) -> i32 {
    unsafe { gr_backendrendertarget_get_width(self.raw_pointer) }
  }

  pub fn get_height(&self) -> i32 {
    unsafe { gr_backendrendertarget_get_height(self.raw_pointer) }
  }

  pub fn get_samples(&self) -> i32 {
    unsafe { gr_backendrendertarget_get_samples(self.raw_pointer) }
  }

  pub fn get_stenciles(&self) -> i32 {
    unsafe { gr_backendrendertarget_get_stencils(self.raw_pointer) }
  }

  pub fn get_backend(&self) -> GrBackend {
    unsafe { gr_backendrendertarget_get_backend(self.raw_pointer) }
  }

  pub fn get_gl_framebufferinfo(&self) -> Option<FrameBufferInfo> {
    let mut info: FrameBufferInfo = FrameBufferInfo {
      fFBOID: 0,
      fFormat: 0,
    };
    unsafe {
      if gr_backendrendertarget_get_gl_framebufferinfo(self.raw_pointer, &mut info) {
        Some(info)
      } else {
        None
      }
    }
  }
}

impl Drop for GrBackendRenderTarget {
  fn drop(&mut self) {
    unsafe {
      gr_backendrendertarget_delete(self.raw_pointer);
    }
  }
}
