// compile with: make test_unveil_image_to_folder

#include <stdio.h>
#include <stdlib.h>

extern void unveil_image_to_folder(
        const char* image_path,
        const char* output_folder);

int main(int argc, const char** argv) {
    if (argc != 3) {
        printf("First argument is the secret <image_path>\nSecond Argument: <output_folder>\n");
        exit(-1);
    }
    unveil_image_to_folder(argv[1], argv[2]);

    return 0;
}
