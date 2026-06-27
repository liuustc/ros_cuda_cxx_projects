import cv2
import urllib.request
import numpy as np
import sys

# Windows 主机 IP（WSL2 网关）
WINDOWS_IP = "172.26.112.1"  # 可通过 ip route | grep default 获取
PORT = 8080

def get_windows_ip():
    """自动获取 Windows 主机 IP"""
    import subprocess
    try:
        result = subprocess.run(['ip', 'route', 'show', 'default'], capture_output=True, text=True)
        return result.stdout.split()[2]
    except:
        return WINDOWS_IP

def snapshot():
    """获取单帧"""
    ip = get_windows_ip()
    url = f"http://{ip}:{PORT}/snapshot"
    print(f"连接到 {url}...")

    resp = urllib.request.urlopen(url, timeout=5)
    img_array = np.asarray(bytearray(resp.read()), dtype=np.uint8)
    frame = cv2.imdecode(img_array, cv2.IMREAD_COLOR)

    cv2.imwrite("snapshot.jpg", frame)
    print(f"已保存 snapshot.jpg ({frame.shape[1]}x{frame.shape[0]})")
    return frame

def save_frames(count=10, interval=1):
    """保存多帧图片"""
    import time
    ip = get_windows_ip()
    url = f"http://{ip}:{PORT}/snapshot"
    print(f"连接到 {url}...")
    print(f"保存 {count} 帧，间隔 {interval} 秒")

    for i in range(count):
        try:
            resp = urllib.request.urlopen(url, timeout=5)
            img_array = np.asarray(bytearray(resp.read()), dtype=np.uint8)
            frame = cv2.imdecode(img_array, cv2.IMREAD_COLOR)

            filename = f"frame_{i:04d}.jpg"
            cv2.imwrite(filename, frame)
            print(f"保存 {filename} ({frame.shape[1]}x{frame.shape[0]})")

            if i < count - 1:
                time.sleep(interval)
        except Exception as e:
            print(f"帧 {i} 失败: {e}")

    print(f"完成！保存了 {count} 帧")

if __name__ == '__main__':
    if len(sys.argv) > 1 and sys.argv[1] == 'save':
        count = int(sys.argv[2]) if len(sys.argv) > 2 else 10
        save_frames(count)
    else:
        snapshot()
