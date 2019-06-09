//! NetCommRPCProxy_Occur.h

extern "C" {
    pub fn NetCommRPCProxy_SetOccurFuncPointer(Occur: Option<extern "C" fn(u32)>);
}
