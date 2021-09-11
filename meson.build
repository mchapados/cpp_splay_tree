project('splay_tree', 'cpp', version: '1.0')
# Build malacryptor executable
src = ['src/main.cpp']
flags = ['-O3', '-Wall', '-Werror', '-Wextra']
executable('splay_tree',
           src,
           cpp_args : flags,
		   dependencies: dependency('threads'))
