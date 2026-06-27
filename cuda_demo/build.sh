#!/bin/bash

export PATH=/usr/local/cuda-13.3/bin:$PATH
export LD_LIBRARY_PATH=/usr/local/cuda-13.3/lib64:${LD_LIBRARY_PATH:-}

# 下载 stb_image 头文件（如果没有）
if [ ! -f stb_image.h ]; then
    echo "下载 stb_image.h..."
    wget -q https://raw.githubusercontent.com/nothings/stb/master/stb_image.h
    wget -q https://raw.githubusercontent.com/nothings/stb/master/stb_image_write.h
fi

# 编译 CUDA 库
echo "编译 CUDA 库..."
nvcc -shared -Xcompiler -fPIC -o libcuda_edge.so cuda_edge.cu -lcudart

# 编译 C++ 处理程序（可选）
# g++ -o edge_processor edge_processor.cpp -I. -L. -lcuda_edge -lcurl -lm

echo "编译完成!"
echo "运行: python3 cuda_edge_processor.py"
