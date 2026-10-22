use libloading::Library;

use crate::NvdecError;

pub(crate) fn symbol<T: Copy>(library: &Library, name: &str) -> Result<T, NvdecError> {
    let found = unsafe { library.get::<T>(name) }.map_err(NvdecError::Load)?;
    Ok(*found)
}
