from .my_rust_lib import *

__doc__ = my_rust_lib.__doc__
if hasattr(my_rust_lib, "__all__"):
    __all__ = my_rust_lib.__all__