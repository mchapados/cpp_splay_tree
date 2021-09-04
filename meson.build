project('splay_tree', 'cpp', version: '1.0')
# Build malacryptor executable
src = ['src/main.cpp']
flags = ['-O3', '-march=haswell', '-Wall', '-Werror', '-Wextra']
executable('splay_tree',
           src,
           cpp_args : flags,
		   link_args: ['-lpthread'])
