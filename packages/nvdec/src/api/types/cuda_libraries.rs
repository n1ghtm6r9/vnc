use libloading::Library;

pub(crate) struct CudaLibraries {
    pub(crate) cuda: Library,
    pub(crate) cuvid: Library,
}
