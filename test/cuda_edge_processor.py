import ctypes
import numpy as np
import urllib.request
import cv2
import sys
import struct
import io
import time

# 获取 Windows IP
def get_windows_ip():
    import subprocess
    try:
        result = subprocess.run(['ip', 'route', 'show', 'default'], capture_output=True, text=True)
        return result.stdout.split()[2]
    except:
        return "172.26.112.1"

# 加载 CUDA 库
def load_cuda_lib():
    try:
        lib = ctypes.CDLL('./libcuda_edge.so')
        lib.cuda_edge_detect.argtypes = [
            ctypes.POINTER(ctypes.c_ubyte),  # input
            ctypes.POINTER(ctypes.c_ubyte),  # output
            ctypes.c_int,                    # width
            ctypes.c_int,                    # height
            ctypes.c_int                     # channels
        ]
        lib.cuda_edge_detect.restype = ctypes.c_int
        return lib
    except Exception as e:
        print(f"加载 CUDA 库失败: {e}")
        sys.exit(1)

# 从 HTTP 获取图片
def fetch_image(url):
    try:
        resp = urllib.request.urlopen(url, timeout=10)
        data = resp.read()
        nparr = np.frombuffer(data, np.uint8)
        img = cv2.imdecode(nparr, cv2.IMREAD_COLOR)
        return img
    except Exception as e:
        print(f"获取图片失败: {e}")
        return None

# 发送图片到 Windows（异步，不等待响应）
def send_image(url, img):
    try:
        _, buf = cv2.imencode('.jpg', img, [cv2.IMWRITE_JPEG_QUALITY, 80])
        data = buf.tobytes()

        req = urllib.request.Request(url, data=data, method='POST')
        req.add_header('Content-Type', 'image/jpeg')
        # 使用短超时，发送后不等待完整响应
        resp = urllib.request.urlopen(req, timeout=3)
        resp.read()  # 消费响应
        return True
    except Exception as e:
        # 忽略超时错误，只要发送成功就行
        if "timed out" in str(e):
            return True
        print(f"发送失败: {e}")
        return False

def main():
    windows_ip = get_windows_ip()
    print(f"Windows IP: {windows_ip}")

    fetch_url = f"http://{windows_ip}:8080/snapshot"
    send_url = f"http://{windows_ip}:8080/upload"

    lib = load_cuda_lib()
    print("CUDA 库已加载")
    print("开始循环处理...")

    frame_count = 0
    start_time = time.time()

    while True:
        try:
            # 获取图片
            img = fetch_image(fetch_url)
            if img is None:
                time.sleep(0.1)
                continue

            h, w = img.shape[:2]
            channels = 3

            # 第一帧打印尺寸
            if frame_count == 0:
                print(f"图片尺寸: {w}x{h}")

            # CUDA 边缘提取
            input_ptr = img.ctypes.data_as(ctypes.POINTER(ctypes.c_ubyte))
            output = np.zeros((h, w), dtype=np.uint8)
            output_ptr = output.ctypes.data_as(ctypes.POINTER(ctypes.c_ubyte))

            ret = lib.cuda_edge_detect(input_ptr, output_ptr, w, h, channels)
            if ret != 0:
                print("CUDA 处理失败")
                continue

            # 发送结果
            send_image(send_url, output)

            frame_count += 1
            if frame_count % 30 == 0:
                elapsed = time.time() - start_time
                fps = frame_count / elapsed
                print(f"已处理 {frame_count} 帧, FPS: {fps:.1f}")

        except KeyboardInterrupt:
            print("\n已停止")
            break
        except Exception as e:
            print(f"错误: {e}")
            time.sleep(0.1)

    return 0

if __name__ == '__main__':
    main()
