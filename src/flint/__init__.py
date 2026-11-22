"""
A python/NumPy interface for rounded FLoating point INTervals or flints.
"""
# Copyright (c) 2023, Jef Wagner <jefwagner@gmail.com>
# SPDX-License-Identifier: MIT

from .numpy_flint import flint

__version__ = "0.3.4"

def get_include() -> str:
    """Return the directory with the 'flint.h' header file"""
    import os
    return os.path.dirname(__file__)
