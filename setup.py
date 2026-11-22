#!/usr/bin/env python
# -*- coding: utf-8 -*-
## @file setup.py Python/Numpy interface for flints
"""\
This package creates a rounded floating point interval (flint) type in python
and extends the type to numpy allowing for the creation and manipulation of
arrays of flints. All the arithmetic operations as well as a few elementary
mathematical functions are enabled for them. The core code is written in c...
because 'why not?'
"""
# Copyright (c) 2023, Jef Wagner <jefwagner@gmail.com>
# SPDX-License-Identifier: MIT

from setuptools import setup, Extension
import numpy as np

setup_args = dict(
    ext_modules = [
        Extension(
            name='flint.numpy_flint',
            sources=['src/flint/numpy_flint.c'],
            depends=[
                'src/flint/flint.h',
                'src/flint/numpy_flint.h',
                'src/flint/numpy_flint.c',
            ],
            include_dirs=[np.get_include()],
        )
    ]
)

setup(**setup_args)
