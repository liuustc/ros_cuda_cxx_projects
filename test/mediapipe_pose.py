import cv2
import numpy as np
import urllib.request
import mediapipe as mp
import time
import sys

# MediaPipe 姿势检测
mp_pose = mp.solutions.pose
mp_drawing = mp.solutions.drawing_utils
mp_drawing_styles = mp.solutions.drawing_styles

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

    # 初始化 MediaPipe Pose
    with mp_pose.Pose(
        model_complexity=1,           # 0, 1, 2 (越小越快)
        smooth_landmarks=True,
        min_detection_confidence=0.5,
        min_tracking_confidence=0.5
    ) as pose:

        print("MediaPipe Pose 已初始化")
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

                # MediaPipe 处理
                results = pose.process(img_rgb)

                # 绘制骨架
                img_output = img.copy()
                if results.pose_landmarks:
                    mp_drawing.draw_landmarks(
                        img_output,
                        results.pose_landmarks,
                        mp_pose.POSE_CONNECTIONS,
                        landmark_drawing_spec=mp_drawing_styles.get_default_pose_landmarks_style()
                    )

                    # 打印关键点信息（可选）
                    if frame_count == 0:
                        landmarks = results.pose_landmarks.landmark
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

    return 0

if __name__ == '__main__':
    main()
