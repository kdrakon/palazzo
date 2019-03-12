#define FUSE_USE_VERSION 31

#include "../libfuse/include/fuse.h"

int rust_fuse_main(int argc, char *argv[], const struct fuse_operations *op) {
	return fuse_main_real(argc, argv, op, sizeof(*(op)), NULL);
}


// int argc, char *argv[], const struct fuse_operations *op,
   		   // size_t op_size, void *private_data