#include <chrono>
#include <memory>
#include "rclcpp/rclcpp.hpp"
#include "std_msgs/msg/string.hpp"

using namespace std::chrono_literals;

class Talker : public rclcpp::Node
{
public:
  Talker() : Node("talker"), count_(0)
  {
    rclcpp::QoS qos(rclcpp::KeepLast(10));
    qos.reliable();
    publisher_ = this->create_publisher<std_msgs::msg::String>("chatter", qos);
    subscription_ = this->create_subscription<std_msgs::msg::String>(
      "reply", qos,
      [this](const std_msgs::msg::String::SharedPtr msg) {
        RCLCPP_INFO(this->get_logger(), "[Talker] 收到回复: '%s'", msg->data.c_str());
      });

    timer_ = this->create_wall_timer(1s, [this]() {
      auto message = std_msgs::msg::String();
      message.data = "你好! 消息 #" + std::to_string(count_++);
      RCLCPP_INFO(this->get_logger(), "[Talker] 发送: '%s'", message.data.c_str());
      publisher_->publish(message);
    });
  }

private:
  rclcpp::Publisher<std_msgs::msg::String>::SharedPtr publisher_;
  rclcpp::Subscription<std_msgs::msg::String>::SharedPtr subscription_;
  rclcpp::TimerBase::SharedPtr timer_;
  size_t count_;
};

int main(int argc, char * argv[])
{
  rclcpp::init(argc, argv);
  rclcpp::spin(std::make_shared<Talker>());
  rclcpp::shutdown();
  return 0;
}
