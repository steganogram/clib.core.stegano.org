LIBS=-lstegano_core_clib -L../target/debug

test_from_c_dbg: test_from_c.c
	gcc -g $< -o $@ ${LIBS}

test_unveil_image_to_folder: test_unveil_image_to_folder.c
	gcc -g $< -o $@ ${LIBS}