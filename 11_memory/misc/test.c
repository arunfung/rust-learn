#include <stdio.h>

struct S1 {
    u_int8_t a;
    u_int16_t b;
    u_int8_t c;
};

struct S2 {
    u_int8_t a;
    u_int8_t c;
    u_int16_t b;
};

void main() {
    printf("size of S1: %d, S2: %d", sizeof(struct S1), sizeof(struct S2));
}

正确答案是:6 和 4。
为什么明明只用了 4 个字节，S1 的大小却是 6 呢?这是因为 CPU 在加载不对齐的内存
时，性能会急剧下降，所以要避免用户定义不对齐的数据结构时，造成的性能影响。 对于这个问题，C 语言会对结构体会做这样的处理:
1. 首先确定每个域的长度和对齐长度，原始类型的对齐长度和类型的长度一致。
2. 每个域的起始位置要和其对齐长度对齐，如果无法对齐，则添加 padding 直至对齐。
3. 结构体的对齐大小和其最大域的对齐大小相同，而结构体的长度则四舍五入到其对齐的 倍数。