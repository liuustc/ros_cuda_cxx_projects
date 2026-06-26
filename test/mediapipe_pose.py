import cv2
import numpy as np
import urllib.request
import mediapipe as mp
from mediapipe.tasks import python
from mediapipe.tasks.python import vision
from mediapipe import Image
import time
import sys

# 获取 Windows IP
def get_windows_ip():
    import subprocess
    try:
        result = subprocess.run(['ip', 'route', 'show', 'default'], capture_output=True, text=True)
        return result.stdout.split()[2]
    except:
        return "172.26.112.1"

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

# 发送图片到 Windows
def send_image(url, img):
    try:
        _, buf = cv2.imencode('.jpg', img, [cv2.IMWRITE_JPEG_QUALITY, 85])
        data = buf.tobytes()

        req = urllib.request.Request(url, data=data, method='POST')
        req.add_header('Content-Type', 'image/jpeg')
        resp = urllib.request.urlopen(req, timeout=3)
        resp.read()
        return True
    except Exception as e:
        if "timed out" in str(e):
            return True
        print(f"发送失败: {e}")
        return False

def draw_landmarks_on_image(image, detection_result):
    """在图片上绘制骨架"""
    pose_landmarks_list = detection_result.pose_landmarks

    # MediaPipe Pose 33 个关键点的连接关系
    POSE_CONNECTIONS = [
        (0, 1), (1, 2), (2, 3), (3, 7),  # 头部
        (0, 4), (4, 5), (5, 6), (6, 8),  # 头部
        (9, 10),  # 嘴巴
        (11, 12),  # 肩膀
        (11, 13), (13, 15),  # 左臂
        (12, 14), (14, 16),  # 右臂
        (15, 17), (15, 19), (15, 21),  # 左手
        (16, 18), (16, 20), (16, 22),  # 右手
        (11, 23), (12, 24),  # 躯干
        (23, 24),  # 臀部
        (23, 25), (25, 27),  # 左腿
        (24, 26), (26, 28),  # 右腿
        (27, 29), (27, 31),  # 左脚
        (28, 30), (28, 32),  # 右脚
    ]

    for pose_landmarks in pose_landmarks_list:
        h, w, _ = image.shape

        # 绘制连接线
        for connection in POSE_CONNECTIONS:
            start_idx = connection[0]
            end_idx = connection[1]

            start_landmark = pose_landmarks[start_idx]
            end_landmark = pose_landmarks[end_idx]

            start_x = int(start_landmark.x * w)
            start_y = int(start_landmark.y * h)
            end_x = int(end_landmark.x * w)
            end_y = int(end_landmark.y * h)

            cv2.line(image, (start_x, start_y), (end_x, end_y), (0, 255, 0), 2)

        # 绘制关键点
        for idx, landmark in enumerate(pose_landmarks):
            x = int(landmark.x * w)
            y = int(landmark.y * h)
            cv2.circle(image, (x, y), 5, (0, 0, 255), -1)

    return image

def main():
    windows_ip = get_windows_ip()
    print(f"Windows IP: {windows_ip}")

    fetch_url = f"http://{windows_ip}:8080/snapshot"
    send_url = f"http://{windows_ip}:8080/upload"

    print("MediaPipe 人体姿势识别")
    print("=" * 50)
    print(f"获取图片: {fetch_url}")
    print(f"发送结果: {send_url}")
    print("=" * 50)

    # 下载模型文件（如果不存在）
    model_path = "pose_landmarker_heavy.task"
    import os
    if not os.path.exists(model_path):
        print(f"下载模型文件: {model_path}")
        import urllib.request
        # 使用 heavy 版本，精度更高
        url = "https://storage.googleapis.com/mediapipe-models/pose_landmarker/pose_landmarker_heavy/float16/latest/pose_landmarker_heavy.task"
        urllib.request.urlretrieve(url, model_path)
        print("模型下载完成")

    # 初始化 PoseLandmarker
    base_options = python.BaseOptions(model_asset_path=model_path)
    options = vision.PoseLandmarkerOptions(
        base_options=base_options,
        running_mode=vision.RunningMode.VIDEO,
        min_pose_detection_confidence=0.5,
        min_pose_presence_confidence=0.5,
        min_tracking_confidence=0.5,
        num_poses=1
    )

    landmarker = vision.PoseLandmarker.create_from_options(options)

    print("MediaPipe PoseLandmarker 已初始化")
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

            # 第一帧打印尺寸
            if frame_count == 0:
                print(f"图片尺寸: {w}x{h}")

            # BGR 转 RGB
            img_rgb = cv2.cvtColor(img, cv2.COLOR_BGR2RGB)

            # 创建 MediaPipe Image
            mp_image = Image(image_format=mp.ImageFormat.SRGB, data=img_rgb)

            # MediaPipe 处理
            timestamp = int(time.time() * 1000)
            detection_result = landmarker.detect_for_video(mp_image, timestamp)

            # 绘制骨架
            img_output = img.copy()
            if detection_result.pose_landmarks:
                img_output = draw_landmarks_on_image(img_output, detection_result)

                # 打印关键点信息（可选）
                if frame_count == 0:
                    landmarks = detection_result.pose_landmarks[0]
                    print(f"检测到 {len(landmarks)} 个关键点")

            # 发送结果
            send_image(send_url, img_output)

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

    landmarker.close()
    return 0

if __name__ == '__main__':
    main()
