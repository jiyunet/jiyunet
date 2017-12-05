# Wrapper functions to make interaction with Rust Jiyunet code simpler and more
# ergonomic/pythonic.

from cffi import FFI
ffi = FFI()
ffi.cdef("""
    uint32_t sum(uint32_t, uint32_t);
""") # TODO Others.
# TODO dlopen the system library

def create_artifact(id, content):
    print('Not Yet Implemented')
    return None

def new_keypair(seed=None):
    print('Not Yet Implemented')
    return None
