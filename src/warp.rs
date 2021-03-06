use libc::{c_int, c_char, c_double};
use std::ptr::null;
use super::raster::RasterDataset;

#[link(name="gdal")]
extern {
    fn GDALReprojectImage(
        hSrcDS: *const (),
        pszSrcWKT: *const c_char,
        hDstDS: *const (),
        pszDstWKT: *const c_char,
        eResampleAlg: c_int,
        dfWarpMemoryLimit: c_double,
        dfMaxError: c_double,
        pfnProgress: *const (),
        pProgressArg: *const (),
        psOptions: *const ()
    ) -> c_int;
}

static GRA_NEARESTNEIGHBOUR:  c_int = 0;
static GRA_BILINEAR:          c_int = 1;
static GRA_CUBIC:             c_int = 2;
static GRA_CUBICSPLINE:       c_int = 3;
static GRA_LANCZOS:           c_int = 4;
static GRA_AVERAGE:           c_int = 5;
static GRA_MODE:              c_int = 6;

static REPROJECT_MEMORY_LIMIT: c_double = 0.0;

pub fn reproject(src: &RasterDataset, dst: &RasterDataset) {
    let rv = unsafe {
        GDALReprojectImage(
                src._c_ptr(),
                null(),
                dst._c_ptr(),
                null(),
                GRA_BILINEAR,
                REPROJECT_MEMORY_LIMIT,
                0.0 as c_double,
                null(),
                null(),
                null()
            )
    } as int;
    assert!(rv == 0);
}
